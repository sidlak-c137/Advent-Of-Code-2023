use std::collections::HashSet;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day4/data/part_b.txt");
    let mut cards = Vec::new();
    for _ in 0..lines.len() {
        cards.push(1);
    }
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
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
        sum += cards[i];
        if count <= 0 {
            continue;
        }
        for j in 1..=count {
            cards[i + j] += cards[i];
        }
    }
    println!("Sum: {}", sum);
}