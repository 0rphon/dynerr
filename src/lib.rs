//! Some macros to help with dynamic error handling/logging.
//! 
//! The goal of this crate is to unify all error types without compromising type safety.
//! 
//! The main features of this crate are the `dynerr!` and `dynmatch!` macros. when used alongside the return type `DynResult<T>`, they allows you to return multiple error types from a function then easily match for them during your error handling. Using dynerr, theres no need to ever wrap errors.



use std::fmt;
use std::path::Path;
use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;

///type alias for an error returned by `dynerr!` and `DynResult<T>`
pub type DynError = Box<dyn std::error::Error>;

/// An alias for result that uses DynError
/// 
///# Example
/// ```rust
///# use dynerr::*;
///# use std::{fmt, error};
///# 
///# ///a custom error type
///# #[derive(Debug)]
///# enum ExampleError1 {
///#     ThisError(u32),
///# }
///# //impl display formatting for error
///# impl fmt::Display for ExampleError1 {
///#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///#         match self {
///#             ExampleError1::ThisError(i)      => write!(f, "ExampleError1::ThisError: {}",i),
///#         }
///#     }
///# }
///# //impl error conversion for error
///# impl error::Error for ExampleError1 {}
///# 
/// fn try_something(x: u32) -> DynResult<u32> {
///     if x > 10 {Ok(x)}
///     else if x < 5 {dynerr!(ExampleError1::ThisError(x))}
///     else {
///         std::fs::File::open("none")?;
///         Ok(x)
///     }
/// }
///#
///# fn main() {
///#     try_something(11);
///# }
/// ```
pub type DynResult<T> = std::result::Result<T, DynError>;


/// A macro for returning custom errors as DynError.
/// 
///# Example
/// 
/// ```rust
///# use dynerr::*;
///# use std::{fmt, error};
///# 
///# //THIS SECTION IS CREATING THE FIRST CUSTOM ERROR
///# ///a custom error type
///# #[derive(Debug)]
///# enum ExampleError1 {
///#     ThisError(u32),
///# }
///# //impl display formatting for error
///# impl fmt::Display for ExampleError1 {
///#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///#         match self {
///#             ExampleError1::ThisError(i)      => write!(f, "ExampleError1::ThisError: {}",i),
///#         }
///#     }
///# }
///# //impl error conversion for error
///# impl error::Error for ExampleError1 {}
///# 
///# //THIS SECTION IS CREATING THE SECOND CUSTOM ERROR
///# #[derive(Debug)]
///# enum ExampleError2 {
///#     ThatError(u32),
///# }
///# impl fmt::Display for ExampleError2 {
///#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///#         match self {
///#             ExampleError2::ThatError(i)      => write!(f, "ExampleError2::ThatError: {}",i),
///#         }
///#     }
///# }
///# impl error::Error for ExampleError2 {}
///# 
///# 
///# ///THIS SECTION IS USING IT
///# 
///# ///shows error handling capabilities using DynError
/// fn example(x: u32) -> DynResult<u32> {
///     match x {
///         1      => Ok(x),                                //Ok
///         2..=4  => dynerr!(ExampleError1::ThisError(x)), //custom error
///         5..=10 => dynerr!(ExampleError2::ThatError(x)), //different custom error
///         _      => {
///             std::fs::File::open("none")?;               //an error not even defined by you!
///             Ok(x)
///         }
///     }
/// }
///# 
///# fn main() {
///#     example(3);
///# }
/// ```
#[macro_export]
macro_rules! dynerr {
    ($e:expr) => {return Err(Box::new($e))};
}

/// Performs a dynamic match operation on multiple error types.
/// 
/// types must be specified beforehand with the "type" keyword.\
/// match arms (excluding the final exhaustive arm) must be specified with the "arm" keyword. 
/// 
///# Example
/// ```rust
///# use dynerr::*;
///# use std::{fmt, error};
///# 
///# //THIS SECTION IS CREATING THE FIRST CUSTOM ERROR
///# ///a custom error type
///# #[derive(Debug)]
///# enum ExampleError1 {
///#     ThisError(u32),
///# }
///# //impl display formatting for error
///# impl fmt::Display for ExampleError1 {
///#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///#         match self {
///#             ExampleError1::ThisError(i)      => write!(f, "ExampleError1::ThisError: {}",i),
///#         }
///#     }
///# }
///# //impl error conversion for error
///# impl error::Error for ExampleError1 {}
///# 
///# //THIS SECTION IS CREATING THE SECOND CUSTOM ERROR
///# #[derive(Debug)]
///# enum ExampleError2 {
///#     ThatError(u32),
///# }
///# impl fmt::Display for ExampleError2 {
///#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///#         match self {
///#             ExampleError2::ThatError(i)      => write!(f, "ExampleError2::ThatError: {}",i),
///#         }
///#     }
///# }
///# impl error::Error for ExampleError2 {}
///# 
///# 
///# ///THIS SECTION IS USING IT
///# 
///# ///shows error handling capabilities using DynError
///# fn example(x: u32) -> DynResult<u32> {
///#     match x {
///#         1      => Ok(x),                                //Ok
///#         2..=4  => dynerr!(ExampleError1::ThisError(x)), //custom error
///#         5..=10 => dynerr!(ExampleError2::ThatError(x)), //different custom error
///#         _      => {
///#             std::fs::File::open("none")?;               //an error not even defined by you!
///#             Ok(x)
///#         }
///#     }
///# }
///# 
///# fn main() {
/// let _i = match example(20) {
///     Ok(i) => i,
///     Err(e) => {
///         dynmatch!(e,                                                                        //the DynError to be matched
///             type ExampleError1 {                                                            //an error type
///                 arm ExampleError1::ThisError(2) => logged_panic!("it was 2!"),              //arm [pattern] => {code}
///                 _ => panic!("{}",e)                                                         //_ => {code}
///             },
///             type ExampleError2 {
///                 arm ExampleError2::ThatError(8) => logged_panic!("it was 8!", "test.log"),
///                 arm ExampleError2::ThatError(9) => 9,
///                 _ => panic!("{}",e)
///             },
///             type std::io::Error {                                                           //an error type not defined by you
///                 arm i if i.kind() == std::io::ErrorKind::NotFound => 5,                      //a match guard included in the match
///                 _ => panic!("{}", e)
///             },
///             _ => panic!("{}",e)                                                             //what to do if error type isn't found
///         )
///     }
/// };
///# }
/// ```
#[macro_export]
macro_rules! dynmatch {
    ($e:expr, $(type $ty:ty {$(arm $( $pattern:pat )|+ $( if $guard: expr )? => $result:expr),*, _ => $any:expr}),*, _ => $end:expr) => (
        $(
            if let Some(e) = $e.downcast_ref::<$ty>() {
                match e {
                    $(
                        $( $pattern )|+ $( if $guard )? => {$result}
                    )*
                    _ => $any
                }
            } else
        )*
        {$end}
    );
}

