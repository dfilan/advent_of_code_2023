use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

// I'm told I should actually do this with btrees instead
// I tried that: with btreesets for add_rows and add_cols, the code takes 140-200 ms to run on the
// main input
// but with vecs, it takes 70-80 ms.

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{}", result);
    let elapsed = start.elapsed();
    println!("Time {:?}", elapsed);
}

// record which rows are expanded
// if galaxies contain rows or cols that are expanded, add 1 mill - 1 to dist
// code takes suspiciously long
// ideally would do set difference thing
// but I've got to take these diffs anyway...

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let universe: Vec<Vec<char>> = file_string.lines().map(|l| l.chars().collect()).collect();
    let mut add_rows: Vec<usize> = Vec::new();
    let mut add_cols: Vec<usize> = Vec::new();
    let dilation_factor: usize = 1_000_000;
    for (i, row) in universe.iter().enumerate() {
        if !row.contains(&'#') {
            add_rows.push(i);
        }
    }
    for j in 0..(universe[0].len()) {
        let any_galaxies = universe.iter().any(|r| r[j] == '#');
        if !any_galaxies {
            add_cols.push(j);
        }
    }
    // find galaxies in universe
    let mut galaxy_coords: Vec<(usize, usize)> = Vec::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_coords.push((i, j));
            }
        }
    }
    // now loop over pairs of galaxies
    let mut sum_dists = 0;
    for (idx, (g1i, g1j)) in galaxy_coords.iter().enumerate() {
        for (g2i, g2j) in &galaxy_coords[(idx + 1)..] {
            let min_i = min(*g1i, *g2i);
            let max_i = max(*g1i, *g2i);
            let min_j = min(*g1j, *g2j);
            let max_j = max(*g1j, *g2j);
            let mut i_diff = max_i - min_i;
            let num_rows_between = add_rows
                .iter()
                .filter(|r| **r > min_i)
                .take_while(|r| **r < max_i)
                .collect::<Vec<_>>()
                .len();
            i_diff += num_rows_between * (dilation_factor - 1);
            let mut j_diff = max_j - min_j;
            let num_cols_between = add_cols
                .iter()
                .filter(|c| **c > min_j)
                .take_while(|c| **c < max_j)
                .collect::<Vec<_>>()
                .len();
            j_diff += num_cols_between * (dilation_factor - 1);
            sum_dists += i_diff + j_diff;
        }
    }
    sum_dists
}

// sum of lengths of shortest paths
// manhattan distance
// rows or columns without galaxies get doubled

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let universe: Vec<Vec<char>> = file_string.lines().map(|l| l.chars().collect()).collect();
    let mut add_rows: HashSet<usize> = HashSet::new();
    let mut add_cols: HashSet<usize> = HashSet::new();
    for (i, row) in universe.iter().enumerate() {
        if !row.contains(&'#') {
            add_rows.insert(i);
        }
    }
    for j in 0..(universe[0].len()) {
        let any_galaxies = universe.iter().any(|r| r[j] == '#');
        if !any_galaxies {
            add_cols.insert(j);
        }
    }
    let mut big_universe: Vec<Vec<char>> = Vec::new();
    for i in 0..(universe.len()) {
        big_universe.push(Vec::new());
        for j in 0..(universe[0].len()) {
            big_universe.last_mut().unwrap().push(universe[i][j]);
            if add_cols.contains(&j) {
                big_universe.last_mut().unwrap().push(universe[i][j]);
            }
        }
        if add_rows.contains(&i) {
            big_universe.push(big_universe.last().unwrap().clone());
        }
    }
    // find galaxies in big universe
    let mut galaxy_coords: Vec<(usize, usize)> = Vec::new();
    for (i, row) in big_universe.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_coords.push((i, j));
            }
        }
    }
    // now loop over pairs of galaxies
    let mut sum_dists = 0;
    for (idx, (g1i, g1j)) in galaxy_coords.iter().enumerate() {
        for (g2i, g2j) in &galaxy_coords[(idx + 1)..] {
            let i_diff = g1i.abs_diff(*g2i);
            let j_diff = g1j.abs_diff(*g2j);
            sum_dists += i_diff + j_diff;
        }
    }
    sum_dists
}
