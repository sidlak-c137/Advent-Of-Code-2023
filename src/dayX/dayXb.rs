use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/dayX/data/part_a.txt");
    for line in lines {
        println!("{}", line);
    }
}