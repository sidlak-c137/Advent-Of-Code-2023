pub mod day2a;
pub mod day2b;

pub fn solve(part: &str) {
    if part == "a" {
        day2a::solve();
    } else if part == "b" {
        day2b::solve();
    } else {
        println!("Invalid part specified");
    }
}