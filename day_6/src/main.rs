use std::fs::read_to_string;
use std::iter::zip;

fn main() {
    let result = part_2("./input.txt");
    println!("result {}", result);
}

fn part_2(file_path: &str) -> f64 {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    let times_line = lines.next().unwrap();
    let mut times = times_line.split_whitespace();
    times.next();
    let time_string = times.collect::<Vec<&str>>().join("");
    let dists_line = lines.next().unwrap();
    let mut dists = dists_line.split_whitespace();
    dists.next();
    let dists_string = dists.collect::<Vec<&str>>().join("");
    let total_time: u64 = time_string.parse().unwrap();
    let dist: u64 = dists_string.parse().unwrap();
    let determinant = ((total_time as f64) / 2.0).powf(2.0) - (dist as f64);
    let threshold_time = (total_time as f64) / 2.0 - determinant.sqrt();
    ((total_time as f64) / 2.0 - (threshold_time + 1.0).floor()) * 2.0 + 1.0
}


// want to go farther in each race than current record holder
// holding button down for n milliseconds gives speed n mm / ms
// can only hold button down for integer number of milliseconds
// per race, want tnumber of ways you can beat the record
// multiply values
// easy way: for each race, iterate thru number of milliseconds you could hold down for

fn part_1(file_path: &str) -> f64 {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    let times_line = lines.next().unwrap();
    let mut times = times_line.split_whitespace();
    times.next();
    let dists_line = lines.next().unwrap();
    let mut dists = dists_line.split_whitespace();
    dists.next();
    let mut accum = 1.0;
    for (t, d) in zip(times, dists) {
        let total_time: u64 = t.parse().unwrap();
        let dist: u64 = d.parse().unwrap();
        let determinant = ((total_time as f64) / 2.0).powf(2.0) - (dist as f64);
        let threshold_time = (total_time as f64) / 2.0 - determinant.sqrt();
        let num_times = ((total_time as f64) / 2.0 - (threshold_time + 1.0).floor()) * 2.0 + 1.0;
        accum *= num_times;
    }
    accum
}
