use std::env;
use clap::Parser;


/// Simple program for booking
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// first name of user
    #[arg(short, long)]
    first_name: String,

    /// last name of user
    #[arg(short, long)]
    last_name: String,

    /// age of the user
    #[arg(short, long, default_value_t = 1)]
    age: u8,
}


fn main() {
    // Collect all the arguments passed to the program
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let args = Args::parse();
    println!("{:?}", args.first_name);
    println!("{:?}", args.last_name);
    println!("{:?}", args.age);

}
