use std::vec;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day10/data/part_a.txt");
    let mut map = Vec::new();
    for line in lines {
        let mut c = line.chars().collect::<Vec<char>>();
        c.insert(0, '.');
        c.push('.');
        map.push(c);
    }
    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);

    let mut i = 0;
    let mut j = 0;
    'outer: for r in 0..map.len() {
        for c in 0..map[i].len() {
            if map[r][c] == 'S' {
                i = r;
                j = c;
                break 'outer;
            }
        }
    }
    let downs = vec!['|', 'L', 'J', 'S'];
    let ups = vec!['|', 'F', '7', 'S'];
    let lefts = vec!['-', 'L', 'F', 'S'];
    let rights = vec!['-', '7', 'J', 'S'];
    let mut steps = 0;
    loop {
        steps += 1;
        if map[i][j] == 'S' {
            if downs.contains(&map[i + 1][j]) {
                map[i][j] = '.';
                i += 1;
            } else if ups.contains(&map[i - 1][j]) {
                map[i][j] = '.';
                i -= 1;
            } else if lefts.contains(&map[i][j - 1]) {
                map[i][j] = '.';
                j -= 1;
            } else if rights.contains(&map[i][j + 1]) {
                map[i][j] = '.';
                j += 1;
            }
        } else {
            if map[i][j] == '|' {
                if map[i + 1][j] != '.' {
                    map[i][j] = '.';
                    i += 1;
                } else if i - 1 < map.len() && map[i - 1][j] != '.' {
                    map[i][j] = '.';
                    i -= 1;
                } else {
                    break;
                }
            } else if map[i][j] == '-' {
                if map[i][j + 1] != '.' {
                    map[i][j] = '.';
                    j += 1;
                } else if map[i][j - 1] != '.' {
                    map[i][j] = '.';
                    j -= 1;
                } else {
                    break;
                }
            } else if map[i][j] == 'L' {
                if  map[i - 1][j] != '.' {
                    map[i][j] = '.';
                    i -= 1;
                } else if map[i][j + 1] != '.' {
                    map[i][j] = '.';
                    j += 1;
                } else {
                    break;
                }
            } else if map[i][j] == '7' {
                if map[i + 1][j] != '.' {
                    map[i][j] = '.';
                    i += 1;
                } else if map[i][j - 1] != '.' {
                    map[i][j] = '.';
                    j -= 1;
                } else {
                    break;
                }
            } else if map[i][j] == 'J' {
                if map[i - 1][j] != '.' {
                    map[i][j] = '.';
                    i -= 1;
                } else if map[i][j - 1] != '.' {
                    map[i][j] = '.';
                    j -= 1;
                } else {
                    break;
                }
            } else if map[i][j] == 'F' {
                if map[i + 1][j] != '.' {
                    map[i][j] = '.';
                    i += 1;
                } else if map[i][j + 1] != '.' {
                    map[i][j] = '.';
                    j += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    println!("Steps: {}", (steps + 1) / 2);
}