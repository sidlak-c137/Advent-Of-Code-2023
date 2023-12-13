pub mod day13a;
pub mod day13b;

pub fn solve(part: &str) {
    if part == "a" {
        day13a::solve();
    } else if part == "b" {
        day13b::solve();
    } else {
        println!("Invalid part specified");
    }
}