use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// longer than 2566
// shorter than num dots (however many that is)
// 7010 is also wrong aaaaaaaaaaaaaaaaaaargh

type Coord = (usize, usize);

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect::<Vec<_>>());
    }
    let num_rows = map.len();
    let num_cols = map[0].len();
    let start_coord: Coord = (0, 1);
    let end_coord: Coord = (num_rows - 1, num_cols - 2);
    // println!("end coord {:?}", end_coord);
    let (collapsed_map, new_end_coord, weight) = collapse_map(start_coord, end_coord, &map);
    // println!("collapsed map: {:?}", collapsed_map);
    println!("collapsed map. Number of entries: {}", collapsed_map.len());
    // println!("collapsed map keys: {:?}", collapsed_map.keys().collect::<Vec<_>>());
    // println!("new end coord {:?}", new_end_coord);
    println!("weight from new end to real end: {}", weight);
    // println!("collapsed map {:?}", collapsed_map);
    // funky_dfs(start_coord, new_end_coord, &collapsed_map) + weight
    weight
}

fn collapse_map(
    start_coord: Coord,
    end_coord: Coord,
    map: &Vec<Vec<char>>,
) -> (HashMap<Coord, Vec<(Coord, u64)>>, Coord, u64) {
    // graph is going to be "from this point, you can get to these points with these costs"
    let mut explored_parents = HashMap::new();
    let mut fringe = VecDeque::from([start_coord]);
    let mut graph = HashMap::new();
    let mut explored_coords = HashSet::new();
    let mut new_end_coord = end_coord;
    let mut end_weight = 0;
    while let Some(coord) = fringe.pop_front() {
        // println!("popping {:?} in collapse_map", coord);
        if !explored_coords.contains(&coord) && coord != end_coord {
            let next_points_and_weights =
                get_next_points_and_weights(coord, map, end_coord, &explored_parents);
            for (point, parent, weight) in &next_points_and_weights {
                fringe.push_back(*point);
                explored_parents
                    .entry(*point)
                    .and_modify(|ps| {
                        ps.insert(*parent);
                    })
                    .or_insert(HashSet::from([*parent]));
                graph
                    .entry(*point)
                    .and_modify(|ends_vec: &mut Vec<(Coord, u64)>| {
                        if !ends_vec.contains(&(coord, *weight)) {
                            ends_vec.push((coord, *weight));
                        }
                    })
                    .or_insert(vec![(coord, *weight)]);
                if *point == end_coord {
                    new_end_coord = coord;
                    end_weight = *weight;
                }
            }
            let filtered_next_points_weights = next_points_and_weights
                .iter()
                .map(|(p, _, w)| (*p, *w))
                .collect::<Vec<_>>();
            graph
                .entry(coord)
                .and_modify(|ends_vec| {
                    for (p, w) in filtered_next_points_weights.iter() {
                        if !ends_vec.contains(&(*p, *w)) {
                            ends_vec.push((*p, *w));
                        }
                    }})
                .or_insert(filtered_next_points_weights);
            explored_coords.insert(coord);
        }
    }
    (graph, new_end_coord, end_weight)
}

