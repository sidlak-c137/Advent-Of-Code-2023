pub mod day8a;
pub mod day8b;

pub fn solve(part: &str) {
    if part == "a" {
        day8a::solve();
    } else if part == "b" {
        day8b::solve();
    } else {
        println!("Invalid part specified");
    }
}