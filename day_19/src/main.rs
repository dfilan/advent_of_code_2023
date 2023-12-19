use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
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

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    let mut workflows: HashMap<&str, Vec<(Condition, &str)>> = HashMap::new();
    let mut have_seen_break = false;
    while !have_seen_break {
        let line = lines.next().unwrap();
        if line.is_empty() {
            have_seen_break = true;
        } else {
            let mut segments = line.split(&['{', '}', ',']);
            let rule_name = segments.next().unwrap();
            let process: Vec<(Condition, &str)> = segments
                .filter(|seg| !seg.is_empty())
                .map(turn_to_rule)
                .collect();
            workflows.insert(rule_name, process);
        }
    }
    // so: could just branch thru-out the whole thing
    // I think this is actually doable
    // OK: start at the end rules, say what's doable there
    // iterate thru map, taking things with rules that I've already covered
    // let mut accum = 0;
    let mut rule_ranges: HashMap<&str, HashSet<PartRange>> = HashMap::new();
    rule_ranges.insert(
        "A",
        HashSet::from_iter(
            [PartRange {
                x: (1, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000),
            }]
            .iter()
            .cloned(),
        ),
    );
    rule_ranges.insert("R", HashSet::new());
    while rule_ranges.len() < workflows.len() + 2 {
        let (name, process) = workflows
            .iter()
            .find(|&(k, v)| rule_ranges.get(k).is_none() && touched_all_leaves(v, &rule_ranges))
            .unwrap();
        // println!("rule_name: {:?}", name);
        let valid_ranges = get_ranges(
            PartRange {
                x: (1, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000),
            },
            &process[..],
            &rule_ranges,
        );
        // for r in valid_ranges.iter() {
        //     println!("adding valid range {:?}", r);
        // }
        rule_ranges.insert(name, valid_ranges);
    }
    let in_ranges = rule_ranges.get(&"in").unwrap();
    num_in_ranges(in_ranges)
}

fn num_in_ranges(rs: &HashSet<PartRange>) -> u64 {
    let mut accum = 0;
    for r in rs.iter() {
        // println!("r: {:?}", r);
        accum +=
            (r.x.1 + 1 - r.x.0) * (r.m.1 + 1 - r.m.0) * (r.a.1 + 1 - r.a.0) * (r.s.1 + 1 - r.s.0);
    }
    accum
}

fn get_ranges(
    current_range: PartRange,
    process: &[(Condition, &str)],
    rule_ranges: &HashMap<&str, HashSet<PartRange>>,
) -> HashSet<PartRange> {
    if process.is_empty() {
        let mut return_set = HashSet::new();
        return_set.insert(current_range);
        return_set
    } else {
        let (cond, next) = process[0];
        let this_rule_next_ranges = rule_ranges.get(&next).unwrap();
        match cond {
            Condition::NoCond => combine_ranges(current_range, this_rule_next_ranges),
            Condition::Cond(field, ord, cutoff) => {
                let (opt_this_range, opt_next_range) =
                    split_range(current_range, field, ord, cutoff);
                let mut return_set = HashSet::new();
                if let Some(this_range) = opt_this_range {
                    let this_rule_ranges = combine_ranges(this_range, this_rule_next_ranges);
                    return_set.extend(this_rule_ranges);
                }
                if let Some(next_range) = opt_next_range {
                    let next_rules_ranges = get_ranges(next_range, &process[1..], rule_ranges);
                    return_set.extend(next_rules_ranges);
                }
                return_set
            }
        }
    }
}

fn split_range(
    range: PartRange,
    field: PartField,
    ord: Ordering,
    cutoff: u64,
) -> (Option<PartRange>, Option<PartRange>) {
    let cut_range = get_field_range(field, &range);
    // println!("rule: {:?}, {:?}, {}", field, ord, cutoff);
    match ord {
        Ordering::Less => {
            if cutoff > cut_range.1 {
                (Some(range), None)
            } else if cutoff <= cut_range.0 {
                (None, Some(range))
            } else {
                let low_range = (cut_range.0, cutoff - 1);
                let high_range = (cutoff, cut_range.1);
                match field {
                    PartField::X => (
                        Some(PartRange {
                            x: low_range,
                            ..range
                        }),
                        Some(PartRange {
                            x: high_range,
                            ..range
                        }),
                    ),
                    PartField::M => (
                        Some(PartRange {
                            m: low_range,
                            ..range
                        }),
                        Some(PartRange {
                            m: high_range,
                            ..range
                        }),
                    ),
                    PartField::A => (
                        Some(PartRange {
                            a: low_range,
                            ..range
                        }),
                        Some(PartRange {
                            a: high_range,
                            ..range
                        }),
                    ),
                    PartField::S => (
                        Some(PartRange {
                            s: low_range,
                            ..range
                        }),
                        Some(PartRange {
                            s: high_range,
                            ..range
                        }),
                    ),
                }
            }
        }
        Ordering::Greater => {
            if cutoff < cut_range.0 {
                (Some(range), None)
            } else if cutoff >= cut_range.1 {
                (None, Some(range))
            } else {
                let low_range = (cut_range.0, cutoff);
                let high_range = (cutoff + 1, cut_range.1);
                match field {
                    PartField::X => (
                        Some(PartRange {
                            x: high_range,
                            ..range
                        }),
                        Some(PartRange {
                            x: low_range,
                            ..range
                        }),
                    ),
                    PartField::M => (
                        Some(PartRange {
                            m: high_range,
                            ..range
                        }),
                        Some(PartRange {
                            m: low_range,
                            ..range
                        }),
                    ),
                    PartField::A => (
                        Some(PartRange {
                            a: high_range,
                            ..range
                        }),
                        Some(PartRange {
                            a: low_range,
                            ..range
                        }),
                    ),
                    PartField::S => (
                        Some(PartRange {
                            s: high_range,
                            ..range
                        }),
                        Some(PartRange {
                            s: low_range,
                            ..range
                        }),
                    ),
                }
            }
        }
        Ordering::Equal => (None, None),
    }
}

