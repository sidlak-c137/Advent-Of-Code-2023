mod helper;
mod day1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];

    if day == "1" {
        day1::solve(part);
    } else {
        println!("Invalid day specified");
    }
}
