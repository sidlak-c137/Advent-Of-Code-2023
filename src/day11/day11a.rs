use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day11/data/part_a.txt");
    let mut space = Vec::new();
    for line in lines {
        space.push(line.chars().collect::<Vec<char>>());
    }
    let mut i = 0;
    while i < space.len() {
        let mut galaxy = 0;
        for c in &space[i] {
            if *c == '#' {
                galaxy += 1;
            }
        }
        if galaxy == 0 {
            space.insert(i, space[i].clone());
            i += 1;
        }
        i += 1;
    }
    let mut j = 0;
    while j < space[0].len() {
        let mut galaxy = 0;
        for i in 0..space.len() {
            if space[i][j] == '#' {
                galaxy += 1;
            }
        }
        if galaxy == 0 {
            for i in 0..space.len() {
                space[i].insert(j, '.');
            }
            j += 1;
        }
        j += 1;
    }

    let mut coords = Vec::new();
    for i in 0..space.len() {
        for j in 0..space[i].len() {
            if space[i][j] == '#' {
                coords.push((i as u32, j as u32));
            }
        }
    }
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let dist = ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u32;
            sum += dist;
        }
    }
    println!("Sum: {}", sum);
}