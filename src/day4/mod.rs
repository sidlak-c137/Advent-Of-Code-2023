pub mod day4a;
pub mod day4b;

pub fn solve(part: &str) {
    if part == "a" {
        day4a::solve();
    } else if part == "b" {
        day4b::solve();
    } else {
        println!("Invalid part specified");
    }
}