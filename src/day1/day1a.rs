use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day1/data/part_a.txt");
    let mut sum = 0;
    for line in lines {
        let mut first_num = 'a';
        let mut last_num = 'a';
        for c in line.chars() { 
            if first_num == 'a' && c.is_digit(10) {
                first_num = c;
            }
            if c.is_digit(10) {
                last_num = c;
            }
        }
        let number = first_num.to_digit(10).unwrap() * 10 + last_num.to_digit(10).unwrap();
        sum += number;
    }
    println!("Sum: {}", sum);
}