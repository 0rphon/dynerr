# dynerr

some simple tools to help with dynamic error handling/logging\
i got tired of copy pasting these from project to project so i made a crate\
\
the main features of this crate are the dynerr! and dynmatch! macros. when used alongside the return type DynResult\<T\> it allows you to return multiple error types then match for them!\
using dynerr, theres no need to wrap errors.

```
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
```

\
as you can see above, Ok() and ? still work fine.\
dynerr works with any error type from any crate, as long as the type being returned implements std::error::Error then DynResult\<T\> should be able to handle it.\
to directly return a custom error its recommended to use the dynerr! macro instead of Err().\
to match against the dynamic error contained in DynResult, use the dynmatch! macro.\
macro usage looks similar to this:

```
 let i = match example(9) { //returns dyn error
     Ok(i) => i,
     Err(e) => {
         dynmatch!(e,                                                    //the error to match
             type ExampleError1 {                                        //enum error type
                 arm ExampleError1::ThisError(2) => panic!("it was 2!"), //arm [pattern] => {code}
                 _ => panic!("{}",e)                                     //_ => {code}
             },
             type ExampleError2 {                                        //another enum error type
                 arm ExampleError2::ThatError(8) => panic!("it was 8!"), //more arms
                 arm ExampleError2::ThatError(9) => 9,
                 _ => panic!("{}",e)                                     //more wildcard matches
             },
             _ => panic!("{}",e)                                         //final wildcard if type not found
         )
     }
 };
```

\
there are also some simple macros to help with lazy logging.\
log! will log an event to the supplied file. defaults to event.log if no log file supplied\
logged_panic! will log an event to file then panic. defaults to event.log if no log file supplied\
check! will call .unwrap_or_else(|e| logged_panic!(e)) on a result. same as above, defaults to event.log if no log file supplied\
if the supplied file doesn't exist then these macros will attempt to create the file.\
these macros all rely on a log() function. log() is capable of panic but shouldn't ever need to under normal circumstances.
