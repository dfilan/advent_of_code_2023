use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

// 618463011412721 too low
// 618466068570321 also too low

fn main() {
    let start = Instant::now();
    let num_steps: usize = 26501365; // should be 26501365 on real input
    let result = part_2("./input.txt", num_steps);
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

fn part_2(file_path: &str, num_steps_total: usize) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut plot: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let char_vec = line.chars().collect::<Vec<_>>();
        plot.push(char_vec);
    }
    let num_rows = plot.len();
    // so: you can go up one thing in num_rows steps
    // also everything on every diagonal is hit in exactly one spot
    // so only need to solve four problems
    // actually: can be hit top, bottom, L, R, also on the diagonal early or late
    // oh god damn it you have exactly enough time to just hit the top / bottom
    let meta_steps = (num_steps_total - (num_rows / 2)) / num_rows;
    println!("meta steps {}", meta_steps);
    // hit meta-coords where |x| + |y| <= meta_steps (maybe plus one on diag?)
    let num_steps_major_diag = (num_rows / 2) + num_rows - 1;
    let num_steps_minor_diag = (num_rows / 2) - 1;
    let top_steps = propagate_num_steps(num_rows - 1, num_rows / 2, num_rows - 1, &plot);
    println!("top_steps {}", top_steps);
    let bot_steps = propagate_num_steps(0, num_rows / 2, num_rows - 1, &plot);
    let l_steps = propagate_num_steps(num_rows / 2, num_rows - 1, num_rows - 1, &plot);
    let r_steps = propagate_num_steps(num_rows / 2, 0, num_rows - 1, &plot);
    let tl_major_steps = propagate_num_steps(num_rows - 1, num_rows - 1, num_steps_major_diag, &plot);
    println!("diag major steps {}", tl_major_steps);
    let tl_minor_steps = propagate_num_steps(num_rows - 1, num_rows - 1, num_steps_minor_diag, &plot);
    println!("diag minor steps {}", tl_minor_steps);
    let tr_major_steps = propagate_num_steps(num_rows - 1, 0, num_steps_major_diag, &plot);
    let tr_minor_steps = propagate_num_steps(num_rows - 1, 0, num_steps_minor_diag, &plot);
    let bl_major_steps = propagate_num_steps(0, num_rows - 1, num_steps_major_diag, &plot);
    let bl_minor_steps = propagate_num_steps(0, num_rows - 1, num_steps_minor_diag, &plot);
    let br_major_steps = propagate_num_steps(0, 0, num_steps_major_diag, &plot);
    let br_minor_steps = propagate_num_steps(0, 0, num_steps_minor_diag, &plot);
    // so interior numbers vary, better get the even and odds
    let interior_steps_even = propagate_num_steps(num_rows / 2, num_rows / 2, num_rows + 1, &plot);
    println!("interior_steps_even {}", interior_steps_even);
    let interior_steps_odd = propagate_num_steps(num_rows / 2, num_rows / 2, num_rows, &plot);
    println!("interior_steps_odd {}", interior_steps_odd);
    let mut accum = 0;
    // how many things on each diagonal?
    // major diagonal is when |x| + |y| = meta_steps, x > 0, y > 0 - that's meta_steps - 1!
    // also minor diagonal has meta_steps things on it
    accum += tl_major_steps * (meta_steps - 1);
    accum += tl_minor_steps * meta_steps;
    accum += tr_major_steps * (meta_steps - 1);
    accum += tr_minor_steps * meta_steps;
    accum += bl_major_steps * (meta_steps - 1);
    accum += bl_minor_steps * meta_steps;
    accum += br_major_steps * (meta_steps - 1);
    accum += br_minor_steps * meta_steps;
    // also add top, bot, left, right
    accum += top_steps;
    accum += bot_steps;
    accum += l_steps;
    accum += r_steps;
    // now, how many things in the interior? and how many are odd vs even?
    // when meta_steps = 2, there's one thing, and it depends on the parity of num_steps
    // total is 2*(meta_steps) * (meta_steps - 1) + 1
    // half of them are even, half are odd
    // wait that's not true
    let match_origin_lim = (meta_steps - 1) / 2;
    let steps_matching_origin = 1 + 4 * match_origin_lim * (match_origin_lim + 1);
    println!("steps_matching_origin {}", steps_matching_origin);
    let off_origin_lim = (meta_steps - 2) / 2;
    let steps_off_origin = 4 * (off_origin_lim + 1).pow(2);
    println!("steps off-origin {}", steps_off_origin);
    if num_steps_total % 2 == 0 {
        // origin will be on an even blink
        accum += steps_matching_origin * interior_steps_even;
        accum += steps_off_origin * interior_steps_odd;
    } else {
        // origin will be on an odd blink
        accum += steps_matching_origin * interior_steps_odd;
        accum += steps_off_origin * interior_steps_even;
    }
    accum
}

