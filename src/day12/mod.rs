pub mod day12a;
pub mod day12b;

pub fn solve(part: &str) {
    if part == "a" {
        day12a::solve();
    } else if part == "b" {
        day12b::solve();
    } else {
        println!("Invalid part specified");
    }
}