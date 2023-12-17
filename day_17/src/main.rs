use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// don't incur top-left unless you return
// after 3 blocks, must turn (but can do 3 in a row)

// so this is A* right? one trickiness is you've got to keep track of steps in a row
// A*: see a state, add it to the

// ok this takes ages
// is there problem structure to exploit? I think not
// am I doing something wrong? well, I'm inserting a ton of shit into the queue
// want to only insert if there's nothing better in the queue
// and want to replace shit in the queue
// but is that even that costly?

fn part_2(file_path: &str) -> u32 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        map.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut best_costs_to: HashMap<CostlessState, u32> = HashMap::new();
    let num_rows = map.len();
    let num_cols = map[0].len();
    for dir in [Dir::D, Dir::R] {
        let cstate = CostlessState {
            row: 0,
            col: 0,
            d: dir,
            num_steps: 1,
        };
        frontier.push(State {
            cstate: cstate,
            cost_to_here: 0,
            est_path_cost: (num_rows + num_cols) as u32,
        });
    }
    while let Some(s) = frontier.pop() {
        if s.cstate.row == num_rows - 1 && s.cstate.col == num_cols - 1 && s.cstate.num_steps > 3 {
            return s.cost_to_here;
        }
        for succ in successors_2(&s.cstate, &map) {
            let succ_heur = heuristic(&succ, &map);
            let succ_cost_to = s.cost_to_here + map[succ.row][succ.col];
            let next_state = State {
                cstate: succ,
                cost_to_here: succ_cost_to,
                est_path_cost: succ_cost_to + succ_heur,
            };
            match best_costs_to.get(&succ) {
                None => {
                    best_costs_to.insert(succ, succ_cost_to);
                    frontier.push(next_state);
                }
                Some(x) => {
                    if succ_cost_to < *x {
                        best_costs_to.insert(succ, succ_cost_to);
                        frontier.push(next_state);
                    }
                }
            };
        }
    }
    0
}

fn successors_2(cs: &CostlessState, map: &Vec<Vec<u32>>) -> Vec<CostlessState> {
    // s.num_steps is 1 if this is your first step going in that dir, 2 if it's your second,
    // 3 if it's your third, etc.
    let mut next_options = Vec::new();
    if cs.num_steps < 10 {
        next_options.append(&mut go_straight(cs, map));
    }
    if cs.num_steps > 3 {
        for next_d in turn_90(cs.d) {
            let turned_state = CostlessState {
                d: next_d,
                num_steps: 0,
                ..*cs
            };
            next_options.append(&mut go_straight(&turned_state, map));
        }
    }
    next_options
}

fn part_1(file_path: &str) -> u32 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        map.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut best_costs_to: HashMap<CostlessState, u32> = HashMap::new();
    let num_rows = map.len();
    let num_cols = map[0].len();
    for dir in [Dir::D, Dir::R] {
        let cstate = CostlessState {
            row: 0,
            col: 0,
            d: dir,
            num_steps: 1,
        };
        frontier.push(State {
            cstate: cstate,
            cost_to_here: 0,
            est_path_cost: (num_rows + num_cols) as u32,
        });
    }
    while let Some(s) = frontier.pop() {
        if s.cstate.row == num_rows - 1 && s.cstate.col == num_cols - 1 {
            return s.cost_to_here;
        }
        for succ in successors(&s.cstate, &map) {
            let succ_heur = heuristic(&succ, &map);
            let succ_cost_to = s.cost_to_here + map[succ.row][succ.col];
            let next_state = State {
                cstate: succ,
                cost_to_here: succ_cost_to,
                est_path_cost: succ_cost_to + succ_heur,
            };
            match best_costs_to.get(&succ) {
                None => {
                    best_costs_to.insert(succ, succ_cost_to);
                    frontier.push(next_state);
                }
                Some(x) => {
                    if succ_cost_to < *x {
                        best_costs_to.insert(succ, succ_cost_to);
                        frontier.push(next_state);
                    }
                }
            };
        }
    }
    0
}

fn successors(cs: &CostlessState, map: &Vec<Vec<u32>>) -> Vec<CostlessState> {
    // s.num_steps is 1 if this is your first step going in that dir, 2 if it's your second,
    // 3 if it's your third
    let mut next_options = Vec::new();
    if cs.num_steps < 3 {
        next_options.append(&mut go_straight(cs, map));
    }
    for next_d in turn_90(cs.d) {
        let turned_state = CostlessState {
            d: next_d,
            num_steps: 0,
            ..*cs
        };
        next_options.append(&mut go_straight(&turned_state, map));
    }
    next_options
}

fn heuristic(cstate: &CostlessState, map: &Vec<Vec<u32>>) -> u32 {
    let num_rows = map.len() as u32;
    let num_cols = map[0].len() as u32;
    let r = cstate.row as u32;
    let c = cstate.col as u32;
    num_rows - r - 1 + num_cols - c - 1
}

fn go_straight(cstate: &CostlessState, map: &Vec<Vec<u32>>) -> Vec<CostlessState> {
    // note: I've got to update the cost to get to get here...
    let num_rows = map.len();
    let num_cols = map[0].len();
    match cstate.d {
        Dir::U => {
            if cstate.row == 0 {
                Vec::new()
            } else {
                vec![CostlessState {
                    row: cstate.row - 1,
                    num_steps: cstate.num_steps + 1,
                    ..*cstate
                }]
            }
        }
        Dir::D => {
            if cstate.row == num_rows - 1 {
                Vec::new()
            } else {
                vec![CostlessState {
                    row: cstate.row + 1,
                    num_steps: cstate.num_steps + 1,
                    ..*cstate
                }]
            }
        }
        Dir::L => {
            if cstate.col == 0 {
                Vec::new()
            } else {
                vec![CostlessState {
                    col: cstate.col - 1,
                    num_steps: cstate.num_steps + 1,
                    ..*cstate
                }]
            }
        }
        Dir::R => {
            if cstate.col == num_cols - 1 {
                Vec::new()
            } else {
                vec![CostlessState {
                    col: cstate.col + 1,
                    num_steps: cstate.num_steps + 1,
                    ..*cstate
                }]
            }
        }
    }
}

fn turn_90(d: Dir) -> Vec<Dir> {
    if [Dir::U, Dir::D].contains(&d) {
        vec![Dir::L, Dir::R]
    } else {
        vec![Dir::U, Dir::D]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    cstate: CostlessState,
    cost_to_here: u32,
    est_path_cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip sign so that it's a min-heap
        other
            .est_path_cost
            .cmp(&self.est_path_cost)
            .then_with(|| self.cstate.cmp(&other.cstate))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct CostlessState {
    row: usize,
    col: usize,
    d: Dir,
    num_steps: u32,
}

impl Ord for CostlessState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row
            .cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.d.cmp(&other.d))
            .then_with(|| self.num_steps.cmp(&other.num_steps))
    }
}

impl PartialOrd for CostlessState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