fn propagate_num_steps(r: usize, c: usize, n_steps: usize, plot: &Vec<Vec<char>>) -> usize {
    // figure out even-odd pattern of original grid
    let mut frontier = HashSet::new();
    frontier.insert((r, c));
    let mut even_steps = HashSet::new();
    let mut odd_steps = HashSet::new();
    for k in 0..n_steps {
        // get the frontier
        // walk in all directions that aren't covered by even steps or odd steps
        // that's the new frontier
        // old frontier goes into (parity of k)_steps
        let mut new_frontier = HashSet::new();
        let current_parity = k % 2;
        for (r, c) in frontier {
            let successors = get_successors(r, c, plot);
            for succ in successors {
                new_frontier.insert(succ);
            }
            if current_parity == 0 {
                even_steps.insert((r, c));
            } else {
                odd_steps.insert((r, c));
            }
        }
        let next_parity = (k + 1) % 2;
        let real_new_frontier: HashSet<(usize, usize)> = if next_parity == 0 {
            new_frontier.difference(&even_steps).copied().collect()
        } else {
            new_frontier.difference(&odd_steps).copied().collect()
        };
        frontier = real_new_frontier;
    }
    if n_steps % 2 == 0 {
        even_steps.len() + frontier.len()
    } else {
        odd_steps.len() + frontier.len()
    }
}


// ok, need to take advantage of how shit's repeated
// all borders are dots, so that's presumably important
// still too slow, i guess because fundamentally I'm still doing search over everything...
// could try A* to get to the next thing?
// like, I can't get away with not knowing how many steps it takes to cross a path...
// oh I should be able to memoize my answers, because there's only like a few hundred options.
// fuck, memoization isn't good enough.
// want to be able to implicitly know how many things I can reach and in what time

// ok, paths have to be symmetric, so that's a saving i can have
// except because i'm going from a definite point to a whole receptor set they're not
// also note that different paths to cross have different parities

// frontier is growing pretty slowly, which i guess makes sense
// wait how long does it take to get to -63, -14?
// and why are we taking so long to pop 1000 things off?
// ok i needed to use a hashset not a vec, obvs

// ok next problem: a given meta-coord can be hit multiple times while it's blinking, from the
// same direction
// solution: if I'm hitting you before you hit equilibrium, got to store all the ways I hit
// you
// which requires starting at the first way I hit you and spreading out
// but i could also hit you from above or below in that time?
// ok but that's feasible to deal with
// also note that it's not going to be from the things that hit you in the same / adjacent
// places
// also you can get hit by a thing that spread from the second thing that hit the thing above
// you...
// actually I don't have to worry about that because of the border of dots: if I hit your
// parent and then hit you, there's another path that hits you directly in the same time

// oh also note that the 'frontier' of things not done blinking can be a few layers deep

// hmmm i feel like all this futzing is a sign that my solution isn't the best thing

// also holy shit I'm treating the corners incorrectly gah - you don't know which way you'll
// be hit first.
// it's not even true that the quickest path to one of your successors comes from the first
// time you got hit. could be that you were hit in a hard-to-manoeuvre spot but your neighbour
// was hit right next to you.

// ok I think I need to rethink this whole thing.

