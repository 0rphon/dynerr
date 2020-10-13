# dynerr

Some macros to help with dynamic error handling/logging.\
The goal of this crate is to unify all error types without compromising type safety.\
The crates.io page can be found [here](https://crates.io/crates/dynerr)\
\
The main features of this crate are the `dynerr!` and `dynmatch!` macros. when used alongside the return type `DynResult<T>`, they allows you to return multiple error types from a function then easily match for them during your error handling. Using dynerr, theres no need to ever wrap errors.

```rust
fn example(x: u32) -> DynResult<u32> {                  //returns a dynamic result
    match x {
        1      => Ok(x),                                //Ok()
        2..=4  => dynerr!(ExampleError1::ThisError(x)), //custom error
        5..=10 => dynerr!(ExampleError2::ThatError(x)), //different custom error
        _      => {
            std::env::current_dir()?;                   //an error not even defined by you!
            Ok(x)
        }
    }
}
```

\
`DynError` is an alias for `Box<dyn error::Error>`. Any error that implements `error::Error` can be turned into a `DynError`.\
`DynResult<T>` is just an alias for `Result<T, DynError>` so anything that works on a `Result<T>` will still work on a `DynResult<T>`.\
Dynerr works with any error type from any crate, as long as the type being returned implements `std::error::Error` then `DynResult<T>` should be able to handle it.\
To directly return a custom error its recommended to use the `dynerr!` macro instead of `Err()`.\
To match against the DynError contained in `DynResult<T>`, use the `dynmatch!` macro.\
`dynmatch!` usage looks similar to this:

```rust
 let i = match example(9) { //returns dyn error
     Ok(i) => i,
     Err(e) => {
         dynmatch!(e,                                                    //the error to match
             type ExampleError1 {                                        //enum error type
                 arm ExampleError1::ThisError(2) => panic!("it was 2!"), //arm [pattern] => {code}
                 _ => panic!("{}",e)                                     //_ => {code}
             },
             type ExampleError2 {                                        //another enum error type
                 arm ExampleError2::ThatError(8) => panic!("it was 8!"), //more arms to match against
                 arm ExampleError2::ThatError(9) => 9,
                 _ => panic!("{}",e)                                     //a final exhaustive match
             },
             _ => panic!("{}",e)                                         //final exhaustive match if type not found
         )
     }
 };
```

\
Aside from its main features, dynerr also has some simple macros to help with lazy logging.\
`log!` will log an event to the supplied file. Defaults to event.log if no log file supplied.\
`logged_panic!` will log an event to file then panic. Defaults to event.log if no log file supplied.\
`check!` will call `.unwrap_or_else(|e| logged_panic!(e))` on a result. Defaults to event.log if no log file supplied.\
If the supplied file doesn't exist then these macros will attempt to create the file.\
To delete a log file use the `clean!` macro.\
These macros all rely on either the `log` or `clean_log` functions. these functions are capable of panicking but shouldn't ever need to under normal circumstances.
\
\
A complete example:

```rust
use dynerr::*;
use std::{fmt, error};

//THIS SECTION IS CREATING THE FIRST CUSTOM ERROR
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


///THIS SECTION IS USING IT

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

fn main() -> DynResult<()> {
    log!("this is a test", "test.log");
    let _i = match example(2) {
        Ok(i) => i,
        Err(e) => {
            dynmatch!(e,                                                                        //the DynError to be matched
                type ExampleError1 {                                                            //an error type
                    arm ExampleError1::ThisError(2) => logged_panic!("it was 2!"),              //arm [pattern] => {code}
                    _ => panic!("{}",e)                                                         //_ => {code}
                },
                type ExampleError2 {
                    arm ExampleError2::ThatError(8) => logged_panic!("it was 8!", "test.log"),
                    arm ExampleError2::ThatError(9) => 9,
                    _ => panic!("{}",e)
                },
                type std::io::Error {                                                           //an error type not defined by you
                    arm i if i.kind() == std::io::ErrorKind::NotFound => panic!("not found"),   //a match guard included in the match
                    _ => panic!("{}", e)
                },
                _ => panic!("{}",e)                                                             //what to do if error type isn't found
            )
        }
    };
    log!("do logged_panic! if error");
    let _i = check!(example(11));
    let _i = check!(example(9), "test.log");
    Ok(())
}
```
