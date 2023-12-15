use std::cmp;
use std::fs::read_to_string;

fn main() {
    let sum = part_2("./input.txt");
    println!("{}", sum);
}

fn part_2(file_path: &str) -> i32 {
    let line_vec = read_lines(file_path);
    let mut sum: i32 = 0;
    let num_lines = line_vec.len();
    for line_num in 0..num_lines {
        for (idx, c) in line_vec[line_num].chars().enumerate() {
            if c == '*' {
                let mut adjacent_nums: Vec<i32> = Vec::new();
                let this_line_nums = group_by(&line_vec[line_num], |c| c.is_ascii_digit())
                    .into_iter()
                    .filter(|(_, _, b)| *b);
                for (num_str, j, _) in this_line_nums {
                    let right_after = j > 0 && j - 1 == idx;
                    let right_before = j + num_str.len() == idx;
                    if right_after || right_before {
                        adjacent_nums.push(num_str.parse().unwrap());
                    }
                }
                if line_num > 0 {
                    let mut prev = adj_nums_in_line(&line_vec[line_num - 1], idx);
                    adjacent_nums.append(&mut prev);
                }
                if line_num < num_lines - 1 {
                    let mut next = adj_nums_in_line(&line_vec[line_num + 1], idx);
                    adjacent_nums.append(&mut next);
                }
                if adjacent_nums.len() == 2 {
                    // println!("line_num {}, index {}", line_num, idx);
                    sum += adjacent_nums.into_iter().product::<i32>();
                }
            }
        }
    }
    sum
}

fn adj_nums_in_line(line: &String, idx: usize) -> Vec<i32> {
    let this_line_nums = group_by(line, |c| c.is_ascii_digit())
        .into_iter()
        .filter(|(_, _, b)| *b);
    let mut adj_nums = Vec::new();
    for (num_str, j, _) in this_line_nums {
        let one_before = if j == 0 { j } else { j - 1 };
        if idx >= one_before && idx <= j + num_str.len() {
            adj_nums.push(num_str.parse().unwrap());
        }
    }
    adj_nums
}

// solution plan:
// read in each line
// chunk into numbers vs not
// for each number, know what index it starts and ends at
// check if previous line, this line, or next line has symbols adjacent to the number
// if so, add the number to the total

fn part_1(file_path: &str) -> i32 {
    let line_vec = read_lines(file_path);
    let mut sum = 0;
    let num_lines = line_vec.len();
    for i in 0..num_lines {
        let groups = group_by(&line_vec[i], |c| c.is_ascii_digit());
        let line_len = line_vec[i].len();
        for (group, start_idx, is_digit) in groups {
            // if it's digits, look around, turn it into a number, etc.
            if is_digit {
                let n: i32 = group.parse().unwrap();
                let glen = group.len();
                let this_line_chars: Vec<char> = line_vec[i].chars().collect();
                let one_before = if start_idx == 0 { 0 } else { start_idx - 1 };
                let goes_to_start = start_idx == 0;
                let one_after = cmp::min(start_idx + glen, line_len - 1);
                let goes_to_end = start_idx + glen == line_len;
                let before_special = !goes_to_start && this_line_chars[one_before] != '.';
                let after_special = !goes_to_end && this_line_chars[one_after] != '.';
                let prev_special =
                    i > 0 && has_special_char(&line_vec[i - 1][one_before..(one_after + 1)]);
                let next_special = i < num_lines - 1
                    && has_special_char(&line_vec[i + 1][one_before..(one_after + 1)]);
                let adjacent_to_special_char =
                    before_special || after_special || prev_special || next_special;
                if adjacent_to_special_char {
                    sum += n;
                }
            }
        }
    }
    sum
}

fn has_special_char(string_slice: &str) -> bool {
    !string_slice.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn group_by<F, T>(my_str: &String, predicate: F) -> Vec<(&str, usize, T)>
where
    F: Fn(char) -> T,
    T: Eq,
{
    // usize is the index of the first character

    // iterate thru the length of the string
    // start off at 0
    // get value of predicate where you are
    // go as far as you can until the value of pred changs
    // group all those together
    // then increment i
    let mut i = 0;
    let mut groups: Vec<(&str, usize, T)> = Vec::new();
    let chars: Vec<char> = my_str.chars().collect();
    while i < my_str.len() {
        let value = predicate(chars[i]);
        let mut j = i + 1;
        while j < my_str.len() {
            if predicate(chars[j]) != value {
                break;
            }
            j += 1;
        }
        groups.push((&my_str[i..j], i, value));
        i = j;
    }
    groups
}
