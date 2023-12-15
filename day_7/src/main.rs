use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let result = part_2("./input.txt");
    // let result = get_rank(&"K9J94");
    println!("result {}", result);
}

const CHARS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

// too low (?)

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut hand_vec: Vec<(&str, u64)> = Vec::new();
    for line in lines {
        let mut sections = line.split(' ');
        let hand = sections.next().unwrap();
        let bet: u64 = sections.next().unwrap().parse().unwrap();
        hand_vec.push((hand, bet));
    }
    hand_vec.sort_by(|(h1, _), (h2, _)| get_rank(h1).partial_cmp(&get_rank(h2)).unwrap());
    let mut total_winnings = 0;
    for (i, (h, b)) in hand_vec.into_iter().enumerate() {
        // if num_jacks(h) >= 0 {
        //     println!("hand {}", h);
        //     println!("value {}", get_rank(h));
        // }
        let j = (i as u64) + 1;
        total_winnings += b * j;
    }
    total_winnings
}

fn num_jacks(hand: &str) -> u64 {
    let hand_chars = hand.chars();
    let mut num_j = 0;
    for c in hand_chars {
        if c == 'J' { num_j += 1; }
    }
    num_j
}

fn get_rank(hand: &str) -> f64 {
    let hand_chars = hand.chars();
    let cloned_chars = hand_chars.clone();
    let mut count = HashMap::new();
    for c in &CHARS[..12] {
        count.insert(*c, 0);
    }
    let mut num_j: u64 = 0;
    for c in hand_chars {
        // println!("{}", c);
        if c == 'J' {
            num_j += 1;
        } else {
            count.entry(c).and_modify(|x| *x += 1);
        }
    }
    let mut count_vals: Vec<u64> = count.values().cloned().collect();
    count_vals.sort();
    let biggest = *count_vals.last().unwrap() + num_j;
    // println!("biggest {}", biggest);
    // println!("num_j {}", num_j);
    // for count in &count_vals {
    //     println!("{}", *count);
    // }
    let mut value: f64;
    if biggest == 5 {
        value = 6.0;
    } else if biggest == 4 {
        value = 5.0;
    } else if biggest == 3 {
        // println!("biggest is 3");
        // println!("next is {}", count_vals[11]);
        if count_vals[10] == 2 {
            value = 4.0;
        } else {
            value = 3.0;
        }
    } else if biggest == 2 {
        if count_vals[10] == 2 {
            value = 2.0;
        } else {
            value = 1.0;
        }
    } else {
        assert_eq!(biggest, 1);
        value = 0.0;
    }
    for (i, c) in cloned_chars.enumerate() {
        let thirteen: f64 = 13.0;
        let c_value = 12 - CHARS.iter().position(|&ch| ch == c).unwrap();
        value += (c_value as f64) / thirteen.powf((i + 1) as f64);
    }
    value
}

// five of a kind (same label) 6
// four of a kind 5
// full house 4
// three of a kind 3
// two pair 2
// one pair 1
// high card 0
// tie break by first card in each hand, then second, etc.

// hand bid
// win bid by rank, lowest is rank 1

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut hand_vec: Vec<(&str, u64)> = Vec::new();
    for line in lines {
        let mut sections = line.split(' ');
        let hand = sections.next().unwrap();
        let bet: u64 = sections.next().unwrap().parse().unwrap();
        hand_vec.push((hand, bet));
    }
    hand_vec.sort_by(|(h1, _), (h2, _)| get_rank_(h1).partial_cmp(&get_rank_(h2)).unwrap());
    let mut total_winnings = 0;
    for (i, (_, b)) in hand_vec.into_iter().enumerate() {
        let j = (i as u64) + 1;
        total_winnings += b * j;
    }
    total_winnings
}

fn get_rank_(hand: &str) -> f64 {
    let hand_chars = hand.chars();
    let cloned_chars = hand_chars.clone();
    let mut count = HashMap::new();
    for c in CHARS {
        count.insert(c, 0);
    }
    for c in hand_chars {
        count.entry(c).and_modify(|x| *x += 1);
    }
    let mut count_vals: Vec<u64> = count.values().cloned().collect();
    count_vals.sort();
    let biggest = *count_vals.last().unwrap();
    let mut value: f64;
    if biggest == 5 {
        value = 6.0;
    } else if biggest == 4 {
        value = 5.0;
    } else if biggest == 3 {
        if count_vals[11] == 2 {
            value = 4.0;
        } else {
            value = 3.0;
        }
    } else if biggest == 2 {
        if count_vals[11] == 2 {
            value = 2.0;
        } else {
            value = 1.0;
        }
    } else {
        assert_eq!(biggest, 1);
        value = 0.0;
    }
    for (i, c) in cloned_chars.enumerate() {
        let thirteen: f64 = 13.0;
        let c_value = 12 - CHARS.iter().position(|&ch| ch == c).unwrap();
        value += (c_value as f64) / thirteen.powf((i + 1) as f64);
    }
    value
}