fn get_next_points_and_weights(
    coord: Coord,
    map: &Vec<Vec<char>>,
    end_coord: Coord,
    explored_parents: &HashMap<Coord, HashSet<Coord>>,
) -> Vec<(Coord, Coord, u64)> {
    let succs = successors_2(coord, map);
    let mut fringe = VecDeque::new();
    for succ in succs {
        if let Some(parents) = explored_parents.get(&coord) {
            if !parents.contains(&succ) {
                // println!("adding {:?} to start of queue", succ);
                fringe.push_back((succ, coord, 1))
            }
        }
        // println!("adding {:?} to start of queue", succ);
        fringe.push_back((succ, coord, 1));
    }
    let mut return_vec = Vec::new();
    while let Some((c, parent, w)) = fringe.pop_front() {
        // println!("popping {:?} in get_next_coords map", c);
        let successors = successors_2(c, map);
        // println!("successors: {:?}", successors);
        let filtered_succs = successors
            .iter()
            .filter(|succ| parent != **succ)
            .collect::<Vec<_>>();
        // println!("filtered successors: {:?}", filtered_succs);
        if c == end_coord || filtered_succs.len() > 1 {
            return_vec.push((c, parent, w));
        } else if filtered_succs.len() == 1 {
            fringe.push_back((*filtered_succs[0], c, w + 1));
        }
    }
    return_vec
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct SearchState2 {
    coord: Coord,
    path_to: HashSet<Coord>,
    cost_to: u64,
}

fn funky_dfs(
    start_coord: Coord,
    end_coord: Coord,
    graph: &HashMap<Coord, Vec<(Coord, u64)>>,
) -> u64 {
    let start_state = SearchState2 {
        coord: start_coord,
        path_to: HashSet::new(),
        cost_to: 0,
    };
    let mut search_stack = vec![start_state];
    let mut longest_cost = 0;
    while let Some(state) = search_stack.pop() {
        // println!("popping coord {:?}", state.coord);
        let succs = graph.get(&state.coord).unwrap();
        for (succ, w) in succs {
            if *succ == end_coord && w + state.cost_to > longest_cost {
                println!("found path to end of weight {}", w + state.cost_to);
                longest_cost = w + state.cost_to;
            } else if !state.path_to.contains(succ) {
                let mut new_path_to = state.path_to.clone();
                new_path_to.insert(state.coord);
                let new_state = SearchState2 {
                    coord: *succ,
                    path_to: new_path_to,
                    cost_to: w + state.cost_to,
                };
                search_stack.push(new_state);
            }
        }
    }
    longest_cost
}

fn funky_bfs_2(
    start_coord: Coord,
    end_coord: Coord,
    graph: &HashMap<Coord, Vec<(Coord, u64)>>,
) -> u64 {
    let start_state = SearchState2 {
        coord: start_coord,
        path_to: HashSet::new(),
        cost_to: 0,
    };
    // let mut cache = HashSet::new();
    let mut search_queue = VecDeque::from([start_state]);
    let mut longest_walk = 0;
    while let Some(state) = search_queue.pop_front() {
        // println!("popping coord {:?}", state.coord);
        if state.coord == end_coord && state.cost_to > longest_walk {
            println!("found newest longest path: {}", state.cost_to);
            longest_walk = state.cost_to;
        }
        if !state.path_to.contains(&state.coord) {
            let mut new_path_to = state.path_to.clone();
            new_path_to.insert(state.coord);
            let succs = graph.get(&state.coord).unwrap();
            for (succ, w) in succs {
                if !new_path_to.contains(succ) {
                    let new_state = SearchState2 {
                        coord: *succ,
                        path_to: new_path_to.clone(),
                        cost_to: w + state.cost_to,
                    };
                    search_queue.push_back(new_state);
                }
            }
        }
    }
    longest_walk
}

fn successors_2(coord: Coord, map: &Vec<Vec<char>>) -> Vec<Coord> {
    let r = coord.0;
    let c = coord.1;
    let num_rows = map.len();
    let num_cols = map[0].len();
    let mut candidate_next = Vec::new();
    if r > 0 {
        candidate_next.push((r - 1, c));
    }
    if r < num_rows - 1 {
        candidate_next.push((r + 1, c));
    }
    if c > 0 {
        candidate_next.push((r, c - 1));
    }
    if c < num_cols - 1 {
        candidate_next.push((r, c + 1));
    }
    candidate_next
        .iter()
        .filter(|(nr, nc)| map[*nr][*nc] != '#')
        .copied()
        .collect::<Vec<_>>()
}

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect::<Vec<_>>());
    }
    let num_rows = map.len();
    let num_cols = map[0].len();
    let start_coord: Coord = (0, 1);
    let end_coord: Coord = (num_rows - 1, num_cols - 2);
    funky_bfs(start_coord, end_coord, &map)
}

// do I need the vec?
#[derive(PartialEq, Eq, Clone, Debug)]
struct SearchState {
    coord: Coord,
    path_to: HashSet<Coord>,
}

fn funky_bfs(start_coord: Coord, end_coord: Coord, map: &Vec<Vec<char>>) -> usize {
    let mut search_queue = VecDeque::new();
    let start_state = SearchState {
        coord: start_coord,
        path_to: HashSet::new(),
    };
    search_queue.push_back(start_state);
    let mut path_lens_to_goal: Vec<usize> = Vec::new();
    while let Some(state) = search_queue.pop_front() {
        let current_coord = state.coord;
        // if the state is the goal
        // record that we found one path to the goal
        // at the end we'll pick the longest such path
        if current_coord == end_coord {
            path_lens_to_goal.push(state.path_to.len());
        } else {
            let succs = successors(current_coord, map);
            for succ in succs {
                // if it's not on the path to the current state
                // add it to the end of the queue
                // delete anything in the queue with that coord
                if !state.path_to.contains(&succ) {
                    let mut next_path_to = state.path_to.clone();
                    next_path_to.insert(current_coord);
                    let previous_coord_opt = search_queue.iter().position(|s| s.coord == succ);
                    if let Some(i) = previous_coord_opt {
                        search_queue.remove(i);
                    }
                    search_queue.push_back(SearchState {
                        coord: succ,
                        path_to: next_path_to,
                    });
                }
            }
        }
    }
    *path_lens_to_goal.iter().max().unwrap()
}

fn successors(coord: Coord, map: &Vec<Vec<char>>) -> Vec<Coord> {
    let r = coord.0;
    let c = coord.1;
    let num_rows = map.len();
    let num_cols = map[0].len();
    let candidate_next = match map[r][c] {
        '>' => vec![(r, c + 1)],
        'v' => vec![(r + 1, c)],
        '.' => {
            let mut building_vec = Vec::new();
            if r > 0 {
                building_vec.push((r - 1, c));
            }
            if r < num_rows - 1 {
                building_vec.push((r + 1, c));
            }
            if c > 0 {
                building_vec.push((r, c - 1));
            }
            if c < num_cols - 1 {
                building_vec.push((r, c + 1));
            }
            building_vec
        }
        _ => Vec::new(),
    };
    candidate_next
        .iter()
        .filter(|(nr, nc)| map[*nr][*nc] != '#')
        .copied()
        .collect::<Vec<_>>()
}
