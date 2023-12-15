use std::cmp::min;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// gah, too low

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let lines_chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let blocks: Vec<Vec<Vec<char>>> = lines_chars
        .split(|v| v.is_empty())
        .map(|v| v.to_vec())
        .collect();
    let mut accum = 0;
    for block in blocks.iter() {
        // println!("new block");
        let num_lines = block.len();
        let num_cols = block[0].len();
        let mut horizontal_reflection = false;
        for i in 0..(num_lines - 1) {
            let mut sum_reflect_errors = 0;
            let mut reflection_works = true;
            let lines_to_start = i + 1;
            let lines_to_end = num_lines - i - 1;
            for k in 0..min(lines_to_start, lines_to_end) {
                sum_reflect_errors += num_unequal(&block[i - k], &block[i + k + 1]);
                if sum_reflect_errors > 1 {
                    reflection_works = false;
                    break;
                }
            }
            if reflection_works && sum_reflect_errors == 1 {
                // println!("horizontal reflection worked, line {}", i);
                accum += (i + 1) * 100;
                horizontal_reflection = true;
                break;
            }
        }

        if !horizontal_reflection {
            for j in 0..(num_cols - 1) {
                let mut sum_reflect_errors = 0;
                let mut reflection_works = true;
                let cols_to_start = j + 1;
                let cols_to_end = num_cols - j - 1;
                for k in 0..min(cols_to_start, cols_to_end) {
                    let col_before: Vec<char> = (0..num_lines).map(|i| block[i][j - k]).collect();
                    let col_after: Vec<char> =
                        (0..num_lines).map(|i| block[i][j + k + 1]).collect();
                    sum_reflect_errors += num_unequal(&col_before, &col_after);
                    if sum_reflect_errors > 1 {
                        reflection_works = false;
                        break;
                    }
                }
                if reflection_works && sum_reflect_errors == 1 {
                    // println!("vertical reflection worked, column {}", j);
                    accum += j + 1;
                    break;
                }
            }
        }
    }
    accum
}

fn num_unequal<T>(v1: &[T], v2: &[T]) -> u64
where
    T: Eq,
{
    let mut num_neq = 0;
    for (i, x) in v1.iter().enumerate() {
        if *x != v2[i] {
            num_neq += 1;
        }
    }
    num_neq
}

// add up num cols to the left of each vertical line of reflection
// plus 100 * num rows above each horizontal line of reflection
// gotta divide lines into groups
// problem with naive alg is that it's quadratic
// OK, we're guaranteed that there's exactly one line of reflection

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let lines_chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let blocks: Vec<Vec<Vec<char>>> = lines_chars
        .split(|v| v.is_empty())
        .map(|v| v.to_vec())
        .collect();
    let mut accum = 0;
    for block in blocks.iter() {
        let num_lines = block.len();
        let num_cols = block[0].len();

        let mut horizontal_reflection = false;
        for i in 0..(num_lines - 1) {
            let mut reflection_works = true;
            let lines_to_start = i + 1;
            let lines_to_end = num_lines - i - 1;
            for k in 0..min(lines_to_start, lines_to_end) {
                if block[i - k] != block[i + k + 1] {
                    reflection_works = false;
                    break;
                }
            }
            if reflection_works {
                // println!("horizontal reflection worked, line {}", i);
                accum += (i + 1) * 100;
                horizontal_reflection = true;
                break;
            }
        }

        if !horizontal_reflection {
            for j in 0..(num_cols - 1) {
                let mut reflection_works = true;
                let cols_to_start = j + 1;
                let cols_to_end = num_cols - j - 1;
                for k in 0..min(cols_to_start, cols_to_end) {
                    let col_before = (0..num_lines).map(|i| block[i][j - k]);
                    let col_after = (0..num_lines).map(|i| block[i][j + k + 1]);
                    if col_before.ne(col_after) {
                        reflection_works = false;
                        break;
                    }
                }
                if reflection_works {
                    // println!("vertical reflection worked, column {}", j);
                    accum += j + 1;
                    break;
                }
            }
        }
    }
    accum
}
