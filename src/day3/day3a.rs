use std::collections::HashMap;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day3/data/part_a.txt");
    let mut part_locations: HashMap<i32, HashMap<i32, Vec<[i32; 2]>>> = HashMap::new();
    let mut sum = 0;
    let mut symbol_locations: Vec<[i32; 2]> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let mut num_so_far: i32 = 0;
        let mut pos: Vec<[i32; 2]> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                num if num.is_digit(10) => {
                    let number = num.to_digit(10).unwrap() as i32;
                    num_so_far = num_so_far * 10 + number;
                    pos.push([i as i32, j as i32]);
                },
                '.' => {
                    add_all(num_so_far, &pos, &mut part_locations);
                    num_so_far = 0;
                    pos.clear();
                },
                _ => {
                    add_all(num_so_far, &pos, &mut part_locations);
                    num_so_far = 0;
                    pos.clear();
                    symbol_locations.push([i as i32, j as i32]);
                }
            }
        }
        add_all(num_so_far, &pos, &mut part_locations);
    }
    let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0], [1, 1], [-1, -1], [1, -1], [-1, 1]];
    for p in symbol_locations {
        for dir in dirs {
            let i = p[0] + dir[0];
            let j = p[1] + dir[1];
            if part_locations.contains_key(&i) && part_locations.get(&i).unwrap().contains_key(&j) {
                let locs = part_locations.get(&i).unwrap().get(&j).unwrap().clone();
                for (k, loc) in locs.iter().enumerate() {
                    let row: i32 = loc[0];
                    let col: i32 = loc[1];
                    if k == 0 {
                        sum += loc[0];
                    } else {
                        part_locations.get_mut(&row).unwrap().remove(&col);
                        if part_locations.get(&row).unwrap().len() == 0 {
                            part_locations.remove(&row);
                        }
                    }
                }
            }
        }
    }
    println!("Sum: {}", sum);
}

pub fn add_all(num_so_far: i32, pos: &Vec<[i32; 2]>, part_locations: &mut HashMap<i32, HashMap<i32, Vec<[i32; 2]>>>) {
    if pos.len() == 0 {
        return;
    }
    let mut positions: Vec<[i32; 2]> = Vec::new();
    positions.push([num_so_far, 0]);
    for p in pos {
        positions.push([p[0], p[1]]);
    }

    for p in pos {
        if !part_locations.contains_key(&p[0]) {
            part_locations.insert(p[0], HashMap::new());
        }
        part_locations.get_mut(&p[0]).unwrap().insert(p[1], positions.clone());
    }
}