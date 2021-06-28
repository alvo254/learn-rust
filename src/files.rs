use std::io;
use std::io::Read;
use std::fs::File;

//------------------------------propagating errors--------------------------------------
// For example, Listing 9-6 shows a function that reads a username from a
// file. If the file doesn’t exist or can’t be read, this function will return those
// errors to the code that called this function.

pub fn read(){
    // This means the function is returning a value of the type Result<T, E> where
    // the generic parameter T has been filled in with the concrete type String
    // and the generic type E has been filled in with the concrete type io::Error.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        // let mut f = match f{
        //     Ok(file) => file,
        //     Err(e) => return Err(e)
        // };
        let mut s = String::from("Alvin");
        // match f.read_to_string(&mut s){
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }

        //--------------------short cut for propagating errors using the ? operator---------------------
        // The ? operator can only be used in functions that have a return type of
        // Result, because it is defined to work in the same way as the match expression
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}