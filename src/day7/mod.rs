pub mod day7a;
pub mod day7b;

pub fn solve(part: &str) {
    if part == "a" {
        day7a::solve();
    } else if part == "b" {
        day7b::solve();
    } else {
        println!("Invalid part specified");
    }
}