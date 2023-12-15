use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let result = part_2("./input.txt");
    println!("result {}", result);
}

// how many tiles enclosed by the loop?
// well, first find a loop that succeeds
// if it succeeds, mark everything in the loop
// also figure out whether you're going clockwise or counter-clockwise (by net angle you rotate, stored as num quarter turns clockwise)
// at every point in the loop, take a straight line in, mark everything that isn't something you've already marked
// return amount of things you've marked minus length of the loop (that works, right?)

// ok I'm too low
// so there's things I'm missing
// what could they be???

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // find S
    let mut start_x = 0; // x is which line, 0 is top
    let mut start_y = 0; // y is which char, 0 is left
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_x = i;
                start_y = j;
            }
        }
    }
    let num_lines = map.len();
    let len_line = map[0].len();
    let mut start_poses: Vec<(usize, usize, Dir, i64)> = Vec::new();
    if start_x > 0 {
        start_poses.push((start_x - 1, start_y, Dir::Up, 0));
    }
    if start_x < num_lines - 1 {
        start_poses.push((start_x + 1, start_y, Dir::Down, 0));
    }
    if start_y > 0 {
        start_poses.push((start_x, start_y - 1, Dir::Left, 0));
    }
    if start_y < len_line - 1 {
        start_poses.push((start_x, start_y + 1, Dir::Right, 0));
    }
    for (x, y, d, t) in start_poses {
        let mut curr_x = x;
        let mut curr_y = y;
        let mut curr_d = d;
        let mut curr_t = t;
        let mut curr_c = map[curr_x][curr_y];
        let mut len_opt = Some(0);
        // gotta fail out if loop fails
        let mut this_loop: Vec<(usize, usize, Dir)> = vec![(x, y, d)];
        while curr_c != 'S' && len_opt.is_some() {
            let maybe_next = next_char(curr_x, curr_y, curr_d, curr_c, curr_t);
            match maybe_next {
                None => {
                    len_opt = None;
                }
                Some((next_x, next_y, next_d, next_t)) => {
                    len_opt = len_opt.map(|n| n + 1);
                    curr_x = next_x;
                    curr_y = next_y;
                    curr_d = next_d;
                    curr_t = next_t;
                    curr_c = map[curr_x][curr_y];
                    this_loop.push((curr_x, curr_y, curr_d));
                }
            }
        }
        // for t, + is clockwise, - is counterclockwise
        // one way to mark things is add them to my new thing
        // if curve goes clockwise, ray goes clockwise
        if let Some(_len) = len_opt {
            let mut inside_curve_set: HashSet<(usize, usize)> = HashSet::new();
            let mut on_loop_set: HashSet<(usize, usize)> = HashSet::new();
            for (x, y, _) in this_loop.clone() {
                on_loop_set.insert((x, y));
            }
            // redo loop but do ray thing
            for (x, y, d) in this_loop {
                let c = map[x][y];
                let in_dirs = get_dirs_in(c, d, curr_t);
                for in_dir in in_dirs {
                    mark_interior(x, y, in_dir, &mut inside_curve_set, &on_loop_set);
                }
            }
            return inside_curve_set.len();
        }
    }
    0
}

fn mark_interior(
    start_x: usize,
    start_y: usize,
    in_dir: Dir,
    inside_curve_set: &mut HashSet<(usize, usize)>,
    on_loop_set: &HashSet<(usize, usize)>,
) {
    // have to go one in before you go in dir
    let (mut x, mut y) = go_one_step(start_x, start_y, in_dir);
    while !on_loop_set.contains(&(x, y)) {
        inside_curve_set.insert((x, y));
        (x, y) = go_one_step(x, y, in_dir);
    }
    // while !(inside_curve_set.contains(&(x, y)) || on_loop_set.contains(&(x, y))) {
    //     inside_curve_set.insert((x, y));
    //     (x, y) = go_one_step(x, y, in_dir);
    // }
}

fn go_one_step(x: usize, y: usize, dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Up => (x - 1, y),
        Dir::Down => (x + 1, y),
        Dir::Left => (x, y - 1),
        Dir::Right => (x, y + 1),
    }
}

