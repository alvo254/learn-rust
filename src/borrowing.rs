pub fn borrow(){
    let mut s = String::from("Alvin");
    change(&mut s);
}
fn change(some_string: &mut String){
    some_string.push_str(" alvo")
}