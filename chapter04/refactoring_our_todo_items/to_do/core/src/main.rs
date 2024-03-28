mod enums;
mod structs;
mod api;

use api::basic_actions::create::create;
use std::env;

use crate::enums::TaskStatus;


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let title: &String = &args[1];
    let status: TaskStatus = enums::TaskStatus::from_string(&args[2])?;

    let to_do_item = create(
        title, 
        status
    )?;
    println!("{}", to_do_item);
    Ok(())
}
