pub mod day9a;
pub mod day9b;

pub fn solve(part: &str) {
    if part == "a" {
        day9a::solve();
    } else if part == "b" {
        day9b::solve();
    } else {
        println!("Invalid part specified");
    }
}