use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day15/data/part_b.txt");
    let mut sum = 0;
    let mut boxes: Vec<Vec<(String, u32)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for line in lines {
        let parts = line.split(",").collect::<Vec<&str>>();
        for part in parts {
            let mut lens = "".to_string();
            let mut focus = 0;
            let mut symbol = "".to_string();
            for c in part.chars() {
                if c.is_alphabetic() {
                    lens.push(c);
                } else if c.is_digit(10) {
                    focus = focus * 10 + c.to_digit(10).unwrap();
                } else {
                    symbol.push(c);
                }
            }
            let b = hash(&lens);
            let mut found = false;
            for i in 0..boxes[b as usize].len() {
                let val = &boxes[b as usize][i];
                if symbol == "-" {
                    if val.0 == lens {
                        boxes[b as usize].remove(i);
                        found = true;
                        break;
                    }
                } else if symbol == "=" {
                    if val.0 == lens {
                        boxes[b as usize][i].1 = focus;
                        found = true;
                        break;
                    }
                }
            }
            if !found && symbol == "=" {
                boxes[b as usize].push((lens, focus));
            }
            for i in 0..256 {
                if boxes[i].len() == 0 {
                    continue;
                }
            }
        }
    }
    for i in 0..256 {
        for j in 0..boxes[i].len() {
            sum += (1 + i as u32) * (1 + j as u32) * boxes[i][j].1;
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