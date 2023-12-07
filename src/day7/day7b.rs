use std::collections::HashMap;

use crate::helper::lines_from_file;

pub fn solve() {
    let lines = lines_from_file("./src/day7/data/part_b.txt");
    let mut hands: HashMap<String, u32> = HashMap::new();
    let mut cards: Vec<String> = Vec::new();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        hands.insert(parts[0].to_string(), parts[1].parse::<u32>().unwrap());
        cards.push(parts[0].to_string());
    }
    cards.sort_by(compare_hands);
    let mut sum = 0;
    for i in 0..cards.len() {
        sum += (1 + i as u32) * hands.get(&cards[i]).unwrap();
    }
    println!("Sum: {}", sum);
}

pub fn compare_hands(a: &String, b: &String) -> std::cmp::Ordering {
    let a_val = get_hand_value(a);
    let b_val = get_hand_value(b);
    if a_val > b_val {
        return std::cmp::Ordering::Greater;
    } else if a_val < b_val {
        return std::cmp::Ordering::Less;
    } else {
        let mut cards = HashMap::new();
        let nums = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
        for i in 0..nums.len() {
            cards.insert(nums[i], i);
        }
        let na: Vec<char> = a.chars().collect();
        let nb: Vec<char> = b.chars().collect();
        for i in 0..cards.len() {
            let a_idx = cards.get(&na[i]).unwrap();
            let b_idx = cards.get(&nb[i]).unwrap();
            if a_idx > b_idx {
                return std::cmp::Ordering::Greater;
            } else if a_idx < b_idx {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

pub fn get_hand_value(hand: &String) -> u32 {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        if !freq.contains_key(&c) {
            freq.insert(c, 0);
        }
        freq.insert(c, freq.get(&c).unwrap() + 1);
    }
    let mut num_jokers = 0;
    if freq.contains_key(&'J') {
        num_jokers = freq.remove(&'J').unwrap();
    }

    if num_jokers > 0 && freq.len() != 0 {
        let mut max_char = 'J';
        let mut max_freq = 0;
        for (k, v) in &freq {
            if *v > max_freq {
                max_char = *k;
                max_freq = *v;
            }
        }
        freq.insert(max_char, freq.get(&max_char).unwrap() + num_jokers);
    } else if num_jokers > 0 {
        return 7;
    }


    let len = freq.len();
    if len == 1 {
        return 7;
    } else if len == 2 {
        for (_, v) in freq {
            if v == 4 {
                return 6;
            } else if v == 3 {
                return 5;
            } else if v == 2 {
                return 5;
            } else {
                return 6;
            }
        }
        return 0;
    } else if len == 3 {
        let mut num_twos = 0;
        for (_, v) in freq {
            if v == 2 {
                num_twos += 1;
            }
        }
        if num_twos == 2 {
            return 3;
        } else {
            return 4;
        }
    } else if len == 4 {
        return 2;
    } else {
        return 1;
    }
}