pub fn user(){
    #[derive(Debug)] //This option is used with the debug display (:?) to print out everything in the struct and enums
    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User{
        username: String::from("alvin"),
        email: String::from("alvin@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("This is the structs username: {}", user1.username);

    let user2 = User{
        username: String::from("Ndung'u"),
        email: String::from("ndungu@gmail.com"),
        //creating an instance from a previous instance with struct update syntax
        ..user1
    };
    //You can use either {:?} to print strings or {:#?} to print pretty strings
    println!("This is user2's email: {:?}", user2)
}
