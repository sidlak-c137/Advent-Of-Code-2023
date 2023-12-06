mod helper;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];

    if day == "1" {
        day1::solve(part);
    } else if day == "2" {
        day2::solve(part);
    } else if day == "3" {
        day3::solve(part);
    } else if day == "4" {
        day4::solve(part);
    } else if day == "5" {
        day5::solve(part);
    } else if day == "6" {
        day6::solve(part);
    } else {
        println!("Invalid day specified");
    }
}
