use std::collections::HashMap;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day8/data/part_a.txt");
    let steps = lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    for line in &lines[2..] {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let key = parts[0];
        let left = &parts[1][1..=3];
        let right = &parts[1][6..=8];
        map.insert(key, [left, right]);
    }
    let mut count = 0;
    let mut curr = "AAA";
    loop {
        let step = steps[count % steps.len()];
        if step == 'L' {
            curr = map.get(curr).unwrap()[0];
        } else {
            curr = map.get(curr).unwrap()[1];
        }
        count += 1;
        if curr == "ZZZ" {
            break;
        }
    }
    println!("Count: {}", count);
}