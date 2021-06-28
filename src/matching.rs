#[derive(Debug)]
enum States{
    Nairobi,

    Mombasa,
    Nakuru,
    Nyeri
}
enum Coins{
    Nickle,
    Dim,
    Penny(States),
    Quarter,
}
pub fn matched(){

    fn catch(coin: Coins) -> i64{
        match coin {
            Coins::Penny(state) => {println!("Lucky Penny: {:?}", state);10},
            Coins::Nickle => 5,
            Coins::Dim => 10,
            Coins::Quarter => 25,
        }
    }
    let uses = catch(Coins::Penny(States::Nakuru));
    let u = catch(Coins::Nickle);
    println!("this is value of a coin: {}", uses);


    //Matching with option<T>
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", seven);
    println!("{:?}", five);

    // The if let syntax lets you combine if and let into a less verbose way to
    // handle values that match one pattern while ignoring the rest.
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let tee = te();
    //println!("this te: {:?}", tee)
}
// Rust also has a pattern we can use when we don’t want to list all possible
// values.
pub fn te(){
    let some_value = 0u8;
    match  some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };
    println!("some {:?}",some_value);
}
