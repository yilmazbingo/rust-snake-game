- If your program fails in a way that is unrecoverable or you cannot handle the error gracefully, then you can call the `panic macro` will immediately quit your program and print out an error message.

```rs
fn main(){
    panic!("crash and burn")
}
```

when it throws error: you get a note: "run with `RUST_BACKTRACE=1` envrionment variable to display a backtrace". backtrace will list out all the functions called leading up to the error.

    `RUST_BACKTRACE=1 cargo run`

## Recoverable Errors:

we use `Result` enum which is similar to `Option` enum.

```rs

use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f=File::open("hello.txt")
    // open returns Result type. Result<File,Error>
    let f=match f {
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                // if file not found, attempt to create a new file. it might fail
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file {:?} ",e)
            }
            other_error:ErrorKind =>{
                panic!("Problem opening the file: {:?} ",other_error)
            }
        }
    }
}
```

- write closures instead of match statements

```rs
use std::fs::File;
use std::io::ErrorKind;
// pay attention unwrap_or_else(|error| it does not have closing )
// unwrap_or_else will give us file or anonymous function closure, passing in the error
let f=File::open("hello.txt").unwrap_or_else(|error|{
    if error.kind()==ErrorKind::NotFound{
        File::create("hello.txt").unwrap_or_else(|error|{
            panic!("Problem creating the file",error);
        // we dont have ; after parentheses, means the abouve File::create is an expression
        })
    } else {
        panic!("Problem opening the file: {:?}",error);
    }
});
}
```

## unwrap()

```rs
    fn main(){
        // unwrap does the same thing as in the match statement
        let f=File::open("hello.txt").unwrap()
        // instead of unwrap
        let f=File::open("hello.txt").expect("Failed to open hello.txt")

    }
```

## Error Propagation

- When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // this will read the file and store it in s. it returns Result
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

- we could write the above function in a shorter way

```rs
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // we are adding "?" to the end. it is used instead of match. if err, instead of panicking, function will return early
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // we are borrowing f here. that is why it is mut f
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

## method chaining

```rs
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // if function fails, we just return because of ?
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

- fs module itelf also have a method `read_to_string`

```rs
    use std::fs;
    use std::io;
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
```

**Where The ? Operator Can Be Used**
The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined in Listing 9-6. In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that it’s compatible with this return.

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

this will return error:"cannot use the `?` operator in a function that returns `()`"
This code opens a file, which might fail. The ? operator follows the Result value returned by File::open, but this main function has the return type of ()

- in general, default should be using Result enum and error propogation. this prevents your program from crashing. You should use only panic in exceptional circumstances. in circumstances in which recovering from the error is not possible and your program cannot continue because it is in a bad state.
