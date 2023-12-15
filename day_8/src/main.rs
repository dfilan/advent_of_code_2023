use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let result = part_2("./input.txt");
    println!("result {}", result);
}

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    let dir_chars: Vec<char> = lines.next().unwrap().chars().collect();
    let mut start_states: Vec<&str> = Vec::new();
    lines.next();
    let mut map = HashMap::new();
    for line in lines {
        let mut sections = line.split(' ');
        let source = sections.next().unwrap();
        if source.ends_with('A') {
            start_states.push(source);
        }
        sections.next();
        let right_path = &sections.next().unwrap()[1..4];
        let left_path = &sections.next().unwrap()[..3];
        map.insert(source, (right_path, left_path));
    }
    let mut times: Vec<u64> = vec![0; start_states.len()];
    for (i, state) in start_states.into_iter().enumerate() {
        let mut num_steps = 0;
        let mut current_state = state;
        while current_state.chars().last().unwrap() != 'Z' {
            let dir = dir_chars[num_steps % dir_chars.len()];
            let fork = map.get(current_state).unwrap();
            if dir == 'L' {
                current_state = fork.0;
            } else if dir == 'R' {
                current_state = fork.1;
            }
            num_steps += 1;
        }
        times[i] = num_steps as u64;
    }
    lcm_vec(times)
}

fn lcm_vec(n_vec: Vec<u64>) -> u64 {
    n_vec.into_iter().fold(1, lcm)
}

fn lcm (n: u64, m: u64) -> u64 {
    n * (m / gcd(n, m))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            let z = m;
            m = n;
            n = z;
        }
        m %= n;
    }
    n
}

// how many steps to reach ZZZ from AAA, sticking with the path
// do it the dumb way?

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    let dir_chars: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map = HashMap::new();
    for line in lines {
        let mut sections = line.split(' ');
        let source = sections.next().unwrap();
        sections.next();
        let right_path = &sections.next().unwrap()[1..4];
        let left_path = &sections.next().unwrap()[..3];
        map.insert(source, (right_path, left_path));
    }
    let mut num_steps = 0;
    let mut current_state = "AAA";
    while current_state != "ZZZ" {
        let dir = dir_chars[num_steps % dir_chars.len()];
        let fork = map.get(current_state).unwrap();
        if dir == 'L' {
            current_state = fork.0;
        } else if dir == 'R' {
            current_state = fork.1;
        }
        num_steps += 1;
    }
    num_steps
}
