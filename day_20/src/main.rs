use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}

// want to know how the states change after you press the button
// eventually find some sort of loop.
// state of flip flops is on vs off
// state of conjunctions is types of most recent pulse from each input
// read each line
// store type of each module, what it sends stuff to
// for conjunctions, also want to know its vector of inputs

// fuck didn't have any loops...

// ok part 2, let's do this the naive way and see what happens
// naive way doesn't work
// could try to work backwards with requirements
// hmmm but it's tricky because I don't just need to receive a bunch of things from parents,
// I need to have received those things most recently (if I'm a conjunction)
// or to have received those things ever (if I'm a flip-flop)

// ok, i've got a graph, and i have some set of conditions that need to be met
// the graph is loopy and the loops are in the middle
// if there weren't loops, i could maybe back-propagate conditions
// for & nodes, conditions are about the most recent pulses from parents.
// for % nodes, conditions are "you need to have had an even / odd number of low pulses ever"
// then for % parents of % nodes, it's about the number of low pulses mod 4 ever received
// for & parents of % nodes, it's "you need to have an even/odd number of times where all your parents sent high pulses"

// ok: looks like you've got these four & nodes, and all their ancestors are either % nodes
// or the & nodes themselves
// and it looks like these are disjoint units

fn part_2(file_path: &str) -> u64 {
    // solved by hand
    let base: u64 = 2;
    let gk_ins: Vec<u64> = vec![0, 2, 3, 6, 8, 9, 10, 11];
    let gk_start = sum_exp(&gk_ins);
    let gk_outs: Vec<u64> = vec![0, 1, 4, 5, 7];
    let gk_reset = sum_exp(&receive_pulse(&gk_ins, gk_outs));
    let gk_loop_period = gk_start - gk_reset;
    let gx_ins: Vec<u64> = vec![0, 1, 4, 6, 8, 9, 10, 11];
    let gx_start = sum_exp(&gx_ins);
    let gx_outs: Vec<u64> = vec![0, 2, 3, 5, 7];
    let gx_reset = sum_exp(&receive_pulse(&gx_ins, gx_outs));
    let gx_loop_period = gx_start - gx_reset;
    let xr_ins: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
    let xr_start = sum_exp(&xr_ins);
    let xr_outs: Vec<u64> = vec![0, 7];
    let xr_loop_period = xr_start - sum_exp(&receive_pulse(&xr_ins, xr_outs));
    let tf_ins: Vec<u64> = vec![0, 2, 4, 5, 7, 8, 9, 10, 11];
    let tf_start = sum_exp(&tf_ins);
    let tf_outs: Vec<u64> = vec![0, 1, 3, 6];
    let tf_loop_period = tf_start - sum_exp(&receive_pulse(&tf_ins, tf_outs));
    let a_vec = vec![gk_start, gx_start, xr_start, tf_start];
    let n_vec = vec![
        gk_loop_period,
        gx_loop_period,
        xr_loop_period,
        tf_loop_period,
    ];
    println!("as {:?} ns {:?}", a_vec, n_vec);
    a_vec.iter().product::<u64>()
    // solve_crt(a_vec, &n_vec)
}

fn solve_crt(a_vec: Vec<u64>, n_vec: &[u64]) -> u64 {
    if a_vec.len() == 1 {
        a_vec[0]
    } else {
        let a1 = a_vec[0] as i64;
        let a2 = a_vec[1] as i64;
        let n1 = n_vec[0] as i64;
        let n2 = n_vec[1] as i64;
        // bezout: m1n1 + m2n2 = 1
        let (m1, m2) = extended_gcd(n1, n2);
        let first_term = (a1 * m2) % (n1 * n2);
        let second_term = (a2 * m1) % (n1 * n2);
        let part_1 = (first_term * n2) % (n1 * n2);
        let part_2 = (second_term * n1) % (n1 * n2);
        let a12 = (part_1 + part_2) as u64;
        let n12 = (n1 * n2) as u64;
        let mut new_a_vec = vec![a12];
        new_a_vec.append(&mut a_vec[2..].to_vec());
        let mut new_n_vec = vec![n12];
        new_n_vec.append(&mut n_vec[2..].to_vec());
        solve_crt(new_a_vec, &new_n_vec)
    }
}

