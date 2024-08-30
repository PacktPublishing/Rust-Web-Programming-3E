
fn print(value: String) {
    println!("{}", value);
}



fn main() {
    let one: String = String::from("one");
    let two: String = one + " two";
    println!("{}", two);
    // println!("{}", one); // this will not compile

    let one = "one".to_string();
    print(one);
    // println!("{}", one); // this will not compile
}
