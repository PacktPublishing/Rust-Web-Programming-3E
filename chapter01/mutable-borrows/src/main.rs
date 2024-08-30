fn print(value: &mut i8) {
    *value += 1;
    println!("In function the value is: {}", value);
}

fn print_two(value: &mut i8, value_two: &mut i8) {
    *value += 1;
    println!("In function the value is: {}", value);
    *value_two += 1;
}



fn main() {
    let mut one: i8 = 5;
    print(&mut one);
    println!("In main the value is: {}", one);

    let mut one: i8 = 5;
    // print_two(&mut one, &mut one); // this will not compile
    println!("In main the value is: {}", one);
}
