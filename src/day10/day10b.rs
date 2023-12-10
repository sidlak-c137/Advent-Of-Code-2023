use std::{vec, collections::VecDeque};

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

    let map2 = map.clone();

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
    let mut steps: Vec<Vec<usize>> = Vec::new();
    loop {
        steps.push(vec![i, j]);
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
    steps.push(vec![i, j]);
    map = map2;
    for step in &steps {
        map[step[0]][step[1]] = 'X';
    }
    let mut total = 0;
    let mut ud: i32 = 0;
    let mut lr: i32 = 0;
    let mut dir = "";
    let mut inside = true;
    for i in 0..steps.len() {
        let step = &steps[i];
        let next = &steps[(i + 1) % steps.len()];
        if ud == 0 && lr == 0 {
            if step[0] < next[0] {
                lr = -1;
                dir = "down";
            } else if step[0] > next[0] {
                lr = 1;
                dir = "up";
            } else if step[1] < next[1] {
                ud = 1;
                dir = "right";
            } else if step[1] > next[1] {
                ud = -1;
                dir = "left";
            }
        } else {
            if ud == 0 && lr == 1 {
                if dir == "up" {
                    if step[1] > next[1] {
                        lr = 0;
                        ud = -1;
                        dir = "left";
    
                    } else if step[1] < next[1] {
                        lr = 0;
                        ud = 1;
                        dir = "right";
                    }
                } else if dir == "down" {
                    if step[1] > next[1] {
                        lr = 0;
                        ud = 1;
                        dir = "left";
    
                    } else if step[1] < next[1] {
                        lr = 0;
                        ud = -1;
                        dir = "right";
                    }
                }
            } else if ud == 0 && lr == -1 {
                if dir == "up" {
                    if step[1] > next[1] {
                        lr = 0;
                        ud = 1;
                        dir = "left";
    
                    } else if step[1] < next[1] {
                        lr = 0;
                        ud = -1;
                        dir = "right";
                    }
                } else if dir == "down" {
                    if step[1] > next[1] {
                        lr = 0;
                        ud = -1;
                        dir = "left";
    
                    } else if step[1] < next[1] {
                        lr = 0;
                        ud = 1;
                        dir = "right";
                    }
                }
            } else if ud == 1 && lr == 0 {
                if dir == "right" {
                    if step[0] > next[0] {
                        lr = 1;
                        ud = 0;
                        dir = "up";
    
                    } else if step[0] < next[0] {
                        lr = -1;
                        ud = 0;
                        dir = "down";
                    }
                } else if dir == "left" {
                    if step[0] > next[0] {
                        lr = -1;
                        ud = 0;
                        dir = "up";
    
                    } else if step[0] < next[0] {
                        lr = 1;
                        ud = 0;
                        dir = "down";
                    }
                }
            } else if ud == -1 && lr == 0 {
                if dir == "right" {
                    if step[0] > next[0] {
                        lr = -1;
                        ud = 0;
                        dir = "up";
    
                    } else if step[0] < next[0] {
                        lr = 1;
                        ud = 0;
                        dir = "down";
                    }
                } else if dir == "left" {
                    if step[0] > next[0] {
                        lr = 1;
                        ud = 0;
                        dir = "up";
    
                    } else if step[0] < next[0] {
                        lr = -1;
                        ud = 0;
                        dir = "down";
                    }
                }
            } 
        }
        let mut queue = VecDeque::new();
        let mut count = 0;
        queue.push_back(vec![step[0] as i32 - ud, step[1] as i32 - lr]);
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if curr[0] < 1 || curr[0] >= (map.len() - 1) as i32 || curr[1] < 1 || curr[1] >= (map[0].len() - 1) as i32 {
                inside = false;
                continue;
            }
            if map[curr[0] as usize][curr[1] as usize] != 'X' {
                count += 1;
            }
            if map[curr[0] as usize][curr[1] as usize] == 'X' {
                continue;
            }
            map[curr[0] as usize][curr[1] as usize] = 'X';
            queue.push_back(vec![curr[0] + 1, curr[1]]);
            queue.push_back(vec![curr[0] - 1, curr[1]]);
            queue.push_back(vec![curr[0], curr[1] + 1]);
            queue.push_back(vec![curr[0], curr[1] - 1]);
        }
        total += count;
    }
    if inside {
        println!("Total: {}", total);
    } else {
        total = 0;
        for i in 1..map.len() - 1 {
            for j in 1..map[i].len() - 1 {
                if map[i][j] != 'X' {
                    total += 1;
                }
            }
        }
        println!("Total: {}", total);
    }
}