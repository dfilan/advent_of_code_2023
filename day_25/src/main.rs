use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

use rand::{rngs, thread_rng, Rng};

fn main() {
    let start = Instant::now();
    let true_min_cut = 3;
    let result = part_1(true_min_cut, "./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// connections aren't directional
// 3 wires that divide components into two separate groups
// multiply size of groups together
// this is a min cut problem

// input graph has 1527 nodes

fn part_1(true_min_cut: u64, file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut graph: HashMap<Vec<&str>, Vec<(u64, Vec<&str>)>> = HashMap::new();
    for line in lines {
        let mut halves = line.split(": ");
        let lhs_name = halves.next().unwrap();
        let rhs_components = halves.next().unwrap().split(' ').collect::<Vec<_>>();
        for component in rhs_components {
            graph
                .entry(vec![lhs_name])
                .and_modify(|v| {
                    if !v.contains(&(1, vec![component])) {
                        v.push((1, vec![component]));
                    }
                })
                .or_insert(vec![(1, vec![component])]);
            graph
                .entry(vec![component])
                .and_modify(|v| {
                    if !v.contains(&(1, vec![lhs_name])) {
                        v.push((1, vec![lhs_name]));
                    }
                })
                .or_insert(vec![(1, vec![lhs_name])]);
        }
    }
    println!("graph length: {}", graph.len());
    let mut rng = thread_rng();
    let mut min_cut = 5;
    let mut product_sizes = 0;
    while min_cut > true_min_cut {
        let (product, cut_val) = karger_stein(&graph, true_min_cut, &mut rng);
        println!("cut val: {}", cut_val);
        min_cut = cut_val;
        product_sizes = product;
    }
    product_sizes
}

// should I be memoizing?
// no, there's a gazillion possible shrunken graphs

fn karger_stein(
    graph: &HashMap<Vec<&str>, Vec<(u64, Vec<&str>)>>,
    min_cut: u64,
    rng: &mut rngs::ThreadRng,
) -> (usize, u64) {
    let num_vertices = graph.len() as f64;
    if num_vertices > 50.0 {
        println!("karger_stein call with num vertices {}", num_vertices);
    }
    if num_vertices > 6.0 {
        let sqrt_2 = f64::sqrt(2.0);
        let t = (1.0 + (num_vertices / sqrt_2).ceil()) as usize;
        let graph_1 = contract(t, graph, rng);
        let graph_2 = contract(t, graph, rng);
        let (p1, cut1) = karger_stein(&graph_1, min_cut, rng);
        if cut1 == min_cut {
            return (p1, cut1);
        }
        let (p2, cut2) = karger_stein(&graph_2, min_cut, rng);
        if cut1 <= cut2 {
            return (p1, cut1);
        } else {
            return (p2, cut2);
        }
    }
    let final_graph = contract(2, graph, rng);
    let mut product = 1;
    let mut min_cut = 0;
    for (k, v) in final_graph.iter() {
        product *= k.len();
        min_cut = v.iter().map(|(n, _)| *n).sum::<u64>();
    }
    (product, min_cut)
}

fn contract<'a>(
    t: usize,
    graph: &'a HashMap<Vec<&'a str>, Vec<(u64, Vec<&'a str>)>>,
    rng: &mut rngs::ThreadRng,
) -> HashMap<Vec<&'a str>, Vec<(u64, Vec<&'a str>)>> {
    // pick a random edge
    // contract the edge
    // recursively call yourself
    // if the graph has two nodes, return the product of their sizes + num edges between them
    let mut mutant_graph = graph.clone();
    while mutant_graph.len() > t {
        // pick a random edge
        let mut num_edges = 0;
        for (w, vec) in mutant_graph.iter() {
            // println!("node: {:?}", w);
            // println!("edges: {:?}", vec);
            num_edges += vec.iter().map(|(n, _)| *n).sum::<u64>();
        }
        num_edges /= 2;
        let random_edge: u64 = rng.gen_range(0..num_edges);
        // println!("random edge: {}", random_edge);
        let mut seen_edges: HashSet<(Vec<&str>, Vec<&str>)> = HashSet::new();
        let mut s1: Vec<&str> = Vec::new();
        let mut s2: Vec<&str> = Vec::new();
        let mut counter = 0;
        'find_edge: for (s, vec) in mutant_graph.iter() {
            let edges_to_check_iter = vec
                .iter()
                .filter(|(_, t)| !seen_edges.contains(&(t.clone(), s.clone())));
            let num_edges = edges_to_check_iter.clone().map(|(n, _)| n).sum::<u64>();
            if counter <= random_edge && counter + num_edges > random_edge {
                for (n, t) in edges_to_check_iter.collect::<Vec<_>>() {
                    // println!("counter: {}", counter);
                    // println!("n: {}", n);
                    if counter <= random_edge && counter + n > random_edge {
                        s1 = s.clone();
                        s2 = t.clone();
                        break 'find_edge;
                    } else {
                        counter += n;
                        seen_edges.insert((s.clone(), t.clone()));
                    }
                }
            } else {
                counter += num_edges;
                for (_, t) in edges_to_check_iter.collect::<Vec<_>>() {
                    seen_edges.insert((s.clone(), t.clone()));
                }
            }
        }
        // contract the edge
        let mut new_vertex = s1.clone();
        let s2_new = s2.clone();
        new_vertex.append(&mut s2);
        // println!("s1: {:?}", s1);
        let s1_connections = mutant_graph.get(&s1).unwrap();
        let s2_connections = mutant_graph.get(&s2_new).unwrap();
        let mut edge_connections = Vec::new();
        for (n, other_vertex) in s1_connections {
            if *other_vertex != s2_new {
                edge_connections.push((*n, other_vertex.clone()));
            }
        }
        for (n, other_vertex) in s2_connections {
            if *other_vertex != s1 {
                edge_connections.push((*n, other_vertex.clone()));
            }
        }
        edge_connections = dedup_edge_list(edge_connections);
        for (_, other_vertex) in edge_connections.clone() {
            mutant_graph.entry(other_vertex).and_modify(|v_list| {
                let opt_i = v_list.iter().position(|(_, v)| *v == s1);
                if let Some(i) = opt_i {
                    v_list[i].1 = new_vertex.clone();
                }
                let opt_j = v_list.iter().position(|(_, v)| *v == s2_new);
                if let Some(j) = opt_j {
                    v_list[j].1 = new_vertex.clone();
                }
                *v_list = dedup_edge_list(v_list.clone());
            });
        }
        mutant_graph.remove(&s1);
        mutant_graph.remove(&s2_new);
        mutant_graph.insert(new_vertex, edge_connections);
    }
    mutant_graph
}

fn dedup_edge_list(edges: Vec<(u64, Vec<&str>)>) -> Vec<(u64, Vec<&str>)> {
    let mut new_list = Vec::new();
    for (n, v) in edges {
        if let Some(i) = new_list.iter().map(|(_, w)| w).position(|w| *w == v) {
            let (existing_num, _) = new_list[i];
            new_list[i] = (n + existing_num, v);
        } else {
            new_list.push((n, v));
        }
    }
    new_list
}
