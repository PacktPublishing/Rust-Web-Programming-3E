//! File: to_do/core/src/main.rs
mod enums;
mod structs;
mod api;
use api::basic_actions::create::create;
use crate::enums::TaskStatus;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let status_enum = TaskStatus::from_string(
        &args.status
    )?;
    let to_do_item = create(
        &args.title, 
        status_enum
    )?;
    println!("{}", to_do_item);
    Ok(())
}

