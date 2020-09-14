use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///an alias for result that uses dynamic errors
pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// a macro for returning custom errors as dynamic errors
/// 
/// #Example
/// ```ignore
/// if x > 3 {dynerr!(MyError::CustomError("x less than 3"))}
/// ```
#[macro_export]
macro_rules! dynerr {
    ($e:expr) => {return Err(Box::new($e))};
}

/// performs a dynamic match operation on multiple error types\
/// takes the DynError, match arms, and default arm\
/// types must be specified beforehand with the "type" keyword
/// 
/// #Example
/// ```ignore
/// ...
/// let i = match example(9) { //returns dyn error
///     Ok(i) => i,
///     Err(e) => {
///         dynmatch!(e,                                                    //the error to match
///             type ExampleError1 {                                        //error type group
///                 arm ExampleError1::ThisError(2) => panic!("it was 2!"), //arm [pattern] => {code}
///                 _ => panic!("{}",e)                                     //_ => {code}
///             },
///             type ExampleError2 {                                        //another error type
///                 arm ExampleError2::ThatError(8) => panic!("it was 8!"), //more arms
///                 arm ExampleError2::ThatError(9) => 9,
///                 _ => panic!("{}",e)                                     //more wildcard matches
///             }, 
///             _ => panic!("{}",e)                                         //final wildcard if type not found
///         )
///     }
/// };
/// ...
/// ```
#[macro_export]
macro_rules! dynmatch {
    ($e:expr, $(type $ty:ty {$(arm $pat:pat => $result:expr),*, _ => $any:expr}),*, _ => $end:expr) => (
        $(
            if let Some(e) = $e.downcast_ref::<$ty>() {
                match e {
                    $(
                        $pat => {$result}
                    )*
                    _ => $any
                }
            } else
        )*
        {$end}
    );
}

/// logs [event] to [log_file]
pub fn log<T: fmt::Display>(event: T, log_file: &str) -> T{
    fn log_event<T: fmt::Display>(event: T, log_file: &str) -> T {
        let mut file = File::open(log_file)
            .unwrap_or_else(|e| panic!("Error opening log during crash: {}\n crash: {}",e,event));
    
        let mut log = String::new();
        file.read_to_string(&mut log)
            .unwrap_or_else(|e| panic!("Error reading log during crash: {}\n crash: {}",e,event));
        if log != "" { log = format!("{}\n{}", log, event.to_string())}
        else {log = event.to_string()}
    
        let mut file = File::create(log_file)
            .unwrap_or_else(|e| panic!("Error creating log during crash: {}\n crash: {}",e,event));
        file.write_all(log.as_bytes())
            .unwrap_or_else(|e| panic!("Error writing log during crash: {}\n crash: {}",e,event));
    
        event
    }

    if !Path::new(log_file).exists() {
        File::create(log_file)
            .unwrap_or_else(|e| panic!("Error creating log file during crash: {}\n crash: {}",e,event));
        log_event("Log file created", log_file);
    }
    log_event(event, log_file)
}

/// logs message to file\
/// if no file supplied then defaults to "event.log"\
/// 
/// #Example
/// ```ignore
/// example(9).unwrap_or_else(|e|log!(e), "error.log")
/// log!("i just logged an error to error.log!")
/// ```
#[macro_export]
macro_rules! log {
    ($event:expr) => {
        $crate::log($event, "event.log")
    };
    ($event:expr, $log:expr) => {
        $crate::log($event, $log)
    };
}

/// logs error to file then panic!\
/// if no file supplied then defaults to "event.log"\
/// 
/// #Example
/// ```ignore
/// example(9).unwrap_or_else(|e|logged_panic!(e));
/// example(9).unwrap_or_else(|e|logged_panic!(e), "mylog.log");
/// ```
#[macro_export]
macro_rules! logged_panic {
    ($e: expr) => {
        panic!("{}",log!($e));
    };

    ($e: expr, $file:expr) => {
        panic!("{}",log!($e, $file));
    }
}


/// Does .unwrap_or_else(|e| logged_panic!(e)) on result\
/// if no file supplied then defaults to "event.log"\
/// 
/// #Example
/// ```ignore
/// let _i = check!(example(9);
/// check!(example(9, "error.log");
/// ```
#[macro_export]
macro_rules! check {
    ($x:expr) => {
        $x.unwrap_or_else(|e| logged_panic!(e))
    };
    ($x:expr, $log:expr) => {
        $x.unwrap_or_else(|e| logged_panic!(e, $log))
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    //THIS SECTION IS CREATING THE FIRST CUSTOM ERROR
    use std::{fmt, error};
    ///a custom error type
    #[derive(Debug)]
    enum ExampleError1 {
        ThisError(u32),
    }
    //impl display formatting for error
    impl fmt::Display for ExampleError1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ExampleError1::ThisError(i)      => write!(f, "ExampleError1::ThisError: {}",i),
            }
        }
    }
    //impl error conversion for error
    impl error::Error for ExampleError1 {}

    //THIS SECTION IS CREATING THE SECOND CUSTOM ERROR
    #[derive(Debug)]
    enum ExampleError2 {
        ThatError(u32),
    }
    impl fmt::Display for ExampleError2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ExampleError2::ThatError(i)      => write!(f, "ExampleError2::ThatError: {}",i),
            }
        }
    }
    impl error::Error for ExampleError2 {}

    //THIS SECTION IS USING IT
    #[test]
    pub fn test() -> DynResult<()> {    
        //shows error handling capabilities using DynError
        fn example(x: u32) -> DynResult<u32> {
            match x {
                1      => Ok(x),                                //Ok
                2..=4  => dynerr!(ExampleError1::ThisError(x)), //custom error
                5..=10 => dynerr!(ExampleError2::ThatError(x)), //different custom error
                _      => {     
                    std::env::current_dir()?;                   //an error not even defined by you!
                    Ok(x)
                }
            }
        }

        log!("this is a test", "test.log");
        let _i = match example(8) {
            Ok(i) => i,
            Err(e) => {
                dynmatch!(e,                                                            //the dynamic error to be matched
                    type ExampleError1 {                                                //an error group
                        arm ExampleError1::ThisError(2) => logged_panic!("it was 2!"),  //arm [pattern] => {code}
                        _ => panic!("{}",e)                                             //_ => {code}
                    },
                    type ExampleError2 {
                        arm ExampleError2::ThatError(8) => logged_panic!("it was 8!", "test.log"),
                        arm ExampleError2::ThatError(9) => 9,
                        _ => panic!("{}",e)
                    }, 
                    _ => panic!("{}",e)                                                 //what to do if error group isn't found
                )
            }
        };
        log!("do logged_panic! if error");
        let _i = check!(example(9));
        Ok(())
    }
}
