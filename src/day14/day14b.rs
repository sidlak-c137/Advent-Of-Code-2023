use std::collections::HashMap;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day14/data/part_a.txt");
    let mut map = Vec::new();
    for line in lines {
        map.push(line.chars().collect::<Vec<char>>());
    }
    let mut seen = HashMap::new();
    let mut num = 0;
    while num < 1000000000 && ((seen.len() == 0) || !seen.contains_key(&stringify(&map))) {
        seen.insert(stringify(&map), num);
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
        num += 1;
    }
    let seen = seen.get(&stringify(&map)).unwrap();
    num = (1000000000 - seen) % (num - seen);
    while num > 0 {
        num -= 1;
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
        calc(&mut map);
        rotate(&mut map);
    }
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                sum += map.len() - i;
            }
        }
    }
    println!("Sum: {}", sum);
}

pub fn rotate(map: &mut Vec<Vec<char>>) {
    let mut new_map = Vec::new();
    for i in 0..map[0].len() {
        let mut row = Vec::new();
        for j in 0..map.len() {
            row.push(map[j][i]);
        }
        new_map.push(row);
    }
    for i in 0..new_map.len() {
        new_map[i].reverse();
    }
    *map = new_map;
}

pub fn stringify(map: &Vec<Vec<char>>) -> String {
    let mut new_map: Vec<String> = Vec::new();
    for row in map {
        new_map.push(row.into_iter().collect());
    }
    return new_map.join("\n");
}

pub fn calc(map: &mut Vec<Vec<char>>) {
    let mut rocks = vec![0; map[0].len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                rocks[j] = i + 1;
            } else if map[i][j] == 'O' {
                map[i][j] = '.';
                map[rocks[j]][j] = 'O';
                rocks[j] += 1;
            }
        }
    }
}