use std::fs::File;
use std::io::ErrorKind;

//Rust groups errors into two major categories: recoverable and unrecoverable
//errors.
// For a recoverable error, such as a file not found error, it’s reasonable
// to report the problem to the user and retry the operation. Unrecoverable
// errors are always symptoms of bugs, like trying to access a location beyond
// the end of an array.
//N.B Rust doesnt have exceptions instead it has type Result<T, E> for recoverable
// errors and the panic! macro that stops execution when the program encounters
// an unrecoverable error
//-----------------------------------------------------------------------------------------------
// By default, when a panic occurs, the program starts unwinding, which means
// Rust walks back up the stack and cleans up the data from each function it
// encounters. But this walking back and cleanup is a lot of work so alternative is to add
// panic = 'abort' to the appropriate [profile] sections in your Cargo.toml
//For the release
// [profile.release]
// panic = 'abort'
pub fn error(){
    //panic!("crash and burn");
    let f = File::open("hello.txt");
    let f = match f{
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {  //this is a match guard
            match File::create("Hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {}", error)
        },
    };
}

