///deletes the supplied file
pub fn clean_log(log_file: &str) {
    if Path::new(log_file).exists() {
        remove_file(log_file).unwrap_or_else(|e| panic!("Dynerr: Error cleaning file: {}", e))
    }
}

/// deletes the supplied log file.
/// 
/// if no file supplied defaults to "event.log".
/// 
/// #Example
/// ```
///# use dynerr::*;
///# fn main() {
/// log!("hello world", "my.log");
/// clean!("my.log");
/// log!("i just cleaned my.log");
/// clean!();
///# }
/// ```
#[macro_export]
macro_rules! clean {
    () => {
        $crate::clean_log("event.log")
    };
    ($log:expr) => {
        $crate::clean_log($log)
    };
}

/// Appends [event] to [log_file].
/// 
/// creates the file if it doesnt exist.\
/// panics on failure to create or appending to file.\
/// not meant to be used on its own. use logging macros instead
pub fn log<T: fmt::Display>(event: T, log_file: &str) -> T {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .unwrap_or_else(|e| panic!("Dynerr: Error opening log during crash: {} (error passed to logger was: {})",e,event));
    file.write_all(format!("{}\n",event.to_string()).as_bytes())
        .unwrap_or_else(|e| panic!("Dynerr: Error appending to log during crash: {} (error passed to logger was: {})",e,event));
    event
}

/// Appends [event] to [file].
/// 
/// If no file supplied then defaults to "event.log".\
/// creates the file if it doesnt exist.
///
/// 
///# Example
/// 
/// ```rust
///# use dynerr::*;
///# fn main() {
/// log!("this is a test", "test.log");
/// log!("do log!");
///# }
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

/// Appends [event] to [file] then panics.
/// 
/// If no file supplied then defaults to "event.log".\
/// creates the file if it doesnt exist.
/// 
///# Example
/// 
/// ```rust
///# use dynerr::*;
///# fn main() {
///#     if false {
/// logged_panic!("Hi!", "test.log");
/// logged_panic!("default log");
///#     }
///# }
/// ```
#[macro_export]
macro_rules! logged_panic {
    ($e: expr) => {
        panic!("{}",log!($e));
    };

    ($e: expr, $log:expr) => {
        panic!("{}",log!($e, $log));
    }
}


/// Shortcut for .unwrap_or_else(|e| logged_panic!(e)) for DynResult.
/// 
/// If no file supplied then defaults to "event.log".\
/// creates the file if it doesnt exist.
/// 
///# Example
/// 
/// ```rust
///# use dynerr::*;
///# ///shows error handling capabilities using DynError
///# fn try_something() -> DynResult<()> {
///#    Ok(())
///# }
///# 
///# fn main() {
/// let i = check!(try_something());
/// let i = check!(try_something(), "test.log");
///# }
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


///an example
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

    ///shows error handling capabilities using DynError
    fn example(x: u32) -> DynResult<u32> {
        match x {
            1      => Ok(x),                                //Ok
            2..=4  => dynerr!(ExampleError1::ThisError(x)), //custom error
            5..=10 => dynerr!(ExampleError2::ThatError(x)), //different custom error
            _      => {
                std::fs::File::open("none")?;               //an error not even defined by you!
                Ok(x)
            }
        }
    }

    ///THIS SECTION IS USING IT
    #[test]
    pub fn test() -> DynResult<()> {    


        log!("this is a test", "test.log");
        let _i = match example(20) {
            Ok(i) => i,
            Err(e) => {
                dynmatch!(e,                                                                        //the DynError to be matched
                    type ExampleError1 {                                                            //an error group
                        arm ExampleError1::ThisError(2) => logged_panic!("it was 2!"),              //arm [pattern] => {code}
                        _ => panic!("{}",e)                                                         //_ => {code}
                    },
                    type ExampleError2 {
                        arm ExampleError2::ThatError(8) => logged_panic!("it was 8!", "test.log"),
                        arm ExampleError2::ThatError(9..=11) => 10,
                        _ => panic!("{}",e)
                    }, 
                    type std::io::Error {                                                           //an error type not defined by you
                        arm i if i.kind() == std::io::ErrorKind::NotFound => 5,                     //a match guard included in the match
                        _ => panic!("{}", e)
                    },
                    _ => panic!("{}",e)                                                             //what to do if error group isn't found
                )
            }
        };
        log!("do logged_panic! if error");
        let _i = check!(example(1));
        let _i = check!(example(1), "test.log");
        Ok(())
    }
}
