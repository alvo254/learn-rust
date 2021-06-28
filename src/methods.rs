// Methods are similar to functions: they’re declared with the fn keyword and
// their name, they can have parameters and a return value, and they contain
// some code that is run when they’re called from somewhere else.
// Their first parameter is always self, which represents
// the instance of the struct the method is being called on.
//-------------------
// Another useful feature of impl blocks is that we’re allowed to define
// functions within impl blocks that don’t take self as a parameter. These
// are called associated functions
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    //more parameters
    fn can_hold(&self, new: &Rectangle) -> bool{
        self.width > new.width && self.height > new.height
    }
    //Lets create an associated function.. because they’re associated with the struct and dont take self
    //as first parameter
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
    //To call this use let sq = Rectangle::square(3); .. more on modules to come
}
pub fn method(){
    let rec = Rectangle{width: 30, height: 50};
    let rec2 = Rectangle{width: 10, height:40};
    //println!("{}", area(&rec));
    println!("{}",rec.area());
    println!("{:?}", rec);
    println!("Can rect1 hold rect2? {}", rec.can_hold(&rec2));
}
//Replaced with method implementation
// fn area(rect: &Rectangle) -> u32{
//     rect.width * rect.height
// }
