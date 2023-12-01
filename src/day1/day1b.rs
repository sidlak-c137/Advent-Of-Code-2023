use crate::helper::lines_from_file;
use std::collections::HashSet;

pub fn solve() {
    let lines = lines_from_file("./src/day1/data/part_b.txt");
    let digits = HashSet::from(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]);
    let mut sum = 0;
    for line in lines {
        let mut first_num = 10;
        let mut last_num = 10;
        let strlen = line.len();
        for (idx, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first_num > 9 {
                    first_num = c.to_digit(10).unwrap();
                }
                last_num = c.to_digit(10).unwrap();
            } else {
                let mut digit = 10;
                if strlen >= 3 && idx < strlen - 2 {
                    let word = &line[idx..idx+3];
                    if digits.contains(word) {
                        match word {
                            "one" => digit = 1,
                            "two" => digit = 2,
                            "six" => digit = 6,
                            _ => (),
                        }
                    }
                }
                if strlen >= 4 && idx < strlen - 3 {
                    let word = &line[idx..idx+4];
                    if digits.contains(word) {
                        match word {
                            "four" => digit = 4,
                            "five" => digit = 5,
                            "nine" => digit = 9,
                            _ => (),
                        }
                    }
                }
                if strlen >= 5 && idx < strlen - 4 {
                    let word = &line[idx..idx+5];
                    if digits.contains(word) {
                        match word {
                            "three" => digit = 3,
                            "seven" => digit = 7,
                            "eight" => digit = 8,
                            _ => (),
                        }
                    }
                }
                if digit < 10 {
                    if first_num > 9 {
                        first_num = digit;
                    }
                    last_num = digit;
                }
            }
            
        }
        sum += first_num * 10 + last_num;
    }
    println!("Sum: {}", sum);
}