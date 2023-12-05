use std::collections::HashSet;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day4/data/part_a.txt");
    let mut sum = 0;
    for line in lines {
        let parts = line.split([':', '|']).collect::<Vec<&str>>();
        let mut winning_nums: HashSet<i32> = HashSet::new();
        let mut num = 0;
        for c in parts[1].chars() {
            if c == ' ' && num == 0 {
                continue;
            } else if c == ' ' {
                winning_nums.insert(num);
                num = 0;
            } else {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }
        }
        let mut count = 0;
        for c in parts[2].chars() {
            if c == ' ' && num == 0 {
                continue;
            } else if c == ' ' {
                if winning_nums.contains(&num) {
                    count += 1;
                }
                num = 0;
            } else {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }
        }
        if winning_nums.contains(&num) {
            count += 1;
        }
        if count > 0 {
            sum += u32::pow(2, count - 1);
        }
    }
    print!("Sum: {}", sum);
}