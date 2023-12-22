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

// 2525 is too low
// 180165 is too high

// make vec of bricks
// sort it by lowest z
// descend brick by brick
// use range arithmetic

type Coord = (u64, u64, u64);

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut brick_vec: Vec<(Coord, Coord)> = Vec::new();
    for line in lines {
        let mut parts = line.split('~');
        let first_part = parts.next().unwrap();
        let first_coord = str_to_coord(first_part);
        let second_part = parts.next().unwrap();
        let second_coord = str_to_coord(second_part);
        brick_vec.push((first_coord, second_coord));
    }
    brick_vec.sort_by_key(|(c1, _)| c1.2);
    // for each brick in brick vec:
    // find the highest placed brick that overlaps with you
    // end up one place higher
    let mut fallen_brick_vec = Vec::new();
    for brick in brick_vec.iter() {
        let mut earlier_overlapping = fallen_brick_vec
            .iter()
            .filter(|b| bricks_overlap(**b, *brick))
            .collect::<Vec<_>>();
        earlier_overlapping.sort_by_key(|(_, c2)| c2.2);
        let reduced_brick_height = brick.1 .2 - brick.0 .2;
        let first_z = match earlier_overlapping.pop() {
            None => 0,
            Some((_, c2)) => c2.2 + 1,
        };
        let second_z = first_z + reduced_brick_height;
        fallen_brick_vec.push((
            (brick.0 .0, brick.0 .1, first_z),
            (brick.1 .0, brick.1 .1, second_z),
        ));
    }
    fallen_brick_vec.sort_by_key(|(c1, _)| c1.2);
    let mut accum = 0;
    for (i, brick) in fallen_brick_vec.iter().enumerate() {
        // println!("looking at fallen brick {:?}", brick);
        // see how many other bricks rest on this brick
        // see what those bricks rest on
        let mut singleton = HashSet::new();
        singleton.insert(*brick);
        let brick_height = brick.1 .2;
        // let num_resting = tower_above(brick_height, brick_height, singleton, &fallen_brick_vec);
        let num_resting = tower_above(i, *brick, &fallen_brick_vec);
        // println!("set resting on that brick: {:?}", num_resting);
        accum += num_resting;
    }
    accum
}

fn tower_above(i: usize, brick: (Coord, Coord), fallen_brick_vec: &[(Coord, Coord)]) -> usize {
    let mut above_me: HashSet<(Coord, Coord)> = HashSet::new();
    above_me.insert(brick);
    for (j, brick_above) in fallen_brick_vec[(i + 1)..].iter().enumerate() {
        if rests_on_set(j + i + 1, *brick_above, &above_me, fallen_brick_vec) {
            above_me.insert(*brick_above);
        }
    }
    above_me.len() - 1
}

fn rests_on_set(
    i: usize,
    brick: (Coord, Coord),
    base_set: &HashSet<(Coord, Coord)>,
    fallen_brick_vec: &[(Coord, Coord)],
) -> bool {
    let support = fallen_brick_vec[..i]
        .iter()
        .filter(|b| b.1 .2 + 1 == brick.0 .2 && bricks_overlap(**b, brick));
    support.clone().all(|b| base_set.contains(b)) && support.count() > 0
}

