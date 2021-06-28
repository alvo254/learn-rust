//primitive string = fixed string immutable stored somewhere in memory
//string growable heap allocated use when need to modify or own string data

pub fn stri() {
    // this is a primitive string
    let set = "Hello";
    //this is a growable string
    let mut s = String::from("Alvin ");
    //Get the string length
    println!("Length of string is: {}", s.len());
    //String methods
    s.push('w');
    s.push_str(" السلام عليكم");
    // creating a new empty string
    let word = String::new();

    //Using the to_string method to create a String from a string literal
    let words = "Inital data".to_string();

    //get capacity in bytes
    println!("can hold this bytes {}", s.capacity());

    //check if is_empty
    println!("if is empty {}", s.is_empty());

    //check if word is contained in string
    println!("word present {}", s.contains("rs"));

    //loop through by white spaces
    for i in s.split_whitespace(){
        println!("Loop starts here {}", i);
    }


    println!("{}", set);
    println!("{}", s);
}