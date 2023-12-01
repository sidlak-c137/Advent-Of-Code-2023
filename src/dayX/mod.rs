pub mod dayXa;
pub mod dayXb;

pub fn solve(part: &str) {
    if part == "a" {
        dayXa::solve();
    } else if part == "b" {
        dayXb::solve();
    } else {
        println!("Invalid part specified");
    }
}