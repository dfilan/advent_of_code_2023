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

// OK: curve is way too big to do our standard thing
// so: divide world into rectangles

fn part_2(file_path: &str) -> i64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut current_point: (i64, i64, Dir) = (0, 0, Dir::D);
    let mut curve_segments: Vec<(i64, i64, u64, Dir)> = Vec::new();
    let mut num_quarter_turns: i64 = 0;
    // TODO: add thing where I track quarter turns, check which way is inside at each stretch
    for line in lines {
        let mut char_groups = line.split(' ');
        char_groups.next();
        char_groups.next();
        let (dir, length) = get_dir_len(char_groups.next().unwrap());
        if dir == clockwise_dir(&current_point.2) {
            num_quarter_turns += 1;
        } else {
            num_quarter_turns -= 1;
        }
        curve_segments.push((current_point.0, current_point.1, length, dir));
        current_point = go_steps_in_dir(current_point, length, dir);
    }
    let mut vertical_segments: Vec<i64> = curve_segments
        .iter()
        .filter(|(_, _, _, d)| [Dir::U, Dir::D].contains(d))
        .map(|entry| entry.1)
        .collect();
    vertical_segments.sort();
    let mut horizontal_segments: Vec<i64> = curve_segments
        .iter()
        .filter(|(_, _, _, d)| [Dir::L, Dir::R].contains(d))
        .map(|entry| entry.0)
        .collect();
    horizontal_segments.sort();
    // get rectangles
    let mut rectangles: Vec<(i64, i64, i64, i64)> = Vec::new();
    for i in 0..(horizontal_segments.len() - 1) {
        for j in 0..(vertical_segments.len() - 1) {
            rectangles.push((
                horizontal_segments[j],
                horizontal_segments[j + 1],
                vertical_segments[i],
                vertical_segments[i + 1],
            ));
        }
    }
    // in area, include the left and top of every rectangle, and the bits of the curve
    // on the right and the bottom
    let inside_rectangles_area: i64 = rectangles
        .iter()
        .filter(|rect| is_inside(rect, &curve_segments, num_quarter_turns))
        .map(|(r1, r2, c1, c2)| (r2 - r1) * (c2 - c1))
        .sum();
    let bottom_right_length: u64 = curve_segments
        .iter()
        .filter(|(_, _, _, d)| on_bottom_right(d, num_quarter_turns))
        .map(|entry| entry.2)
        .sum();
    println!("inside rectangles area {}", inside_rectangles_area);
    println!("bottom_right_length {}", bottom_right_length);
    inside_rectangles_area + (bottom_right_length as i64) + 1
}

fn go_steps_in_dir(point: (i64, i64, Dir), steps: u64, dir: Dir) -> (i64, i64, Dir) {
    match dir {
        Dir::U => (point.0 - (steps as i64), point.1, dir),
        Dir::D => (point.0 + (steps as i64), point.1, dir),
        Dir::L => (point.0, point.1 - (steps as i64), dir),
        Dir::R => (point.0, point.1 + (steps as i64), dir),
    }
}

fn is_inside(
    rect: &(i64, i64, i64, i64),
    curve_segments: &[(i64, i64, u64, Dir)],
    num_quarter_turns: i64,
) -> bool {
    // go up, see if the first border you find has its inside facing down
    // println!("\nrect: {:?}", *rect);
    let overlaps = |segment: &(i64, i64, u64, Dir)| {
        if segment.3 == Dir::L {
            rect.2 >= segment.1 - (segment.2 as i64) && rect.3 <= segment.1
        } else {
            rect.2 >= segment.1 && rect.3 <= segment.1 + (segment.2 as i64)
        }
    };
    let all_lr_segments = curve_segments
        .iter()
        .filter(|entry| [Dir::L, Dir::R].contains(&entry.3));
    // for seg in all_lr_segments {
    //     println!("seg: {:?}", seg);
    //     println!("rect.2 {:?}", rect.2);
    //     println!("segment.1 {:?}", seg.1);
    //     if seg.3 == Dir::L {
    //         println!("segment.1 - segment.2 {:?}", seg.1 - (seg.2 as i64));
    //     } else {
    //         println!("segment.1 + segment.2 {:?}", seg.1 + (seg.2 as i64))
    //     }
    //     println!("overlaps? {:?}", overlaps(seg));
    // }
    let mut lr_segments: Vec<(i64, i64, u64, Dir)> = curve_segments
        .iter()
        .filter(|entry| [Dir::L, Dir::R].contains(&entry.3) && entry.0 <= rect.0 && overlaps(entry))
        .copied()
        .collect::<Vec<_>>();
    if !lr_segments.is_empty() {
        lr_segments.sort_by_key(|e| e.0);
        // can check facing down with on_bottom_right
        let first_segment = lr_segments.pop().unwrap();
        // println!("first segment above: {:?}", first_segment);
        // println!("is it on the bottom right? {}", on_bottom_right(&first_segment.3, num_quarter_turns));
        !on_bottom_right(&first_segment.3, num_quarter_turns)
    } else {
        // println!("nothing's above this rect");
        false
    }
}

fn on_bottom_right(d: &Dir, num_quarter_turns: i64) -> bool {
    if num_quarter_turns > 0 {
        // we've turned clockwise
        [Dir::L, Dir::D].contains(d)
    } else {
        [Dir::R, Dir::U].contains(d)
    }
}

