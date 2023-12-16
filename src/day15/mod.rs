pub mod day15a;
pub mod day15b;

pub fn solve(part: &str) {
    if part == "a" {
        day15a::solve();
    } else if part == "b" {
        day15b::solve();
    } else {
        println!("Invalid part specified");
    }
}