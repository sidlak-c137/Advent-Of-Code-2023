use crate::helper::lines_from_file;

pub fn solve() {
    let mut lines = lines_from_file("./src/day13/data/part_a.txt");
    lines.push("".to_string());
    let mut map = Vec::new();
    let mut sum = 0;
    for line in lines {
        if line == "" {
            let row = reflect(&map);
            if row >= 0 {
                sum += (row + 1) * 100;
            } else {
                let col = reflect(&transpose(&map));
                sum += col + 1;
            }
            map = Vec::new();
            continue;
        }
        map.push(line.chars().collect::<Vec<char>>());
    }
    println!("Sum: {}", sum);
}

fn reflect(map: &Vec<Vec<char>>) -> i32 {
    let mut new_map: Vec<String> = Vec::new();
    for row in map {
        new_map.push(row.into_iter().collect());
    }
    for i in 0..(new_map.len() - 1) {
        let mut count = 0;
        for j in 0..new_map.len() {
            if i + 1 + i < j {
                continue;
            }
            if i + 1 + i - j < new_map.len() && new_map[i + 1 + i - j] != new_map[j] {
                for k in 0..map[j].len() {
                    if map[j][k] != map[i + 1 + i - j][k] {
                        count += 1;
                    }
                }
            }
        }
        if count == 0 {
            return i as i32;
        }
    }
    return -1;
}

fn transpose(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for i in 0..map[0].len() {
        let mut row = Vec::new();
        for j in 0..map.len() {
            row.push(map[j][i]);
        }
        new_map.push(row);
    }
    new_map
}

