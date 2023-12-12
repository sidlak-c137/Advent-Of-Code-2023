use std::collections::HashMap;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day12/data/part_a.txt");
    let mut sum = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let nums = parts[1].split(",").collect::<Vec<&str>>();
        let nums = nums.iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let mut unfolded_nums = Vec::new();
        for _ in 0..5 {
            unfolded_nums.extend(nums.clone());
        }
        let chars = parts[0].chars().collect::<Vec<char>>();
        let mut unfolded_chars = chars.clone();
        for _ in 0..4 {
            unfolded_chars.push('?');
            unfolded_chars.extend(chars.clone());
        }
        unfolded_chars.push('.');
        let mut dp = HashMap::new();
        sum += calc(&mut dp, &unfolded_chars, &unfolded_nums, 0, 0, 0);
    }
    println!("{}", sum);
}

pub fn calc(dp: &mut HashMap<(usize, u64, usize), u64>, chars: &Vec<char>, nums: &Vec<u64>, curr: usize, num: u64, seqs: usize) -> u64 {
    if dp.contains_key(&(curr, num, seqs)) {
        return dp[&(curr, num, seqs)];
    }
    if curr == chars.len() {
        if seqs == nums.len() {
            dp.insert((curr, num, seqs), 1);
            return 1;
        } else {
            dp.insert((curr, num, seqs), 0);
            return 0;
        }
    } else if chars[curr] == '#' {
        let ret = calc(dp, chars, nums, curr + 1, num + 1, seqs);
        dp.insert((curr, num, seqs), ret);
        return ret;
    } else if chars[curr] == '.' || seqs == nums.len() {
        if seqs < nums.len() && num == nums[seqs] {
            let ret = calc(dp, chars, nums, curr + 1, 0, seqs + 1);
            dp.insert((curr, num, seqs), ret);
            return ret;
        } else if num == 0 {
            let ret = calc(dp, chars, nums, curr + 1, 0, seqs);
            dp.insert((curr, num, seqs), ret);
            return ret;
        } else {
            dp.insert((curr, num, seqs), 0);
            return 0;
        }
    } else {
        let hash = calc(dp, chars, nums, curr + 1, num + 1, seqs);
        let mut dot = 0;
        if num == nums[seqs] {
            dot = calc(dp, chars, nums, curr + 1, 0, seqs + 1);
        } else if num == 0 {
            dot = calc(dp, chars, nums, curr + 1, 0, seqs);
        }
        dp.insert((curr, num, seqs), hash + dot);
        return hash + dot;
    }
}