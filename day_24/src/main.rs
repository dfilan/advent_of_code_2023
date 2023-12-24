use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // let lower_bound = 200000000000000.0;
    // // let lower_bound = 7.0;
    // // let upper_bound = 27.0;
    // let upper_bound = 400000000000000.0;
    // let result = part_1(lower_bound, upper_bound, "./input.txt");
    let result = part_2("./test.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// want to find a time when all of the paths are collinear
// then project down to the ground (nope, to time 0)
// no, that's not right - it's moving at integer speed
// hmmm - i think there might be only one line that passes thru all the lines?
// so note that once you have two positions and two times, that defines a trajectory
// and because I have velocity info, that just needs two positions
// is this one of those things where I need to think in 4d?

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut traj_vec: Vec<((f64, f64, f64), (f64, f64, f64))> = Vec::new();
    for line in lines {
        let mut parts = line.split(" @ ");
        let first_part = parts.next().unwrap();
        let mut coords = first_part.split(", ");
        let x_coord = coords.next().unwrap().parse::<f64>().unwrap();
        let y_coord = coords.next().unwrap().parse::<f64>().unwrap();
        let z_coord = coords.next().unwrap().parse::<f64>().unwrap();
        let next_part = parts.next().unwrap();
        // println!("next part: {}", next_part.clone());
        let mut vels = next_part.split(", ");
        let x_vel = vels.next().unwrap().trim().parse::<f64>().unwrap();
        let y_vel = vels.next().unwrap().trim().parse::<f64>().unwrap();
        let z_vel = vels.next().unwrap().trim().parse::<f64>().unwrap();
        // println!("y_vel: {}", y_vel);
        traj_vec.push((x_coord, y_coord, z_coord), (x_vel, y_vel, z_vel));
        // pos_vel_vec.push((coord_tup, vels_tup));
    }
    let mut accum = 0;
    let num_lines = traj_vec.len();
    for i in 0..num_lines {
        let (mi, bi, x0i, i_forward) = traj_vec[i];
        for &(mj, bj, x0j, j_forward) in &traj_vec[(i + 1)..] {
            if mi != mj {
                let intersect_x = (bj - bi) / (mi - mj);
                if intersect_x >= lower_bound && intersect_x <= upper_bound {
                    let intersect_future_i = (intersect_x > x0i) == i_forward;
                    let intersect_future_j = (intersect_x > x0j) == j_forward;
                    let intersect_future = intersect_future_i && intersect_future_j;
                    let intersect_y = mi * intersect_x + bi;
                    if intersect_future && intersect_y >= lower_bound && intersect_y <= upper_bound
                    {
                        accum += 1
                    }
                }
            }
        }
    }
    accum
}


fn part_1(lower_bound: f64, upper_bound: f64, file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    // entries are (m, b, x0, forward_in_future)
    let mut traj_vec: Vec<(f64, f64, f64, bool)> = Vec::new();
    for line in lines {
        let mut parts = line.split(" @ ");
        let first_part = parts.next().unwrap();
        let mut coords = first_part.split(", ");
        let x_coord = coords.next().unwrap().parse::<f64>().unwrap();
        let y_coord = coords.next().unwrap().parse::<f64>().unwrap();
        let next_part = parts.next().unwrap();
        // println!("next part: {}", next_part.clone());
        let mut vels = next_part.split(", ");
        let x_vel = vels.next().unwrap().trim().parse::<f64>().unwrap();
        let y_vel = vels.next().unwrap().trim().parse::<f64>().unwrap();
        // println!("y_vel: {}", y_vel);
        let m = y_vel / x_vel;
        let b = y_coord - x_coord * m;
        traj_vec.push((m, b, x_coord, x_vel > 0.0));
        // pos_vel_vec.push((coord_tup, vels_tup));
    }
    let mut accum = 0;
    let num_lines = traj_vec.len();
    for i in 0..num_lines {
        let (mi, bi, x0i, i_forward) = traj_vec[i];
        for &(mj, bj, x0j, j_forward) in &traj_vec[(i + 1)..] {
            if mi != mj {
                let intersect_x = (bj - bi) / (mi - mj);
                if intersect_x >= lower_bound && intersect_x <= upper_bound {
                    let intersect_future_i = (intersect_x > x0i) == i_forward;
                    let intersect_future_j = (intersect_x > x0j) == j_forward;
                    let intersect_future = intersect_future_i && intersect_future_j;
                    let intersect_y = mi * intersect_x + bi;
                    if intersect_future && intersect_y >= lower_bound && intersect_y <= upper_bound
                    {
                        accum += 1
                    }
                }
            }
        }
    }
    accum
}
