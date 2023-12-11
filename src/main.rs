mod helper;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

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
    } else if day == "7" {
        day7::solve(part);
    } else if day == "8" {
        day8::solve(part);
    } else if day == "9" {
        day9::solve(part);
    } else if day == "10" {
        day10::solve(part);
    } else if day == "11" {
        day11::solve(part);
    } else {
        println!("Invalid day specified");
    }
}
