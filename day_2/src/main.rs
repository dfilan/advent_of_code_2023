use std::fs;
use std::io::{self, BufRead};
use std::path;

const BAG_AMOUNTS: [i32; 3] = [12, 13, 14];

fn main() {
    let answer = part_2("./input.txt");
    println!("{}", answer);
}

fn part_1(file_path: &str) -> u32 {
    let mut sum = 0;
    if let Ok(lines) = read_lines(file_path) {
        // parse each line
        // get vector of 3-tuples of counts by dividing by colons and semicolons
        // check if every element of every vector is less than bag_amounts
        for line in lines {
            if let Ok(line_content) = line {
                let (x, v) = parse_line(line_content);
                if v.iter().all(draw_is_ok) {
                    sum += x;
                }
            }
        }
    }
    sum
}

fn part_2(file_path: &str) -> i32 {
    let mut sum = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line_content) = line {
                let (_, v) = parse_line(line_content);
                sum += power(v);
            }
        }
    }
    sum
}

fn power(v: Vec<[i32; 3]>) -> i32 {
    let mut min_arr = [0, 0, 0];
    for arr in v.into_iter() {
        if arr[0] > min_arr[0] { min_arr[0] = arr[0]; }
        if arr[1] > min_arr[1] { min_arr[1] = arr[1]; }
        if arr[2] > min_arr[2] { min_arr[2] = arr[2]; }
    }
    min_arr[0] * min_arr[1] * min_arr[2]
}

fn draw_is_ok(arr: &[i32; 3]) -> bool {
    arr[0] <= BAG_AMOUNTS[0] && arr[1] <= BAG_AMOUNTS[1] && arr[2] <= BAG_AMOUNTS[2]
}

fn parse_line(line: String) -> (u32, Vec<[i32; 3]>) {
    let line_slice: &str = &line[..];
    let mut chunks = line_slice.split(&[':', ';']);
    let first_part = chunks.next().expect("The line should have stuff in it");
    let line_num: u32 = first_part
        .split(' ')
        .next_back()
        .expect("Lines should have at least one character")
        .parse()
        .expect("Lines should end with an integer");
    let mut draws: Vec<[i32; 3]> = Vec::new();
    for draw in chunks {
        let mut array = [0, 0, 0];
        let amounts = draw.split(',');
        for amount in amounts {
            let mut parts = amount.split(' ');
            // amount begins with a space, so the first part will be an empty string
            parts.next();
            let num: i32 = parts.next().unwrap().parse().unwrap();
            let colour = parts.next().unwrap();
            match colour {
                "red" => array[0] += num,
                "green" => array[1] += num,
                "blue" => array[2] += num,
                _ => (),
            };
            draws.push(array);
        }
    }
    (line_num, draws)
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
