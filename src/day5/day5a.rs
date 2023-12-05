use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day5/data/part_a.txt");
    let line_0: Vec<&str> = lines[0].split(' ').collect();
    let mut seeds: Vec<i64> = Vec::new();
    for seed in &line_0[1..] {
        seeds.push((**seed).parse::<i64>().unwrap());
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
    let mut loc = i64::MAX;
    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            for m in map {
                if val >= m[0] && val <= m[1] {
                    val = val + m[2];
                    break;
                }
            }
        }
        loc = loc.min(val);
    }
    println!("Location: {}", loc);

}