fn get_field_range(field: PartField, part_range: &PartRange) -> (u64, u64) {
    match field {
        PartField::X => part_range.x,
        PartField::M => part_range.m,
        PartField::A => part_range.a,
        PartField::S => part_range.s,
    }
}

fn combine_ranges(range: PartRange, ranges: &HashSet<PartRange>) -> HashSet<PartRange> {
    if invalid_range(&range) {
        HashSet::new()
    } else {
        ranges
            .iter()
            .map(|r| intersect_ranges(r, &range))
            .filter(|r| !invalid_range(r))
            .collect::<HashSet<PartRange>>()
    }
}

fn intersect_ranges(range1: &PartRange, range2: &PartRange) -> PartRange {
    PartRange {
        x: (max(range1.x.0, range2.x.0), min(range1.x.1, range2.x.1)),
        m: (max(range1.m.0, range2.m.0), min(range1.m.1, range2.m.1)),
        a: (max(range1.a.0, range2.a.0), min(range1.a.1, range2.a.1)),
        s: (max(range1.s.0, range2.s.0), min(range1.s.1, range2.s.1)),
    }
}

fn touched_all_leaves(
    process: &[(Condition, &str)],
    rule_ranges: &HashMap<&str, HashSet<PartRange>>,
) -> bool {
    process.iter().all(|&(_, n)| rule_ranges.get(&n).is_some())
}

fn invalid_range(p: &PartRange) -> bool {
    p.x.0 > p.x.1 || p.m.0 > p.m.1 || p.a.0 > p.a.1 || p.s.0 > p.s.1
}

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut workflows: HashMap<&str, Vec<(Condition, &str)>> = HashMap::new();
    let mut accum = 0;
    let mut have_seen_break = false;
    for line in lines {
        if line.is_empty() {
            have_seen_break = true;
        } else if !have_seen_break {
            let mut segments = line.split(&['{', '}', ',']);
            let rule_name = segments.next().unwrap();
            let process: Vec<(Condition, &str)> = segments
                .filter(|seg| !seg.is_empty())
                .map(turn_to_rule)
                .collect();
            workflows.insert(rule_name, process);
        } else {
            let segments = line.split(&['{', '}', ',']);
            let values: Vec<u64> = segments
                .filter(|s| !s.is_empty())
                .map(|s| s[2..].parse::<u64>().unwrap())
                .collect();
            let weight: u64 = values.iter().sum();
            let part = Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            };
            let dest = get_dest(&part, &workflows);
            if dest == "A" {
                accum += weight;
            }
        }
    }
    accum
}

fn turn_to_rule(rule_thing: &str) -> (Condition, &str) {
    if rule_thing.contains(':') {
        let mut halves = rule_thing.split(':');
        let fore_part = halves.next().unwrap();
        let field = &fore_part[0..1];
        let part_field = if field == "x" {
            PartField::X
        } else if field == "m" {
            PartField::M
        } else if field == "a" {
            PartField::A
        } else {
            PartField::S
        };
        let order_sign = fore_part[1..].trim_end_matches(char::is_numeric);
        let order = if order_sign == "<" {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        let number_part = &fore_part[2..];
        let number: u64 = number_part.parse().unwrap();
        let aft_part = halves.next().unwrap();
        (Condition::Cond(part_field, order, number), aft_part)
    } else {
        (Condition::NoCond, rule_thing)
    }
}

fn get_dest<'a>(
    part: &'a Part,
    workflows: &'a HashMap<&'a str, Vec<(Condition, &'a str)>>,
) -> &'a str {
    let mut rule_name = "in";
    while !["R", "A"].contains(&rule_name) {
        let rule = workflows.get(&rule_name).unwrap();
        rule_name = get_next_rule(part, rule);
    }
    rule_name
}

fn get_next_rule<'a>(part: &'a Part, rule: &'a [(Condition, &'a str)]) -> &'a str {
    for seg in rule {
        match seg.0 {
            Condition::NoCond => {
                return seg.1;
            }
            Condition::Cond(field, ord, num) => {
                let part_val = get_field_val(field, part);
                if part_val.cmp(&num) == ord {
                    return seg.1;
                }
            }
        }
    }
    // should never get here
    "R"
}

fn get_field_val(field: PartField, part: &Part) -> u64 {
    match field {
        PartField::X => part.x,
        PartField::M => part.m,
        PartField::A => part.a,
        PartField::S => part.s,
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct PartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PartField {
    X,
    M,
    A,
    S,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Condition {
    NoCond,
    Cond(PartField, Ordering, u64),
}