fn extended_gcd(n1: i64, n2: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (n1, n2);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    (old_s, old_t)
}

fn sum_exp(v: &[u64]) -> u64 {
    let base: u64 = 2;
    let mut accum = 0;
    for x in v {
        accum += base.pow(*x as u32);
    }
    accum
}

fn receive_pulse(term_ins: &[u64], term_outs: Vec<u64>) -> Vec<u64> {
    // term_ins are all on
    // toggle term_outs, change things accordingly
    let mut new_ins = term_ins.to_vec();
    let mut perturbed = VecDeque::from(term_outs);
    while let Some(i) = perturbed.pop_front() {
        if let Some(j) = new_ins.iter().position(|&x| x == i) {
            new_ins.swap_remove(j);
            if i + 1 < 12 {
                perturbed.push_back(i + 1);
            }
        } else {
            new_ins.push(i);
        }
    }
    new_ins
}

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let num_pushes: usize = 4095; // should be 1000
                                  // first vec is things that you send to, second vec is things that send to you
    let mut modules: HashMap<&str, (ModuleState, Vec<&str>, Vec<&str>)> = HashMap::new();
    for line in lines {
        let mut parts = line.split(" -> ");
        let first_part = parts.next().unwrap();
        let part_type = match &first_part[0..1] {
            "%" => ModuleState::FlipFlop(false),
            "&" => ModuleState::Conjunction(Vec::new()),
            _ => ModuleState::Broadcast,
        };
        let part_name = first_part.trim_start_matches(|c: char| ['&', '%'].contains(&c));
        let second_part = parts.next().unwrap();
        let mut sends_to = Vec::new();
        for recipient_name in second_part.split(", ") {
            if let Some((m, _, receives_from)) = modules.get_mut(&recipient_name) {
                receives_from.push(part_name);
                if let ModuleState::Conjunction(state_vec) = m {
                    state_vec.push(false);
                }
            } else {
                modules.insert(
                    recipient_name,
                    (ModuleState::Broadcast, Vec::new(), vec![part_name]),
                );
            }
            sends_to.push(recipient_name);
        }
        modules
            .entry(part_name)
            .and_modify(|(m, i_send_to, receives_from)| {
                let num_receives = receives_from.len();
                if let ModuleState::Conjunction(_) = part_type {
                    let state_vec = vec![false; num_receives];
                    *m = ModuleState::Conjunction(state_vec);
                    *i_send_to = sends_to.clone();
                } else {
                    *m = part_type.clone();
                    *i_send_to = sends_to.clone();
                }
            })
            .or_insert((part_type, sends_to, Vec::new()));
    }
    // first entry is low pulses sent, second is high pulses sent
    let mut low_pulse_history: Vec<u64> = Vec::new();
    let mut high_pulse_history: Vec<u64> = Vec::new();
    let mut state_history = vec![modules.clone()];
    for k in 0..(num_pushes + 1) {
        println!("push {}", k);
        let (low_sent, high_sent) = push_button(&mut modules, k == 3922);
        low_pulse_history.push(low_sent);
        high_pulse_history.push(high_sent);
        if let Some(i) = state_history.iter().position(|ms| *ms == modules) {
            let cycle_length = k + 1 - i;
            let pushes_left = num_pushes - k - 1;
            let cycles_left = (pushes_left / cycle_length) as u64;
            let reduced_pushes_left = pushes_left % cycle_length;
            let low_pulses_pre_cycle: u64 = low_pulse_history[..i].iter().sum();
            let high_pulses_pre_cycle: u64 = high_pulse_history[..i].iter().sum();
            let low_pulses_in_cycle: u64 = low_pulse_history[i..].iter().sum();
            let high_pulses_in_cycle: u64 = high_pulse_history[i..].iter().sum();
            let low_pulses_post_cycle: u64 =
                low_pulse_history[i..(i + reduced_pushes_left)].iter().sum();
            let high_pulses_post_cycle: u64 = high_pulse_history[i..(i + reduced_pushes_left)]
                .iter()
                .sum();
            println!(
                "k {}, i {}, cycle_length {}, pushes_left {}, cycles_left {}",
                k, i, cycle_length, pushes_left, cycles_left
            );
            println!(
                "low_pre {}, low_in {}, low_post {}",
                low_pulses_pre_cycle, low_pulses_in_cycle, low_pulses_post_cycle
            );
            println!(
                "high_pre {}, high_in {}, high_post {}",
                high_pulses_pre_cycle, high_pulses_in_cycle, high_pulses_post_cycle
            );
            let total_low = low_pulses_pre_cycle
                + (low_pulses_in_cycle * (cycles_left + 1))
                + low_pulses_post_cycle;
            let total_high = high_pulses_pre_cycle
                + (high_pulses_in_cycle * (cycles_left + 1))
                + high_pulses_post_cycle;
            return total_low * total_high;
        } else {
            state_history.push(modules.clone());
        }
    }
    low_pulse_history.iter().sum::<u64>() * high_pulse_history.iter().sum::<u64>()
}