fn get_dirs_in(c: char, d: Dir, t: i64) -> Vec<Dir> {
    // do I have to do anything if c = S?
    // I think no because every interior point can be reached in 4 dirs
    if t > 0 {
        // clockwise
        match c {
            '-' => match d {
                Dir::Left => vec![Dir::Up],
                Dir::Right => vec![Dir::Down],
                _ => Vec::new(),
            },
            '|' => match d {
                Dir::Up => vec![Dir::Right],
                Dir::Down => vec![Dir::Left],
                _ => Vec::new(),
            },
            'L' => match d {
                Dir::Down => vec![Dir::Down, Dir::Left],
                Dir::Left => Vec::new(),
                _ => Vec::new(),
            },
            'J' => match d {
                Dir::Down => Vec::new(),
                Dir::Right => vec![Dir::Down, Dir::Right],
                _ => Vec::new(),
            },
            '7' => match d {
                Dir::Up => vec![Dir::Up, Dir::Right],
                Dir::Right => Vec::new(),
                _ => Vec::new(),
            },
            'F' => match d {
                Dir::Up => Vec::new(),
                Dir::Left => vec![Dir::Up, Dir::Left],
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    } else {
        // anticlockwise
        // should really check t != 0, t = 4 or -4
        match c {
            '-' => match d {
                Dir::Left => vec![Dir::Down],
                Dir::Right => vec![Dir::Up],
                _ => Vec::new(),
            },
            '|' => match d {
                Dir::Up => vec![Dir::Left],
                Dir::Down => vec![Dir::Right],
                _ => Vec::new(),
            },
            'L' => match d {
                Dir::Down => Vec::new(),
                Dir::Left => vec![Dir::Down, Dir::Left],
                _ => Vec::new(),
            },
            'J' => match d {
                Dir::Down => vec![Dir::Down, Dir::Right],
                Dir::Right => Vec::new(),
                _ => Vec::new(),
            },
            '7' => match d {
                Dir::Up => Vec::new(),
                Dir::Right => vec![Dir::Up, Dir::Right],
                _ => Vec::new(),
            },
            'F' => match d {
                Dir::Up => vec![Dir::Up, Dir::Left],
                Dir::Left => Vec::new(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }
}

// S is start, there's pipe of some shape on that tile
// find tile most steps from original animal
// find S
// go in each 4 directions
// keep following
// stop when you can't get anywhere
// figure out which is the loop, how long it is until you get back

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // find S
    let mut start_x = 0; // x is which line, 0 is top
    let mut start_y = 0; // y is which char, 0 is left
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_x = i;
                start_y = j;
            }
        }
    }
    let num_lines = map.len();
    let len_line = map[0].len();
    let mut start_poses: Vec<(usize, usize, Dir)> = Vec::new();
    if start_x > 0 {
        start_poses.push((start_x - 1, start_y, Dir::Up));
    }
    if start_x < num_lines - 1 {
        start_poses.push((start_x + 1, start_y, Dir::Down));
    }
    if start_y > 0 {
        start_poses.push((start_x, start_y - 1, Dir::Left));
    }
    if start_y < len_line - 1 {
        start_poses.push((start_x, start_y + 1, Dir::Right));
    }
    for (x, y, d) in start_poses {
        let mut curr_x = x;
        let mut curr_y = y;
        let mut curr_d = d;
        let mut curr_c = map[curr_x][curr_y];
        let mut n_opt = Some(0);
        // gotta fail out if loop fails
        while curr_c != 'S' && n_opt.is_some() {
            let maybe_next = next_char_old(curr_x, curr_y, curr_d, curr_c);
            match maybe_next {
                None => {
                    n_opt = None;
                }
                Some((next_x, next_y, next_d)) => {
                    n_opt = n_opt.map(|n| n + 1);
                    curr_x = next_x;
                    curr_y = next_y;
                    curr_d = next_d;
                    curr_c = map[curr_x][curr_y];
                }
            }
        }
        if let Some(len) = n_opt {
            return (len + 1) / 2;
        }
    }
    0
}

fn next_char_old(x: usize, y: usize, d: Dir, c: char) -> Option<(usize, usize, Dir)> {
    match c {
        '|' => match d {
            Dir::Up => Some((x - 1, y, Dir::Up)),
            Dir::Down => Some((x + 1, y, Dir::Down)),
            _ => None,
        },
        '-' => match d {
            Dir::Left => Some((x, y - 1, Dir::Left)),
            Dir::Right => Some((x, y + 1, Dir::Right)),
            _ => None,
        },
        'L' => match d {
            Dir::Down => Some((x, y + 1, Dir::Right)),
            Dir::Left => Some((x - 1, y, Dir::Up)),
            _ => None,
        },
        'J' => match d {
            Dir::Down => Some((x, y - 1, Dir::Left)),
            Dir::Right => Some((x - 1, y, Dir::Up)),
            _ => None,
        },
        '7' => match d {
            Dir::Up => Some((x, y - 1, Dir::Left)),
            Dir::Right => Some((x + 1, y, Dir::Down)),
            _ => None,
        },
        'F' => match d {
            Dir::Up => Some((x, y + 1, Dir::Right)),
            Dir::Left => Some((x + 1, y, Dir::Down)),
            _ => None,
        },
        _ => None,
    }
}

fn next_char(x: usize, y: usize, d: Dir, c: char, t: i64) -> Option<(usize, usize, Dir, i64)> {
    match c {
        '|' => match d {
            Dir::Up => Some((x - 1, y, Dir::Up, t)),
            Dir::Down => Some((x + 1, y, Dir::Down, t)),
            _ => None,
        },
        '-' => match d {
            Dir::Left => Some((x, y - 1, Dir::Left, t)),
            Dir::Right => Some((x, y + 1, Dir::Right, t)),
            _ => None,
        },
        'L' => match d {
            Dir::Down => Some((x, y + 1, Dir::Right, t - 1)),
            Dir::Left => Some((x - 1, y, Dir::Up, t + 1)),
            _ => None,
        },
        'J' => match d {
            Dir::Down => Some((x, y - 1, Dir::Left, t + 1)),
            Dir::Right => Some((x - 1, y, Dir::Up, t - 1)),
            _ => None,
        },
        '7' => match d {
            Dir::Up => Some((x, y - 1, Dir::Left, t - 1)),
            Dir::Right => Some((x + 1, y, Dir::Down, t + 1)),
            _ => None,
        },
        'F' => match d {
            Dir::Up => Some((x, y + 1, Dir::Right, t + 1)),
            Dir::Left => Some((x + 1, y, Dir::Down, t - 1)),
            _ => None,
        },
        _ => None,
    }
}
