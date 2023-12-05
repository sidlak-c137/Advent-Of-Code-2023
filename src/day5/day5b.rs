use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day5/data/part_a.txt");
    let line_0: Vec<&str> = lines[0].split(' ').collect();
    let mut seeds: Vec<[i64; 2]> = Vec::new();
    for i in (1..line_0.len()).step_by(2) {
        let start = line_0[i].parse::<i64>().unwrap();
        let end = line_0[i + 1].parse::<i64>().unwrap();
        seeds.push([start, start + end - 1]);
    }
    let mut maps: Vec<Vec<[i64; 3]>> = Vec::new();
    for line in &lines[1..] {
        if line.len() == 0 {
            maps.push(Vec::new());
        } else if line.contains(":") {
            continue;
        } else {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let dst = parts[0].parse::<i64>().unwrap();
            let src = parts[1].parse::<i64>().unwrap();
            let range = parts[2].parse::<i64>().unwrap();
            maps.last_mut().unwrap().push([src, src + range - 1, dst - src]);
        }
    }
    for map in &mut maps {
        map.sort_by(|a, b| a[0].cmp(&b[0]));
    }
    let mut loc = i64::MAX;
    let mut vals = seeds.clone();
    for map in &maps {
        let mut new_vals: Vec<[i64; 2]> = Vec::new();
        intersect(&mut map.clone(), &mut vals, &mut new_vals);
        vals = new_vals;
    }
    for val in vals {
        loc = loc.min(val[0]);
    }
    println!("Location: {}", loc);
}

pub fn intersect(ranges: &mut Vec<[i64; 3]>, seeds: &mut Vec<[i64; 2]>, ret: &mut Vec<[i64; 2]>) {
    for seed in seeds {
        let mut min = seed[0];
        let max = seed[1];
        if min < ranges[0][0] && max < ranges[0][0] {
            ret.push([min, max]);
        } else if min < ranges[0][0] {
            ret.push([min, ranges[0][0] - 1]);
            min = ranges[0][0];
        }
        for range in &mut *ranges {
            if min >= range[0] && min <= range[1] {
                ret.push([min + range[2], max.min(range[1]) + range[2]]);
                min = range[1] + 1;
            }
            if max < min {
                break;
            }
        }
        if max >= min {
            ret.push([min, max]);
        }
    }
}