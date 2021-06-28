//A dangling pointer is a pointer that references a location in memory that may have been give
//to someone else by freeing some memory while preserving a pointer to that memory

pub fn dang(){
    let refer_to_nothing = no_dangle();
}
//--------------------------------------------------------
//This block of code wount work
// fn dangle() -> &String{
//     let s = String::from("alvin");
//     &s
// }
//--------------------------------------------------------


//This works well
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
