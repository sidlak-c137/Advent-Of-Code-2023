use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day15/data/part_a.txt");
    let mut sum = 0;
    for line in lines {
        let parts = line.split(",").collect::<Vec<&str>>();
        for part in parts {
            sum += hash(part);
        }
    }
    println!("Sum: {}", sum);
}

fn hash(input: &str) -> u64 {
    let mut result: u64 = 0;
    for c in input.chars() {
        result += c as u64;
        result = result * 17;
        result = result % 256;
    }
    result
}