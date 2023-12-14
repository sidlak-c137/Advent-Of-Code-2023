use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day14/data/part_a.txt");
    let mut map = Vec::new();
    for line in lines {
        map.push(line.chars().collect::<Vec<char>>());
    }
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