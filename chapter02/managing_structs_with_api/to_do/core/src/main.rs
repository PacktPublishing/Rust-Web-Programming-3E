mod enums;
mod structs;
mod api;

use api::basic_actions::create::create;


fn main() {
    let to_do_item = create(
        "washing", 
        enums::TaskStatus::PENDING
    );
    println!("{}", to_do_item);
}