fn get_dir_len(hex_string: &str) -> (Dir, u64) {
    let len_string = &hex_string[2..7];
    let len = u64::from_str_radix(len_string, 16).unwrap();
    let mut chars_vec = hex_string.chars().collect::<Vec<_>>();
    chars_vec.pop();
    let final_char = chars_vec.pop();
    let dir = match final_char {
        Some('0') => Dir::R,
        Some('1') => Dir::D,
        Some('2') => Dir::L,
        _ => Dir::U,
    };
    (dir, len)
}

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut current_point: (i64, i64, Dir) = (0, 0, Dir::D);
    let mut points_on_curve: Vec<(i64, i64, Dir, Dir)> = Vec::new();
    // second dir entry is next direction you're going
    let mut num_quarter_turns: i64 = 0;
    for line in lines {
        let mut char_groups = line.split(' ');
        let dir = str_to_dir(char_groups.next().unwrap());
        let curve_len = points_on_curve.len();
        if curve_len > 0 {
            points_on_curve[curve_len - 1].3 = dir;
        }
        let length: u64 = char_groups.next().unwrap().parse().unwrap();
        if dir == clockwise_dir(&current_point.2) {
            num_quarter_turns += 1;
        } else {
            num_quarter_turns -= 1;
        }
        for _ in 0..length {
            current_point = step_in_dir(current_point, dir);
            points_on_curve.push((
                current_point.0,
                current_point.1,
                current_point.2,
                current_point.2,
            ));
        }
    }
    let mut in_curve_set: HashSet<(i64, i64)> = HashSet::new();
    let on_curve_set = HashSet::from_iter(points_on_curve.iter().map(|&(r, c, _, _)| (r, c)));
    for (r, c, d, next_d) in points_on_curve.iter() {
        let in_dirs = get_in_dir(num_quarter_turns, d, next_d);
        for in_dir in in_dirs {
            accum_in_dir(*r, *c, in_dir, &mut in_curve_set, &on_curve_set);
        }
    }
    in_curve_set.len() + points_on_curve.len()
}

fn accum_in_dir(
    r: i64,
    c: i64,
    in_dir: Dir,
    in_set: &mut HashSet<(i64, i64)>,
    on_curve_set: &HashSet<(i64, i64)>,
) {
    let mut current_point = step_in_dir((r, c, in_dir), in_dir);
    let mut current_rc = (current_point.0, current_point.1);
    while !on_curve_set.contains(&current_rc) {
        in_set.insert(current_rc);
        current_point = step_in_dir(current_point, in_dir);
        current_rc = (current_point.0, current_point.1);
    }
}

fn get_in_dir(num_quarter_turns: i64, d: &Dir, next_d: &Dir) -> Vec<Dir> {
    if num_quarter_turns > 0 {
        // curve goes clockwise
        // get stuff in clockwise direction
        if *d == *next_d {
            // you're going straight
            vec![clockwise_dir(d)]
        } else if *next_d == clockwise_dir(d) {
            // kink is on outside
            Vec::new()
        } else {
            // kink is on inside
            vec![*d, opposite_dir(next_d)]
        }
    } else if *d == *next_d {
        vec![opposite_dir(&clockwise_dir(d))]
    } else if *next_d == clockwise_dir(d) {
        vec![*d, opposite_dir(next_d)]
    } else {
        Vec::new()
    }
}

fn clockwise_dir(d: &Dir) -> Dir {
    match *d {
        Dir::U => Dir::R,
        Dir::D => Dir::L,
        Dir::L => Dir::U,
        Dir::R => Dir::D,
    }
}

fn opposite_dir(d: &Dir) -> Dir {
    match *d {
        Dir::U => Dir::D,
        Dir::D => Dir::U,
        Dir::L => Dir::R,
        Dir::R => Dir::L,
    }
}

// fn inside_curve(r: u64, c: u64, top: u64, points_on_curve: &HashSet<(u64, u64)>) -> bool {
//     // TODO go in the fastest direction to save time
//     let verbose = r == 1 && c == 2;
//     let mut current_r = r;
//     if points_on_curve.contains(&(r, c)) {
//         false
//     } else {
//         let mut num_crossings = 0;
//         let mut currently_crossing = false;
//         while current_r < top {
//             current_r += 1;
//             if verbose {
//                 println!("current r {}", current_r);
//                 println!("on curve? {}", points_on_curve.contains(&(current_r, c)))
//             }
//             if points_on_curve.contains(&(current_r, c)) {
//                 if !currently_crossing {
//                     num_crossings += 1;
//                 }
//                 currently_crossing = true;
//             } else {
//                 currently_crossing = false;
//             }
//         }
//         num_crossings % 2 == 1
//     }
// }

fn step_in_dir(point: (i64, i64, Dir), d: Dir) -> (i64, i64, Dir) {
    let r = point.0;
    let c = point.1;
    match d {
        Dir::U => (r - 1, c, d),
        Dir::D => (r + 1, c, d),
        Dir::L => (r, c - 1, d),
        Dir::R => (r, c + 1, d),
    }
}

fn str_to_dir(d_str: &str) -> Dir {
    if d_str == "U" {
        Dir::U
    } else if d_str == "D" {
        Dir::D
    } else if d_str == "L" {
        Dir::L
    } else {
        Dir::R
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
enum Dir {
    U,
    D,
    L,
    R,
}
