# dynerr
some simple tools to help with dynamic error handling/logging\
i got tired of copy pasting these from project to project so i made a crate\
\
the main features of this crate are the dynerr! and dynmatch! macros. when used alongside the return type DynResult<T> it allows you to return multiple error types then match for them!\
using dynerr, theres no need to wrap errors.\

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
dynerr works with any error type from any crate, as long as the type being returned implements std::error::Error then DynResult<T> should be able to handle it\
to directly return a custom error its recommended to use the dynerr! macro instead of Err().\
to match against the dynamic error contained in DynResult, use the dynmatch! macro.\
macro usage looks similar to this
    
```
match example(9) {    //returns a DynResult
    Ok(_) => Ok(()),
    Err(e) => {
        dyn_match!(e, //the DynError to match
            type ExampleError1: ExampleError1::ThisError(2) => panic!("it was 2!"), //match arms to match against
            type ExampleError2: ExampleError2::ThatError(8) => panic!("it was 8!"), //type T: pattern => {code}
            type ExampleError2: ExampleError2::ThatError(9) => println!("it was 9!"),
            default i => panic!("{}", i)    //the final arm if none of the above match
        );
        Ok(())
    }
}
```
\
there are also some simple macros to help with lazy logging\
log! will log an event to event.log\
lodgged_panic! will log an event to event.log then panic
