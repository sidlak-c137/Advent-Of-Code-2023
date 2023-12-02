use crate::helper::lines_from_file;
use std::cmp;

pub fn solve() {
    let lines = lines_from_file("./src/day2/data/part_b.txt");
    let mut sum = 0;
    for line in lines {
        let split = line.split([':', ';']);
        let parts = split.collect::<Vec<&str>>();
        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;
        for part in &parts[1..] {
            let split2 = part.split(',');
            let draw = split2.collect::<Vec<&str>>();
            for cubes in draw {
                let split3 = cubes.trim().split(' ');
                let cube = split3.collect::<Vec<&str>>();
                let num = cube[0].parse::<i32>().unwrap();
                match cube[1] {
                    "green" => {
                        green = cmp::max(green, num);
                    },
                    "red" => {
                        red = cmp::max(red, num);
                    },
                    "blue" => {
                        blue = cmp::max(blue, num);
                    },
                    _ => {
                        println!("error {}: {}", cube[0], cube[1]);
                    }
                }
            }
        }
        sum += green * red * blue;
    }
    println!("Sum: {}", sum);
}