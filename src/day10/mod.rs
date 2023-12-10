pub mod day10a;
pub mod day10b;

pub fn solve(part: &str) {
    if part == "a" {
        day10a::solve();
    } else if part == "b" {
        day10b::solve();
    } else {
        println!("Invalid part specified");
    }
}