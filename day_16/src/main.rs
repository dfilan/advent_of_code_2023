use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// was originally caching recursive calls but that was a mistake

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut layout: Vec<Vec<char>> = Vec::new();
    for line in lines {
        layout.push(line.chars().collect());
    }
    let mut save_trees: HashMap<(usize, usize, Dir), HashSet<(usize, usize, Dir)>> = HashMap::new();
    let num_rows = layout.len();
    let num_cols = layout[0].len();
    let mut energized_set: HashSet<usize> = HashSet::new();
    for start_row in 0..num_rows {
        for d in [Dir::L, Dir::R] {
            energized_set.insert(count_energized(&layout, start_row, d, &mut save_trees));
        }
    }
    for start_col in 0..num_cols {
        for d in [Dir::U, Dir::D] {
            energized_set.insert(count_energized(&layout, start_col, d, &mut save_trees));
        }
    }
    *energized_set.iter().max().unwrap()
}

fn count_energized(
    layout: &Vec<Vec<char>>,
    start_num: usize,
    d: Dir,
    save_trees: &mut HashMap<(usize, usize, Dir), HashSet<(usize, usize, Dir)>>,
) -> usize {
    let start_r = if [Dir::L, Dir::R].contains(&d) {
        start_num
    } else if d == Dir::D {
        0
    } else {
        layout.len() - 1
    };
    let start_c = if [Dir::U, Dir::D].contains(&d) {
        start_num
    } else if d == Dir::R {
        0
    } else {
        layout[0].len() - 1
    };

    let mut traversed: HashSet<(usize, usize, Dir)> = HashSet::new();
    let mut current_points: Vec<(usize, usize, Dir)> = vec![(start_r, start_c, d)];
    while let Some((r, c, next_d)) = current_points.pop() {
        if !traversed.contains(&(r, c, next_d)) {
            let mut next = next_points(r, c, next_d, &layout);
            current_points.append(&mut next);
            traversed.insert((r, c, next_d));
        }
    }
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    for (r, c, _) in traversed.drain() {
        energized.insert((r, c));
    }
    energized.len()
}

fn get_subtree(
    r: usize,
    c: usize,
    d: Dir,
    layout: &Vec<Vec<char>>,
    save_trees: &mut HashMap<(usize, usize, Dir), HashSet<(usize, usize, Dir)>>,
) -> HashSet<(usize, usize, Dir)> {
    if let Some(s) = save_trees.get(&(r, c, d)) {
        s.clone()
    } else {
        let mut return_set: HashSet<(usize, usize, Dir)> = HashSet::new();
        // println!("r,c,d {}, {}, {:?}", r, c, d);
        let next = next_points(r, c, d, layout);
        for (next_r, next_c, next_d) in next {
            return_set.insert((next_r, next_c, next_d));
            let further_subtree = get_subtree(next_r, next_c, next_d, layout, save_trees);
            return_set.extend(&further_subtree);
        }
        save_trees.insert((r, c, d), return_set.clone());
        return_set
    }
}

// iterate thru grid
// add energized points to set
// follow paths iteratively
// maybe: if you're on the same point moving in the same direction, don't bother

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut layout: Vec<Vec<char>> = Vec::new();
    for line in lines {
        layout.push(line.chars().collect());
    }
    let mut traversed: HashSet<(usize, usize, Dir)> = HashSet::new();
    let mut current_points: Vec<(usize, usize, Dir)> = vec![(0, 0, Dir::R)];
    while let Some((r, c, d)) = current_points.pop() {
        if !traversed.contains(&(r, c, d)) {
            let mut next = next_points(r, c, d, &layout);
            current_points.append(&mut next);
            traversed.insert((r, c, d));
        }
    }
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    for (r, c, _) in traversed.drain() {
        energized.insert((r, c));
    }
    energized.len()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn next_points(
    curr_row: usize,
    curr_col: usize,
    d: Dir,
    layout: &Vec<Vec<char>>,
) -> Vec<(usize, usize, Dir)> {
    let curr_char = layout[curr_row][curr_col];
    match curr_char {
        '.' => progress_one(curr_row, curr_col, d, layout),
        '/' => match d {
            Dir::U => progress_one(curr_row, curr_col, Dir::R, layout),
            Dir::D => progress_one(curr_row, curr_col, Dir::L, layout),
            Dir::L => progress_one(curr_row, curr_col, Dir::D, layout),
            Dir::R => progress_one(curr_row, curr_col, Dir::U, layout),
        },
        '\\' => match d {
            Dir::U => progress_one(curr_row, curr_col, Dir::L, layout),
            Dir::D => progress_one(curr_row, curr_col, Dir::R, layout),
            Dir::L => progress_one(curr_row, curr_col, Dir::U, layout),
            Dir::R => progress_one(curr_row, curr_col, Dir::D, layout),
        },
        '|' => {
            if [Dir::L, Dir::R].contains(&d) {
                let mut up = progress_one(curr_row, curr_col, Dir::U, layout);
                let mut down = progress_one(curr_row, curr_col, Dir::D, layout);
                up.append(&mut down);
                up
            } else {
                progress_one(curr_row, curr_col, d, layout)
            }
        }
        '-' => {
            if [Dir::U, Dir::D].contains(&d) {
                let mut left = progress_one(curr_row, curr_col, Dir::L, layout);
                let mut right = progress_one(curr_row, curr_col, Dir::R, layout);
                left.append(&mut right);
                left
            } else {
                progress_one(curr_row, curr_col, d, layout)
            }
        }
        _ => Vec::new(),
    }
}

fn progress_one(
    curr_row: usize,
    curr_col: usize,
    d: Dir,
    layout: &Vec<Vec<char>>,
) -> Vec<(usize, usize, Dir)> {
    let num_rows = layout.len();
    let num_cols = layout[0].len();
    match d {
        Dir::U => {
            if curr_row > 0 {
                return vec![(curr_row - 1, curr_col, d)];
            }
        }
        Dir::D => {
            if curr_row < num_rows - 1 {
                return vec![(curr_row + 1, curr_col, d)];
            }
        }
        Dir::L => {
            if curr_col > 0 {
                return vec![(curr_row, curr_col - 1, d)];
            }
        }
        Dir::R => {
            if curr_col < num_cols - 1 {
                return vec![(curr_row, curr_col + 1, d)];
            }
        }
    };
    Vec::new()
}
