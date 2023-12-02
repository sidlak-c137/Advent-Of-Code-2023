use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day2/data/part_a.txt");
    let mut sum = 0;
    let mut id = 1;
    for line in lines {
        let split = line.split([':', ';']);
        let parts = split.collect::<Vec<&str>>();
        let mut impossible = false;
        for part in &parts[1..] {
            let split2 = part.split(',');
            let draw = split2.collect::<Vec<&str>>();
            for cubes in draw {
                let split3 = cubes.trim().split(' ');
                let cube = split3.collect::<Vec<&str>>();
                let num = cube[0].parse::<i32>().unwrap();
                match cube[1] {
                    "green" => {
                        if num > 13 {
                            impossible = true;
                        }
                    },
                    "red" => {
                        if num > 12 {
                            impossible = true;
                        }
                    },
                    "blue" => {
                        if num > 14 {
                            impossible = true;
                        }
                    },
                    _ => {
                        println!("error {}: {}", id, cube[1]);
                    }
                }
            }
        }
        if !impossible {
            sum += id;
        }
        id += 1;
    }
    println!("Sum: {}", sum);
}