pub mod day11a;
pub mod day11b;

pub fn solve(part: &str) {
    if part == "a" {
        day11a::solve();
    } else if part == "b" {
        day11b::solve();
    } else {
        println!("Invalid part specified");
    }
}