pub mod day6a;
pub mod day6b;

pub fn solve(part: &str) {
    if part == "a" {
        day6a::solve();
    } else if part == "b" {
        day6b::solve();
    } else {
        println!("Invalid part specified");
    }
}