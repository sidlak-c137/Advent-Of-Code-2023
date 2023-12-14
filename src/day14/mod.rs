pub mod day14a;
pub mod day14b;

pub fn solve(part: &str) {
    if part == "a" {
        day14a::solve();
    } else if part == "b" {
        day14b::solve();
    } else {
        println!("Invalid part specified");
    }
}