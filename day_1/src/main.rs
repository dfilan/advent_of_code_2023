use std::fs;
use std::io::{self, BufRead};
use std::path;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let num = part_2("./input.txt");
    println!("{:?}", num);
}

// how to solve this problem
// take in the file
// read each line
// get the first number digit by reading the line from the front
// get the last number digit by reading the line from the back
// multiply first digit by 10, add second digit
// add that to the accumulator

fn part_1(file_path: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(file_path) {
        let mut sum = 0;
        for (counter, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                // println!("{:?}", line_content);
                let mut numbers = line_content.chars().filter(|c| c.is_ascii_digit());
                if let Some(x) = numbers.next() {
                    let first_digit = x.to_digit(10).expect("I filtered for digits");
                    let last_digit = match numbers.next_back() {
                        None => x.to_digit(10).expect("I filtered for digits"),
                        Some(y) => y.to_digit(10).expect("I filtered for digits"),
                    };
                    sum += 10 * first_digit + last_digit;
                    // println!("{:?}", first_digit);
                    // println!("{:?}", last_digit);
                } else {
                    return None;
                }
            }
            // if counter > 3 {
            //     break;
            // }
        }
        return Some(sum);
    }
    None
}

fn part_2(file_path: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(file_path) {
        let mut sum = 0;
        for (counter, line) in lines.enumerate() {
            if let Ok(line_content) = line {
                // println!("{:?}", line_content);
                // keep going forward until last segment is either a real digit or in DIGITS.
                // similarly the other way.
                let l = line_content.len();
                let mut first_digit = 20;
                for n in 1..(l + 1) {
                    let init_segment = &line_content[..n];
                    // println!("{}", init_segment);
                    if init_segment.as_bytes()[n - 1].is_ascii_digit() {
                        first_digit = (init_segment.as_bytes()[n - 1] as char)
                            .to_digit(10)
                            .expect("I said it was an ascii digit");
                    } else {
                        for (num, digit) in DIGITS.into_iter().enumerate() {
                            let ld = digit.len();
                            if n >= ld && &init_segment[(n - ld)..n] == digit {
                                first_digit = (num as u32) + 1;
                            }
                        }
                    }
                    if first_digit != 20 {
                        break;
                    }
                }
                if first_digit == 20 {
                    return None;
                }
                let mut last_digit = 20;
                for n in (0..l).rev() {
                    let final_segment = &line_content[n..];
                    // println!("{}", final_segment);
                    if final_segment.as_bytes()[0].is_ascii_digit() {
                        last_digit = (final_segment.as_bytes()[0] as char)
                            .to_digit(10)
                            .expect("I said it was an ascii digit");
                    } else {
                        for (num, digit) in DIGITS.into_iter().enumerate() {
                            let ld = digit.len();
                            if l >= n + ld && &final_segment[..ld] == digit {
                                last_digit = (num as u32) + 1;
                            }
                        }
                    }
                    if last_digit != 20 {
                        break;
                    }
                }
                if last_digit == 20 {
                    return None;
                }
                // println!("{:}", first_digit);
                // println!("{:}", last_digit);
                sum += 10 * first_digit + last_digit
            }
            // if counter > 3 {
            //     break;
            // }
        }
        return Some(sum);
    }
    None
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