// fn tower_above(
//     min_height: u64,
//     max_height: u64,
//     base_set: HashSet<(Coord, Coord)>,
//     fallen_brick_vec: &[(Coord, Coord)],
// ) -> usize {
//     // ) -> HashSet<(Coord, Coord)> {
//     // println!("calling tower_above with {:?}", base_set);
//     let only_resting_on_me = fallen_brick_vec
//         .iter()
//         .enumerate()
//         .take_while(|(_, b)| b.0 .2 <= max_height + 1)
//         .filter(|(_, b)| b.0 .2 > min_height)
//         .filter(|(n, b)| rests_on_set(*n, **b, &base_set, fallen_brick_vec))
//         .map(|(_, b)| *b)
//         .collect::<HashSet<(Coord, Coord)>>();
//     // set minus, not equal lengths
//     let nothing_new = base_set.is_superset(&only_resting_on_me);
//     if nothing_new {
//         base_set.len() - 1
//         // base_set
//     } else {
//         let mut next_set = base_set;
//         next_set.extend(only_resting_on_me);
//         let new_highest = next_set.clone().iter().map(|b| b.1 .2).max().unwrap();
//         tower_above(min_height, new_highest, next_set, fallen_brick_vec)
//     }
// }

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut brick_vec: Vec<(Coord, Coord)> = Vec::new();
    for line in lines {
        let mut parts = line.split('~');
        let first_part = parts.next().unwrap();
        let first_coord = str_to_coord(first_part);
        let second_part = parts.next().unwrap();
        let second_coord = str_to_coord(second_part);
        brick_vec.push((first_coord, second_coord));
    }
    brick_vec.sort_by_key(|(c1, _)| c1.2);
    // for each brick in brick vec:
    // find the highest placed brick that overlaps with you
    // end up one place higher
    let mut fallen_brick_vec = Vec::new();
    for brick in brick_vec.iter() {
        let mut earlier_overlapping = fallen_brick_vec
            .iter()
            .filter(|b| bricks_overlap(**b, *brick))
            .collect::<Vec<_>>();
        earlier_overlapping.sort_by_key(|(_, c2)| c2.2);
        let reduced_brick_height = brick.1 .2 - brick.0 .2;
        let first_z = match earlier_overlapping.pop() {
            None => 0,
            Some((_, c2)) => c2.2 + 1,
        };
        let second_z = first_z + reduced_brick_height;
        fallen_brick_vec.push((
            (brick.0 .0, brick.0 .1, first_z),
            (brick.1 .0, brick.1 .1, second_z),
        ));
    }
    fallen_brick_vec.sort_by_key(|(c1, _)| c1.2);
    let mut accum = 0;
    for (i, brick) in fallen_brick_vec.iter().enumerate() {
        // println!("looking at fallen brick {:?}", brick);
        // see how many other bricks rest on this brick
        // see what those bricks rest on
        let num_i_rest_on = |(j, &b): (usize, &(Coord, Coord))| {
            fallen_brick_vec[..j]
                .iter()
                .filter(|lower_b| lower_b.1 .2 + 1 == b.0 .2 && bricks_overlap(**lower_b, b))
                .count()
        };
        let num_only_resting = fallen_brick_vec[(i + 1)..]
            .iter()
            .enumerate()
            .filter(|(_, b)| brick.1 .2 + 1 == b.0 .2 && bricks_overlap(**b, *brick))
            .map(|(n, b)| num_i_rest_on((n + i + 1, b)))
            .filter(|n| *n == 1)
            .count();
        if num_only_resting == 0 {
            accum += 1;
        }
    }
    accum
}

fn bricks_overlap(b1: (Coord, Coord), b2: (Coord, Coord)) -> bool {
    // overlap if the xs and the ys overlap
    let xs_overlap = intervals_overlap((b1.0 .0, b1.1 .0), (b2.0 .0, b2.1 .0));
    let ys_overlap = intervals_overlap((b1.0 .1, b1.1 .1), (b2.0 .1, b2.1 .1));
    xs_overlap && ys_overlap
}

fn intervals_overlap(i1: (u64, u64), i2: (u64, u64)) -> bool {
    let first_within = i2.0 <= i1.0 && i2.1 >= i1.1;
    let first_overlaps_start = i1.0 <= i2.0 && i1.1 >= i2.0;
    let first_overlaps_end = i1.0 <= i2.1 && i1.1 >= i2.1;
    first_within || first_overlaps_start || first_overlaps_end
}

fn str_to_coord(coord_str: &str) -> Coord {
    let coord_iter = coord_str.split(',');
    let mut coord_vec = Vec::new();
    for num in coord_iter {
        coord_vec.push(num.parse::<u64>().unwrap());
    }
    (coord_vec[0], coord_vec[1], coord_vec[2])
}