fn push_button<'a>(
    mods: &mut HashMap<&'a str, (ModuleState, Vec<&'a str>, Vec<&'a str>)>,
    verbose: bool,
) -> (u64, u64) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    // will probably have a recursive call in here
    // each node needs to know who sent the thing, what pulse was sent
    // can't do it neatly recursively because things go in rounds
    // need to keep a fifo queue
    let mut pulses_to_process = VecDeque::from([("button", "broadcaster", false)]);
    while let Some((from_pulse, to_pulse, pulse_type)) = pulses_to_process.pop_front() {
        // println!("pulse from {} to {} with value {}", from_pulse, to_pulse, pulse_type);
        if verbose {
            println!(
                "pulse from {} to {} with value {}",
                from_pulse, to_pulse, pulse_type
            );
        }
        if pulse_type {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }
        let (m, sends_to, receives_from) = mods.get_mut(&to_pulse).unwrap();
        match m {
            ModuleState::Broadcast => {
                // println!("broadcast, sending low to {:?}", sends_to);
                for next_name in sends_to {
                    pulses_to_process.push_back((to_pulse, next_name, pulse_type));
                }
            }
            ModuleState::FlipFlop(state_bool) => {
                // println!("flip flop");
                if !pulse_type {
                    *state_bool = !(*state_bool);
                    let propagate_type = *state_bool;
                    if to_pulse == "gd" {
                        println!(
                            "pulse is low, sending new state to {:?} of type {}",
                            sends_to, propagate_type
                        );
                    }
                    for next_name in sends_to {
                        pulses_to_process.push_back((to_pulse, next_name, propagate_type));
                    }
                }
            }
            ModuleState::Conjunction(state_vec) => {
                // println!("conjunction");
                let index_received_from =
                    receives_from.iter().position(|x| *x == from_pulse).unwrap();
                state_vec[index_received_from] = pulse_type;
                let propagate_type = !state_vec.iter().all(|b| *b);
                // println!("sending pulse value {} to {:?}", propagate_type, sends_to);
                for next_name in sends_to {
                    pulses_to_process.push_back((to_pulse, next_name, propagate_type));
                }
            }
        };
    }
    (low_pulses, high_pulses)
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum ModuleState {
    Broadcast,
    FlipFlop(bool),         // true = on, false = off
    Conjunction(Vec<bool>), // true = high, false = low
}