fn part_2_dumb(file_path: &str, num_steps_total: u64) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut plot: Vec<Vec<char>> = Vec::new();
    let (mut s_row, mut s_col) = (0, 0);
    for (i, line) in lines.enumerate() {
        let char_vec = line.chars().collect::<Vec<_>>();
        plot.push(char_vec.clone());
        if let Some(j) = char_vec.iter().position(|c| *c == 'S') {
            s_row = i;
            s_col = j;
        }
    }
    println!("mapped world");
    // figure out even-odd pattern of original grid
    let mut frontier = HashSet::new();
    frontier.insert((s_row, s_col));
    let mut even_steps = HashSet::new();
    let mut odd_steps = HashSet::new();
    let mut steps_complete_blinking = 0;
    while !frontier.is_empty() {
        // get the frontier
        // walk in all directions that aren't covered by even steps or odd steps
        // that's the new frontier
        // old frontier goes into (parity of (k - 1))_steps
        let mut new_frontier = HashSet::new();
        let old_parity = steps_complete_blinking % 2;
        for (r, c) in frontier {
            let successors = get_successors(r, c, &plot);
            for succ in successors {
                new_frontier.insert(succ);
            }
            if old_parity == 0 {
                even_steps.insert((r, c));
            } else {
                odd_steps.insert((r, c));
            }
        }
        let current_parity = (steps_complete_blinking + 1) % 2;
        let real_new_frontier: HashSet<(usize, usize)> = if current_parity == 0 {
            new_frontier.difference(&even_steps).copied().collect()
        } else {
            new_frontier.difference(&odd_steps).copied().collect()
        };
        frontier = real_new_frontier;
        steps_complete_blinking += 1;
    }
    println!("figured out evens and odds");
    // now steps_complete_blinking records how many steps it takes to get to the blinking
    // equilibrium
    let num_rows = plot.len();
    let num_cols = plot[0].len();
    // (row, col, meta_row, meta_col, steps_to_there, dir you came from)
    let mut frontier: VecDeque<(usize, usize, i64, i64, u64, Option<Dir>)> =
        VecDeque::from([(s_row, s_col, 0, 0, 0, None)]);
    let mut explored: Vec<(usize, usize, i64, i64, u64, Option<Dir>)> =
        vec![(s_row, s_col, 0, 0, 0, None)];
    let mut explored_metas: HashSet<(i64, i64)> = HashSet::new();
    explored_metas.insert((0, 0));
    let mut bfs_cache: HashMap<(usize, usize, Dir), (u64, usize)> = HashMap::new();
    let mut k = 0;
    let mut checkpoint = Instant::now();
    while let Some((row, col, meta_row, meta_col, num_steps, opt_dir)) = frontier.pop_front() {
        k += 1;
        if k % 1_000_000 == 0 {
            println!("size of frontier: {}", frontier.len());
            println!("size of explored: {}", explored.len());
            println!("expanding meta-coords {}, {}", meta_row, meta_col);
            println!("took us {} to get there", num_steps);
            println!("time to do 1,000,000 pops {:?}", checkpoint.elapsed());
            checkpoint = Instant::now();
        }

        // find all the things you can get to without going over num_steps_total
        // go out and clockwise, but don't add a meta_row, meta_col pair that's already on
        // the frontier or already explored (sad we have to add this last thing)
        let to_dirs = match opt_dir {
            None => vec![Dir::N, Dir::S, Dir::E, Dir::W],
            Some(Dir::N) => vec![Dir::N, Dir::E],
            Some(Dir::S) => vec![Dir::S, Dir::W],
            Some(Dir::E) => vec![Dir::E, Dir::S],
            Some(Dir::W) => vec![Dir::W, Dir::N],
        };
        for next_dir in to_dirs {
            let (next_meta_row, next_meta_col) = new_metas(meta_row, meta_col, next_dir);
            let (steps_there, edge_coord) =
                bfs_path_find(row, col, next_dir, &plot, &mut bfs_cache);
            let next_num_steps = num_steps + steps_there + 1;
            let next_in_explored = explored_metas.contains(&(next_meta_row, next_meta_col));
            if next_num_steps <= num_steps_total && !next_in_explored {
                // add them to the frontier with their coords and steps to getting to
                // there
                // also add them to the explored vec + hash set
                let (next_row, next_col) = match next_dir {
                    Dir::N => (num_rows - 1, edge_coord),
                    Dir::S => (0, edge_coord),
                    Dir::E => (edge_coord, 0),
                    Dir::W => (edge_coord, num_cols - 1),
                };
                frontier.push_back((
                    next_row,
                    next_col,
                    next_meta_row,
                    next_meta_col,
                    next_num_steps,
                    Some(next_dir),
                ));
                explored.push((
                    next_row,
                    next_col,
                    next_meta_row,
                    next_meta_col,
                    next_num_steps,
                    Some(next_dir),
                ));
                explored_metas.insert((next_meta_row, next_meta_col));
            }
        }
    }
    // now, for everything in explored, if it's had enough time to reach the blinking
    // equilibrium, figure out which state of that it's in, else, figure out where it gets to
    // in the time it has left
    let start_adding = Instant::now();
    let mut accum = 0;
    println!("num explored {}", explored.len());
    println!("steps_complete_blinking {}", steps_complete_blinking);
    let mut num_time_to_percolate = 0;
    let mut fringe_info: HashMap<(i64, i64), (Option<Dir>, Vec<HashSet<(usize, usize)>>)> =
        HashMap::new();
    let mut fringe_vec: VecDeque<(i64, i64, Vec<(usize, usize)>)> = VecDeque::new();
    for (row, col, meta_row, meta_col, arrival_step, arrival_dir) in explored {
        let steps_left = num_steps_total - arrival_step;
        if steps_left > steps_complete_blinking {
            num_time_to_percolate += 1;
            if (num_steps_total + arrival_step) % 2 == 0 {
                accum += even_steps.len();
            } else {
                accum += odd_steps.len();
            }
        } else {
            let mut singleton_set = HashSet::new();
            singleton_set.insert((row, col));
            let mut step_num = arrival_step;
            // println!("arrival_step {}", arrival_step);
            // println!("arrival coord {}, {}", row, col);
            let mut current_set = singleton_set;
            let mut next_set = HashSet::new();
            let mut blinks = vec![current_set.clone()];
            while step_num != num_steps_total {
                for (r, c) in current_set.iter() {
                    let successors = get_successors(*r, *c, &plot);
                    // if step_num == 6 {
                    //     println!("point: {}, {}", *r, *c);
                    //     println!("successors: {:?}", successors);
                    // }
                    for successor in successors {
                        next_set.insert(successor);
                    }
                }
                step_num += 1;
                current_set = next_set;
                // println!(
                //     "things you reach in {} steps: {}",
                //     step_num,
                //     current_set.len()
                // );
                next_set = HashSet::new();
                blinks.push(current_set.clone());
            }
            // remember that (meta_row, meta_col) is on the fringe
            // remember which way we came to it,
            // and its sequence of blinkings
            fringe_info.insert((meta_row, meta_col), (arrival_dir, blinks));
            fringe_vec.push_back((meta_row, meta_col, vec![(row, col)]));
            accum += current_set.len();
        }
    }

    // let inner_fringe = Vec::new();
    // for (m_row, m_col, hits_vec) in fringe_vec {
    //     let arrival_dir = fringe_info.get(&(m_row, m_col)).unwrap().0;
    //     let parent = get_parent(m_row, m_col, arrival_dir);
    //     if !fringe_info.contains_key(&parent) {
    //         inner_fringe.push();
    //     }
    // }
    
    // ok, broadly I want to hit every time something on the fringe is hit with something new
    // so I should iterate thru times things are hit with new things
    // first loop should be everything on the inner-most fringe: get its parents not on the
    // fringe, see when the parents hit it in novel ways, add those to my working vec

    // oh but the parents might not have been in equilibrium... god damn...


    // second loop is over the working vec: see new touches, propagate, add new touches to working vec
    
    println!("num time to percolate {}", num_time_to_percolate);
    println!("time to add stuff up: {:?}", start_adding.elapsed());
    accum
}

