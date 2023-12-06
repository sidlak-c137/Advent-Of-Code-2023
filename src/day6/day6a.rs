
use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day6/data/part_a.txt");
    let mut times = Vec::new();
    let mut dists = Vec::new();

    let mut line = &lines[0];
    let mut split: Vec<&str> = line.split([':']).collect::<Vec<&str>>();
    let mut nums = split[1].split([' ']).collect::<Vec<&str>>();
    for c in nums {
        if c.trim().len() > 0 {
            let num = c.trim().parse::<i32>().unwrap();
            times.push(num);
        }
    }

    line = &lines[1];
    split = line.split([':']).collect::<Vec<&str>>();
    nums = split[1].split([' ']).collect::<Vec<&str>>();
    for c in nums {
        if c.trim().len() > 0 {
            let num = c.trim().parse::<i32>().unwrap();
            dists.push(num);
        }
    }
    let mut prod = 1.0;
    for i in 0..times.len() {
        let t = times[i];
        let d = dists[i];
        let a = quadratic_plus(1.0, -t as f32, d as f32);
        let b = quadratic_minus(1.0, -t as f32, d as f32);
        let val = (a - 1.0).ceil() - (b + 1.0).floor() + 1.0;
        prod *= val;
    }
    println!("Product: {}", prod);

}

pub fn quadratic_plus(a: f32, b: f32, c: f32) -> f32 {
   return (-b + f32::sqrt(0_f32.max(b * b - 4.0 * a * c))) / (2.0 * a);
}

pub fn quadratic_minus(a: f32, b: f32, c: f32) -> f32 {
    return  (-b - f32::sqrt(0_f32.max(b * b - 4.0 * a * c))) / (2.0 * a);
 }