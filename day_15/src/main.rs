use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut boxes: HashMap<u64, Vec<(&str, u64)>> = HashMap::new();
    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }
    let mut accum = 0;
    for line in lines {
        for segment in line.split(',') {
            let parts: Vec<&str> = segment.split_inclusive(&['-', '=']).collect();
            let separator_idx = parts[0].len() - 1;
            let letters = &parts[0][..separator_idx];
            let hash = my_hash(letters);
            let my_box = boxes.get_mut(&hash).unwrap();
            if parts.len() == 1 {
                if let Some(i) = my_box.iter().position(|&(x, _)| x == letters) {
                    my_box.remove(i);
                }
            } else {
                let foc_len: u64 = parts[1].parse().unwrap();
                if let Some(i) = my_box.iter().position(|&(x, _)| x == letters) {
                    my_box[i] = (letters, foc_len);
                } else {
                    my_box.push((letters, foc_len));
                }
            }
        }
    }
    for i in 0..256 {
        let mut box_power = 0;
        let my_box = boxes.get(&i).unwrap();
        for (j, (_, foc_len)) in my_box.iter().enumerate() {
            box_power += ((j as u64) + 1) * foc_len;
        }
        box_power *= (i as u64) + 1;
        accum += box_power;
    }
    accum
}

fn my_hash(s: &str) -> u64 {
    let mut hash = 0;
    for c in s.as_bytes() {
        hash += *c as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

// ignore newline chars
// separate by commas

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut accum = 0;
    for line in lines {
        for segment in line.split(',') {
            let mut segment_hash: u64 = 0;
            for c in segment.as_bytes() {
                segment_hash += *c as u64;
                segment_hash *= 17;
                segment_hash %= 256;
            }
            accum += segment_hash;
        }
    }
    accum
}
