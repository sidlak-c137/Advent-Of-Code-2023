use std::collections::BTreeSet;
use std::ops::Bound::Excluded;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day11/data/part_b.txt");
    let mut space = Vec::new();
    for line in lines {
        space.push(line.chars().collect::<Vec<char>>());
    }
    let mut i = 0;
    let mut rows = BTreeSet::new();
    let expansion = 1000000;
    while i < space.len() {
        let mut galaxy = 0;
        for c in &space[i] {
            if *c == '#' {
                galaxy += 1;
            }
        }
        if galaxy == 0 {
            rows.insert(i as u64);
        }
        i += 1;
    }
    let mut j = 0;
    let mut cols = BTreeSet::new();
    while j < space[0].len() {
        let mut galaxy = 0;
        for i in 0..space.len() {
            if space[i][j] == '#' {
                galaxy += 1;
            }
        }
        if galaxy == 0 {
            cols.insert(j as u64);
        }
        j += 1;
    }

    let mut coords = Vec::new();
    for i in 0..space.len() {
        for j in 0..space[i].len() {
            if space[i][j] == '#' {
                coords.push((i as u64, j as u64));
            }
        }
    }
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let mut dist = ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u64;
            if x1 > x2 {
                rows.range((Excluded(&x2), Excluded(&x1))).for_each(|_| dist += expansion - 1);
            } else if x1 < x2{
                rows.range((Excluded(&x1), Excluded(&x2))).for_each(|_| dist += expansion - 1);
            }
            if y1 > y2 {
                cols.range((Excluded(&y2), Excluded(&y1))).for_each(|_| dist += expansion - 1);
            } else if y1 < y2 {
                cols.range((Excluded(&y1), Excluded(&y2))).for_each(|_| dist += expansion - 1);
            }
            sum += dist;
        }
    }
    println!("Sum: {}", sum);
}