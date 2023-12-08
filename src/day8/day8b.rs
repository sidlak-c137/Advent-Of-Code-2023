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
    let mut counts = Vec::new();
    for k in map.keys() {
        if !k.ends_with('A') {
            continue;
        }
        let mut count = 0;
        let mut curr = *k;
        loop {
            let step = steps[count % steps.len()];
            if step == 'L' {
                curr = map.get(curr).unwrap()[0];
            } else {
                curr = map.get(curr).unwrap()[1];
            }
            count += 1;
            if curr.ends_with('Z') {
                break;
            }
        }
        counts.push(count as u64);
    }
    println!("{}", lcm(counts));
}

pub fn lcm(mut nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    nums.remove(0);
    let b = lcm(nums);
    a * (b / gcd_of_two_numbers(a, b))
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}