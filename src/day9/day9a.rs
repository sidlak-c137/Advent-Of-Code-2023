use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day9/data/part_a.txt");
    let mut sum: i32 = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let mut tree = Vec::new();
        let mut curr = Vec::new();
        for part in parts {
            let num = part.parse::<i32>().unwrap();
            curr.push(num);
        }
        tree.push(curr);

        loop {
            let mut next = Vec::new();
            let mut num_zeros = 0;
            for i in 0..tree[tree.len() - 1].len() - 1 {
                let num = tree[tree.len() - 1][i + 1] - tree[tree.len() - 1][i];
                next.push(num);
                if num == 0 {
                    num_zeros += 1;
                }
            }
            tree.push(next);
            if num_zeros == tree[tree.len() - 1].len() {
                break;
            }
        }
        let mut val = 0;
        for i in (1..tree.len()).rev() {
            let layer_up = tree[i - 1][tree[i - 1].len() - 1];
            val = layer_up + val;
        }
        sum += val;
    }
    println!("Sum: {}", sum);
}