//Enums allow you to create a type by enumerating its possible values
// v4 and v6 are now what we call variants of the enum
// Note that the variants of the enum are namespaced under its identifier,
// and we use a double colon to separate the two. The reason this is useful
// is that now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same
// type: IpAddrKind. We can then, for instance, define a function that takes any
// IpAddrKind:

//There’s another advantage to using an enum rather than a struct: each
//variant can have different types and amounts of associated data
//----------------------------
//You should know that rust has no null or none but it provides an option to simulate them which is
//Option<T>
//implemented as ...
// enum Option<T>{
// some(T),
// None,
//}
//The <T> is a generic type to be encountered later
// use std::Option;
#[derive(Debug)]
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

// enum Option<T>{
//     Some(T),
//     None,
// }

// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }
pub fn enums_to(){
    //declare directly to variants v4(String) to use ::
    let home = IpAddrKind::V4(127,0,0,1);
    //let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));
    let _some_number = Some(5);
    let _some_thing = Some(6);
    //let absent_number: option<i32> = None;
    println!("{:?}", home)

    //If we want to use struct to do the same above but longer
    //let home = IpAddr{
    //      kind: IpAddrKind::v4,
    //      address: String::from("127.0.0.1");

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}