pub mod day3a;
pub mod day3b;

pub fn solve(part: &str) {
    if part == "a" {
        day3a::solve();
    } else if part == "b" {
        day3b::solve();
    } else {
        println!("Invalid part specified");
    }
}