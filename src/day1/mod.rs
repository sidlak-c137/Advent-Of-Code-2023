pub mod day1a;
pub mod day1b;

pub fn solve(part: &str) {
    if part == "a" {
        day1a::solve();
    } else if part == "b" {
        day1b::solve();
    } else {
        println!("Invalid part specified");
    }
}