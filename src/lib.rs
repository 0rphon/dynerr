pub use std::{error, fmt};
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::path::Path;

///an alias for result that impls dynamic errors
pub type DynResult<T> = Result<T, Box<dyn error::Error>>;

///a macro for returning custom errors
#[macro_export]
macro_rules! err {
    ($x: expr) => {Err(Box::new($x))};
}

///logs message to event.log
#[macro_export]
macro_rules! log {
    ($event: expr) => {
        {
            const ERROR_LOG: &str = "event.log";
            fn log_event<T: fmt::Display>(event: T) -> T {
                let mut file = File::open(ERROR_LOG)
                    .unwrap_or_else(|e| panic!("Error opening log during crash: {}\n crash: {}",e,event));
            
                let mut log = String::new();
                file.read_to_string(&mut log)
                    .unwrap_or_else(|e| panic!("Error reading log during crash: {}\n crash: {}",e,event));
                if log != "" { log = format!("{}\n{}", log, event.to_string())}
                else {log = event.to_string()}
            
                let mut file = File::create(ERROR_LOG)
                    .unwrap_or_else(|e| panic!("Error creating log during crash: {}\n crash: {}",e,event));
                file.write_all(log.as_bytes())
                    .unwrap_or_else(|e| panic!("Error writing log during crash: {}\n crash: {}",e,event));
            
                event
            }

            if !Path::new(ERROR_LOG).exists() {
                File::create(ERROR_LOG)
                    .unwrap_or_else(|e| panic!("Error creating log file during crash: {}\n crash: {}",e,$event));
                log_event(format!("Log file created"));
            }
            log_event($event)
        }
    };
}

///logs error then panics
#[macro_export]
macro_rules! logged_panic {
    ($x: expr) => {
        {
            panic!("{}",log!($x));
        }
    };
}





#[test]
pub fn example() {
    ///a custom error type
    #[derive(Debug)]
    enum ExampleError {
        MyError(u32),
    }
    //impl display formatting for error
    impl fmt::Display for ExampleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ExampleError::MyError(i)      => write!(f, "ExampleError::MyError: {}",i),
            }
        }
    }
    //impl error conversion for error
    impl error::Error for ExampleError {}

    fn example(x: u32) -> DynResult<u32> {
        match x {
            1 => Ok(x),
            2 => err!(ExampleError::MyError(x)),
            _ => {
                let y: Result<u32, &str> = Ok(0);
                Ok(y?)
            }
        }
    }

    log!("this is a test");
    example(1).unwrap_or_else(|e| logged_panic!(e));
}