fn get_parent(meta_row: i64, meta_col: i64, dir: Dir) -> (i64, i64) {
    match dir {
        Dir::N => (meta_row + 1, meta_col),
        Dir::S => (meta_row - 1, meta_col),
        Dir::E => (meta_row, meta_col - 1),
        Dir::W => (meta_row, meta_col + 1),
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct SearchState {
    row: usize,
    col: usize,
    steps_to: u64,
    est_path_cost: u64,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip sign so that BFS uses a min-heap
        other
            .est_path_cost
            .cmp(&self.est_path_cost)
            .then_with(|| self.steps_to.cmp(&other.steps_to))
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.row.cmp(&other.row))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs_path_find(
    start_row: usize,
    start_col: usize,
    dir: Dir,
    plot: &[Vec<char>],
    cache: &mut HashMap<(usize, usize, Dir), (u64, usize)>,
) -> (u64, usize) {
    let input = (start_row, start_col, dir);
    if let Some(result) = cache.get(&input) {
        *result
    } else {
        // search state is basically (row, col, num_steps, heuristic)
        let heuristic = |r, c, _| match dir {
            Dir::N => r,
            Dir::S => plot.len() - r - 1,
            Dir::E => plot[0].len() - c - 1,
            Dir::W => c,
        };
        let mut frontier: BinaryHeap<SearchState> = BinaryHeap::from([SearchState {
            row: start_row,
            col: start_col,
            steps_to: 0,
            est_path_cost: heuristic(start_row, start_col, 0) as u64,
        }]);
        let mut best_costs_to: HashMap<(usize, usize), u64> = HashMap::new();
        while let Some(search_state) = frontier.pop() {
            let r = search_state.row;
            let c = search_state.col;
            let n = search_state.steps_to;
            match dir {
                Dir::N => {
                    if r == 0 {
                        cache.insert(input, (n, c));
                        return (n, c);
                    }
                }
                Dir::S => {
                    if r == plot.len() - 1 {
                        cache.insert(input, (n, c));
                        return (n, c);
                    }
                }
                Dir::E => {
                    if c == plot[0].len() - 1 {
                        cache.insert(input, (n, r));
                        return (n, r);
                    }
                }
                Dir::W => {
                    if c == 0 {
                        cache.insert(input, (n, r));
                        return (n, r);
                    }
                }
            };
            let successors = get_successors(r, c, plot);
            // only push things that haven't been explored and where it's the cheapest way to
            // get there
            for (next_r, next_c) in successors {
                let succ_heur = heuristic(next_r, next_c, n + 1) as u64;
                let next_state = SearchState {
                    row: next_r,
                    col: next_c,
                    steps_to: n + 1,
                    est_path_cost: succ_heur + n + 1,
                };
                match best_costs_to.get(&(next_r, next_c)) {
                    None => {
                        best_costs_to.insert((next_r, next_c), n + 1);
                        frontier.push(next_state);
                    }
                    Some(x) => {
                        if n + 1 < *x {
                            best_costs_to.insert((next_r, next_c), n + 1);
                            frontier.push(next_state);
                        }
                    }
                }
            }
        }
        (0, 0)
    }
}

fn new_metas(meta_row: i64, meta_col: i64, dir: Dir) -> (i64, i64) {
    match dir {
        Dir::N => (meta_row - 1, meta_col),
        Dir::S => (meta_row + 1, meta_col),
        Dir::E => (meta_row, meta_col + 1),
        Dir::W => (meta_row, meta_col - 1),
    }
}

// fn get_successors_2(
//     point: (usize, usize, i64, i64),
//     plot: &Vec<Vec<char>>,
// ) -> Vec<(usize, usize, i64, i64)> {
//     let (row, col, meta_row, meta_col) = point;
//     let num_rows = plot.len();
//     let num_cols = plot[0].len();
//     let mut successors = Vec::new();
//     if row > 0 {
//         successors.push((row - 1, col, meta_row, meta_col));
//     } else {
//         successors.push((num_rows - 1, col, meta_row - 1, meta_col));
//     }
//     if row < num_rows - 1 {
//         successors.push((row + 1, col, meta_row, meta_col));
//     } else {
//         successors.push((0, col, meta_row + 1, meta_col));
//     }
//     if col > 0 {
//         successors.push((row, col - 1, meta_row, meta_col));
//     } else {
//         successors.push((row, num_cols - 1, meta_row, meta_col - 1));
//     }
//     if col < num_cols - 1 {
//         successors.push((row, col + 1, meta_row, meta_col));
//     } else {
//         successors.push((row, 0, meta_row, meta_col + 1));
//     }
//     successors
//         .iter()
//         .filter(|(r, c, _, _)| plot[*r][*c] != '#')
//         .copied()
//         .collect::<Vec<_>>()
// }

// bfs?
// except it's places in exactly n steps
// different queues for different num steps?

// this is inefficient because I could more easily deal with what happens when you walk back
// but whatever it's fine
// also what if you go in loops, this is graph search

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let num_steps: u64 = 10; // should be 64
    let mut plot: Vec<Vec<char>> = Vec::new();
    let (mut s_row, mut s_col) = (0, 0);
    for (i, line) in lines.enumerate() {
        let char_vec = line.chars().collect::<Vec<_>>();
        plot.push(char_vec.clone());
        if let Some(j) = char_vec.iter().position(|c| *c == 'S') {
            s_row = i;
            s_col = j;
        }
    }
    // first thing in tuple is depth, second thing in vec is row, third thing is col
    let mut singleton_set = HashSet::new();
    singleton_set.insert((s_row, s_col));
    let mut step_num = 0;
    let mut current_set = singleton_set;
    let mut next_set = HashSet::new();
    while step_num != num_steps {
        for (r, c) in current_set.iter() {
            let successors = get_successors(*r, *c, &plot);
            for successor in successors {
                next_set.insert(successor);
            }
        }
        step_num += 1;
        current_set = next_set;
        next_set = HashSet::new();
    }
    current_set.len()
}

fn get_successors(row: usize, col: usize, plot: &[Vec<char>]) -> Vec<(usize, usize)> {
    let num_rows = plot.len();
    let num_cols = plot[0].len();
    let mut successors = Vec::new();
    if row > 0 {
        successors.push((row - 1, col));
    }
    if row < num_rows - 1 {
        successors.push((row + 1, col));
    }
    if col > 0 {
        successors.push((row, col - 1));
    }
    if col < num_cols - 1 {
        successors.push((row, col + 1));
    }
    successors
        .iter()
        .filter(|(r, c)| plot[*r][*c] != '#')
        .map(|(r, c)| (*r, *c))
        .collect::<Vec<_>>()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}
