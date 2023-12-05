pub mod day5a;
pub mod day5b;

pub fn solve(part: &str) {
    if part == "a" {
        day5a::solve();
    } else if part == "b" {
        day5b::solve();
    } else {
        println!("Invalid part specified");
    }
}