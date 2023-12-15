use std::fs::read_to_string;

fn main() {
    let result = part_1("./input.txt");
    println!("result {}", result);
}

// extrapolate values, get sum

fn part_2(file_path: &str) -> i64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut accum = 0;
    let minus_one: i64 = -1;
    for line in lines {
        let num_strs = line.split(' ');
        let mut nums: Vec<i64> = num_strs.map(|s| s.parse().unwrap()).collect();
        let mut first_elements: Vec<i64> = vec![*nums.clone().first().unwrap()];
        while some_non_zero(&nums) {
            let mut new_nums: Vec<i64> = Vec::new();
            let mut current = nums[0];
            for elt in &nums[1..] {
                new_nums.push(elt - current);
                current = *elt;
            }
            first_elements.push(*new_nums.first().unwrap());
            nums = new_nums;
        }
        for (i, num) in first_elements.into_iter().enumerate() {
            accum += num * minus_one.pow(i as u32);
        }
    }
    accum
}

fn part_1(file_path: &str) -> i64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut accum = 0;
    for line in lines {
        let num_strs = line.split(' ');
        let mut nums: Vec<i64> = num_strs.map(|s| s.parse().unwrap()).collect();
        let mut last_elements: Vec<i64> = vec![*nums.clone().last().unwrap()];
        while some_non_zero(&nums) {
            let mut new_nums: Vec<i64> = Vec::new();
            let mut current = nums[0];
            for elt in &nums[1..] {
                new_nums.push(elt - current);
                current = *elt;
            }
            last_elements.push(*new_nums.last().unwrap());
            nums = new_nums;
        }
        for num in last_elements {
            accum += num;
        }
    }
    accum
}

fn some_non_zero(v: &Vec<i64>) -> bool {
    for elt in v {
        if *elt != 0 {
            return true;
        }
    }
    false
}
