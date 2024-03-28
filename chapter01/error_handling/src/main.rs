fn error_check(check: bool) -> Result<i8, &'static str> {
    if check {
        Err("this is an error")
        } 
    else {
        Ok(1)
    }
}

fn error_check_two(check: bool) -> Result<i8, &'static str> {
    let outcome: i8 = error_check(check)?;
    Ok(outcome)
}



fn main() {
    println!("{:?}", error_check(false));
    println!("{:?}", error_check(false).is_err());
    println!("{:?}", error_check(true));
    println!("{:?}", error_check(true).is_err());

}
