use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::time::Instant;

use num_integer::binomial;

fn main() {
    let start = Instant::now();
    let result = part_2_better("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed)
}

// . operational
// # damaged
// ? unknown
// num is damaged
// how many arrangements fit criteria
// sum over lines

// this solution takes 2.77 sec

fn part_2_better(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let spring_records: Vec<(Vec<char>, Vec<u64>)> = lines.map(parse_line).collect();
    let num_repeat = 5; // should be 5
    let mut accum = 0;
    let mut num_sol_memo_map: HashMap<(Vec<char>, Vec<u64>), u64> = HashMap::new();
    for (i, mut record) in spring_records.into_iter().enumerate() {
        println!("record number {} is {:?}", i, record);
        // multiply records by five
        let num_chars = record.0.len();
        record.0.push('?');
        let char_vec: Vec<char> = record
            .0
            .into_iter()
            .cycle()
            .take((num_chars + 1) * num_repeat - 1)
            .collect();
        let num_count = record.1.len();
        let count_vec: Vec<u64> = record
            .1
            .into_iter()
            .cycle()
            .take(num_count * num_repeat)
            .collect();
        // get solutions to this record
        let record_sols = count_sols(&char_vec, &count_vec, &mut num_sol_memo_map);
        println!("num solutions to record {}", record_sols);
        accum += record_sols;
    }
    accum
}

fn count_sols(
    char_vec: &Vec<char>,
    count_vec: &Vec<u64>,
    memo_map: &mut HashMap<(Vec<char>, Vec<u64>), u64>,
) -> u64 {
    if let Some(num) = memo_map.get(&(char_vec.clone(), count_vec.clone())) {
        *num
    } else {
        let result = {
            if count_vec.is_empty() {
                if char_vec.contains(&'#') {
                    0
                } else {
                    1
                }
            } else if char_vec.is_empty() {
                0
            } else {
                match char_vec[0] {
                    '#' => count_sols_pound(char_vec, count_vec, memo_map),
                    '.' => count_sols_dot(char_vec, count_vec, memo_map),
                    _ => {
                        count_sols_pound(char_vec, count_vec, memo_map)
                            + count_sols_dot(char_vec, count_vec, memo_map)
                    }
                }
            }
        };
        memo_map.insert((char_vec.clone(), count_vec.clone()), result);
        result
    }
}

fn count_sols_dot(
    char_vec: &Vec<char>,
    count_vec: &Vec<u64>,
    memo_map: &mut HashMap<(Vec<char>, Vec<u64>), u64>,
) -> u64 {
    count_sols(&char_vec[1..].to_vec(), count_vec, memo_map)
}

fn count_sols_pound(
    char_vec: &Vec<char>,
    count_vec: &Vec<u64>,
    memo_map: &mut HashMap<(Vec<char>, Vec<u64>), u64>,
) -> u64 {
    let first_count: usize = (*count_vec.first().unwrap()).try_into().unwrap();
    // had better be able to treat all the first count characters as hashes
    if char_vec.len() < first_count {
        0
    } else if char_vec[..first_count].contains(&'.') {
        0
    } else if char_vec.len() == first_count {
        // can only fit one solution
        if count_vec.len() == 1 { 1 } else { 0 }
    } else if char_vec[first_count] == '#' {
        // the next character better be a potential dot
        0
    } else {
        // split off first_count + 1 elements of char_vec, pop off first count.
        count_sols(
            &char_vec[(first_count + 1)..].to_vec(),
            &count_vec[1..].to_vec(),
            memo_map,
        )
    }
}

// what I originally wrote
// takes 550 sec to run on input

fn part_2(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let spring_records: Vec<(Vec<char>, Vec<u64>)> = lines.map(parse_line).collect();
    let num_repeat = 5; // should be 5
    let mut accum = 0;
    let mut num_sol_memo_map: HashMap<(Vec<char>, Vec<u64>, bool), u64> = HashMap::new();
    for (i, mut record) in spring_records.into_iter().enumerate() {
        println!("record number {} is {:?}", i, record);
        // multiply records by five
        let num_chars = record.0.len();
        record.0.push('?');
        let char_vec: Vec<char> = record
            .0
            .into_iter()
            .cycle()
            .take((num_chars + 1) * num_repeat - 1)
            .collect();
        let num_count = record.1.len();
        let count_vec: Vec<u64> = record
            .1
            .into_iter()
            .cycle()
            .take(num_count * num_repeat)
            .collect();
        // split the record at the dots
        let char_subproblems: Vec<Vec<char>> = char_vec
            .split(|&c| c == '.')
            .filter(|v| !v.contains(&'.'))
            .map(|v| v.to_vec())
            .collect();
        // think about solutions without dots
        // so first I've got to iterate over ways of splitting up the count vec
        // that are compatible with that
        // take a sum over the solutions below
        let mut record_sols = 0;
        for count_vecs in get_count_groups(&char_subproblems, &count_vec) {
            // println!("count_vecs {:?}", count_vecs);
            // take the product of the sub-solutions below
            let mut group_solutions = 1;
            for (subproblem, sub_count_vec) in zip(char_subproblems.iter(), count_vecs.iter()) {
                // you've got a count vec like (1,2,4) (or maybe (7) or ())
                // and you've got a subproblem like '#?????##?????#???#?'
                // so now, you want to do the same sort of things with the gaps:
                // split up the subproblem, try to fit the gaps into the sub-sub-problems
                // aargh, but note that the lengths in the count vec constrain heavily what
                // where we can put gaps
                // and there are going to be a but-ton of feasible gaps, ignoring the hashes
                // so don't want to iterate over all of those - somehow, you only want to iterate
                // over the actually feasible gaps
                // one thing you could do is stop once your gap prefix is infeasible
                // and hope that ends things soon enough
                // you can also be sure to only put gaps where there are question marks
                // so that would mean infeasibility gets cashed out as "the thing right after
                // a set of hashes is itself a hash"
                // or "the final gap contains hashes"
                // big question: is there a smarter way?
                // let's do the dumb way first
                let subproblem_sols =
                    num_subproblem_sols(subproblem, sub_count_vec, false, &mut num_sol_memo_map);
                group_solutions *= subproblem_sols;
            }
            record_sols += group_solutions;
        }
        println!("num solutions to record {}", record_sols);
        accum += record_sols;
    }
    accum
}

fn get_count_groups(char_subproblems: &Vec<Vec<char>>, count_vec: &Vec<u64>) -> Vec<Vec<Vec<u64>>> {
    // println!(
    //     "recursive call of get_count_groups on args {:?} and {:?}",
    //     char_subproblems, count_vec
    // );
    if char_subproblems.is_empty() {
        Vec::new()
    } else if char_subproblems.len() == 1 {
        vec![vec![count_vec.clone().to_vec()]]
    } else if count_vec.is_empty() {
        vec![vec![Vec::new(); char_subproblems.len()]]
    } else {
        // char_subproblems.len() >= 2
        // count_vec.len() >= 1
        let len_first_prob = char_subproblems[0].len();
        let len_last_prob = char_subproblems.last().unwrap().len();
        // to figure out what fits in the remainder can I be greedy?
        // let's ignore the lower bound of "the number so that the remainder of counts fits in
        // the remainder of the subproblems" for now
        let mut return_vec = Vec::new();
        for prefix in prefixes(count_vec)
            .iter()
            .take_while(|&v| {
                v.is_empty()
                    || (usize::try_from(v.iter().sum::<u64>()).unwrap() + v.len() - 1
                        <= len_first_prob)
            })
            .filter(|p| counts_cover_hashes(p, &char_subproblems[0]))
        {
            // println!("considering prefix {:?}", prefix);
            let recursive_call = get_count_groups(
                &char_subproblems[1..].to_vec(),
                &count_vec[prefix.len()..].to_vec(),
            );
            for mut count_allocation in recursive_call {
                // recursive_call has type Vec<Vec<Vec<u64>>>
                // so count_allocation has type Vec<Vec<u64>>
                // only append if stuff fits in the final bin
                // and if all the sub-call counts cover their hashes (because maybe they're
                // empty if stuff didn't work)
                // last_grop_fits will only be active on second-to-last iteration
                let last_group = count_allocation.last().unwrap(); // type Vec<u64>
                let last_group_fits = last_group.is_empty()
                    || (last_group.len()
                        + usize::try_from(last_group.iter().sum::<u64>()).unwrap()
                        - 1
                        <= len_last_prob);
                let rest_cover = count_allocation
                    .iter()
                    .enumerate()
                    .all(|(i, c)| counts_cover_hashes(c, &char_subproblems[i + 1]));
                if last_group_fits && rest_cover {
                    let mut my_vec = vec![prefix.clone()]; // has same type as count_vec, so a Vec<u64>
                    my_vec.append(&mut count_allocation);
                    return_vec.push(my_vec);
                }
            }
        }
        return_vec
    }
}

fn counts_cover_hashes(sub_count_vec: &[u64], subproblem: &Vec<char>) -> bool {
    // println!("calling counts_cover_hashes on {:?} and {:?}", sub_count_vec, subproblem);
    // ideally once we'd solved this we'd have no work left to do
    if let Some(first_hash_idx) = subproblem.iter().position(|c| c == &'#') {
        if let Some(len_first_hashes) = sub_count_vec.first() {
            // I'm not sure this matters but:
            // so the real start_rest is calculated like this
            // take a thing of length len_first_hashes
            // overlay it at first_hash_idx
            // then move it back until the thing after is a ?
            // if you move it back to the start, then counts don't cover hashes
            let start_rest = first_hash_idx + usize::try_from(*len_first_hashes).unwrap();
            if start_rest > subproblem.len() {
                true
            } else {
                let recurse_subproblem = subproblem[start_rest..].to_vec();
                let recurse_sub_count_vec = sub_count_vec[1..].to_vec();
                counts_cover_hashes(&recurse_sub_count_vec, &recurse_subproblem)
            }
        } else {
            // nothing in sum_count_vec
            // but subproblem contains hashes
            false
        }
    } else {
        // no hashes in subproblem
        true
    }
}

fn prefixes<T>(v: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut return_vec = vec![Vec::new()];
    for i in 1..(v.len() + 1) {
        return_vec.push(v[..i].to_vec());
    }
    return_vec
}

fn num_subproblem_sols(
    subproblem: &Vec<char>,
    sub_count_vec: &Vec<u64>,
    have_recursed: bool,
    num_sol_memo_map: &mut HashMap<(Vec<char>, Vec<u64>, bool), u64>,
) -> u64 {
    // would like to change this to "this many in this empty group, this at this hash" etc.
    // then analytically solve those sub-problems
    // first, start with zero
    // then get the number of viable places the first group could start
    // for each viable place, get the number of recursive sub-problem solutions
    // then add those up
    if let Some(num) =
        num_sol_memo_map.get(&(subproblem.clone(), sub_count_vec.clone(), have_recursed))
    {
        *num
    } else {
        let mut num_sols = 0;
        if sub_count_vec.is_empty() {
            if !subproblem.contains(&'#') {
                num_sols += 1;
            }
        } else if !subproblem.contains(&'#') {
            let free_gap = if have_recursed {
                subproblem.len()
                    - usize::try_from(sub_count_vec.iter().sum::<u64>()).unwrap()
                    - sub_count_vec.len()
            } else {
                subproblem.len() + 1
                    - usize::try_from(sub_count_vec.iter().sum::<u64>()).unwrap()
                    - sub_count_vec.len()
            };
            let k = sub_count_vec.len() - 1;
            num_sols = u64::try_from(binomial(free_gap + k + 1, k + 1)).unwrap();
        } else {
            // first group could start anywhere up to min(where there's a hash, where you can no
            // longer fit the rest in)
            // note: if we're in a recursive call, gap has to be bigger than 0
            // but if we're not, then it can
            let space_for_rest = usize::try_from(sub_count_vec.iter().sum::<u64>()).unwrap()
                + sub_count_vec.len()
                - 1;
            if subproblem.len() >= space_for_rest {
                // want to ensure that the rest can fit in the rest, but also that the rest covers the rest
                let fit_rest_in_index = subproblem.len() - space_for_rest;
                // println!("fit_rest_in_index {}", fit_rest_in_index);
                let first_hash = match subproblem.iter().position(|c| c == &'#') {
                    None => fit_rest_in_index + 1,
                    Some(n) => n,
                };
                // println!("first_hash {}", first_hash);
                let must_start_hashes = min(fit_rest_in_index, first_hash);
                // println!("must_start_hashes {}", must_start_hashes);
                let start_range = if have_recursed { 1 } else { 0 };
                for first_gap in start_range..(must_start_hashes + 1) {
                    // println!("first_gap {}", first_gap);
                    // pop off gap + count_vec[0] things from sub_problem
                    let new_subproblem = subproblem
                        .split_at(first_gap + usize::try_from(sub_count_vec[0]).unwrap())
                        .1
                        .to_vec();
                    let new_sub_counts = &sub_count_vec[1..];
                    // println!("new subproblem {:?}", new_subproblem);
                    // println!("new sub_counts {:?}", new_sub_counts);
                    // println!("counts cover hashes? {}", counts_cover_hashes(&new_sub_counts, &new_subproblem));
                    if counts_cover_hashes(new_sub_counts, &new_subproblem) {
                        num_sols += num_subproblem_sols(
                            &new_subproblem,
                            &new_sub_counts.to_vec(),
                            true,
                            num_sol_memo_map,
                        );
                    } // would like to break if else but that doesn't work.
                }
            }
        }
        num_sol_memo_map.insert(
            (subproblem.clone(), sub_count_vec.clone(), have_recursed),
            num_sols,
        );
        num_sols
    }
}

fn part_2_bad(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let spring_records: Vec<(Vec<char>, Vec<u64>)> = lines.map(parse_line).collect();
    let num_repeat = 5;
    let mut accum = 0;
    for mut record in spring_records {
        let num_chars = record.0.len();
        record.0.append(&mut vec!['?']);
        let new_char_vec: Vec<char> = record
            .0
            .into_iter()
            .cycle()
            .take((num_chars + 1) * num_repeat - 1)
            .collect();
        let num_count = record.1.len();
        let new_count_vec: Vec<u64> = record
            .1
            .into_iter()
            .cycle()
            .take(num_count * num_repeat)
            .collect();
        let (char_vec, count_vec) = simplify_record_2((new_char_vec, new_count_vec));
        // let verbose = char_vec == vec!['?', '#', '?', '?', '?', '#', '?', '#', '?', '?', '.', '#', '.', '#', '#', '#', '.', '?'];
        println!("char vec {:?}", char_vec);
        println!("count vec {:?}", count_vec);
        let verbose = false;
        let mut num_arrangements = 0;
        if count_vec.is_empty() {
            num_arrangements = 1;
        } else {
            // we've got work to do
            let num_non_final_gaps: usize = count_vec.len();
            let count_vec_sum: u64 = count_vec.iter().sum();
            if verbose {
                println!(
                    "char vec len {}, count_vec_sum {}",
                    u64::try_from(char_vec.len()).unwrap(),
                    count_vec_sum
                );
            }
            let num_undamaged: u64 = u64::try_from(char_vec.len()).unwrap() - count_vec_sum;
            for gap_vec in gap_iterator(num_non_final_gaps, num_undamaged) {
                if verbose {
                    println!("gap_vec {:?}", gap_vec);
                }
                // check if this gap vec + this count vec matches the char vec
                // if so add 1 to iterator
                assert!(
                    count_vec_sum + gap_vec.iter().sum::<u64>()
                        <= u64::try_from(char_vec.len()).unwrap()
                );
                assert_eq!(gap_vec.len(), count_vec.len());
                let mut reconstructed: Vec<char> = Vec::new();
                let mut amount_added: usize = 0;
                for (gap, num_damaged) in zip(gap_vec.iter(), count_vec.iter()) {
                    let gap_usize = usize::try_from(*gap).unwrap();
                    let nd_usize = usize::try_from(*num_damaged).unwrap();
                    reconstructed.append(&mut vec!['.'; gap_usize]);
                    reconstructed.append(&mut vec!['#'; nd_usize]);
                    amount_added += gap_usize;
                    amount_added += nd_usize;
                }
                // append one dot for everything we didn't add
                // which is char vec len - (gap vec sum + count vec sum)
                //
                reconstructed.append(&mut vec!['.'; char_vec.len() - amount_added]);
                if verbose {
                    println!("reconstructed {:?}", reconstructed);
                }
                let gaps_work =
                    zip(reconstructed.iter(), char_vec.iter()).all(|(r, o)| [*r, '?'].contains(o));
                // let mut gaps_work = true;
                // for (rec_char, orig_char) in zip(reconstructed.iter(), char_vec.iter()) {
                //     gaps_work = gaps_work && ([*rec_char, '?'].contains(orig_char));
                // }
                if gaps_work {
                    if verbose {
                        println!("gap vec works");
                    }
                    num_arrangements += 1;
                } else if verbose {
                    println!("gap vec doesn't work");
                }
            }
        }
        // println!("num arrangements {}", num_arrangements);
        // println!("dumb_num_arrangements {}", dumb_num_arrangements);
        // assert_eq!(num_arrangements, dumb_num_arrangements);
        accum += num_arrangements;
    }
    accum
}

fn part_1(file_path: &str) -> u64 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let spring_records: Vec<(Vec<char>, Vec<u64>)> = lines.map(parse_line).collect();
    let mut accum = 0;
    for record in spring_records {
        // want to at least trim things off the ends
        // so recursively:
        // strip off dots at ends
        // strip off hashes at ends but modify counts accordingly
        // (and strip off question marks to match)
        // let dumb_char_vec = record.0.clone();
        // let dumb_count_vec = record.1.clone();
        // let mut dumb_num_arrangements = 0;
        // let dumb_non_final_gaps = dumb_count_vec.len();
        // let dumb_count_vec_sum: u64 = dumb_count_vec.iter().sum();
        // let dumb_num_undamaged: u64 = u64::try_from(dumb_char_vec.len()).unwrap() - dumb_count_vec_sum;
        // for gap_vec in gap_iterator(dumb_non_final_gaps, dumb_num_undamaged, false) {
        //     let mut reconstructed: Vec<char> = Vec::new();
        //     let mut amount_added: usize = 0;
        //     for (gap, num_damaged) in zip(gap_vec.iter(), dumb_count_vec.iter()) {
        //         let gap_usize = usize::try_from(*gap).unwrap();
        //         let nd_usize = usize::try_from(*num_damaged).unwrap();
        //         reconstructed.append(&mut vec!['.'; gap_usize]);
        //         reconstructed.append(&mut vec!['#'; nd_usize]);
        //         amount_added += gap_usize;
        //         amount_added += nd_usize;
        //     }
        //     reconstructed.append(&mut vec!['.'; dumb_char_vec.len() - amount_added]);
        //     let mut gaps_work = true;
        //     for (rec_char, orig_char) in zip(reconstructed.iter(), dumb_char_vec.iter()) {
        //         gaps_work = gaps_work && ([*rec_char, '?'].contains(orig_char));
        //     }
        //     if gaps_work {
        //         dumb_num_arrangements += 1;
        //     }
        // }

        // println!("record {:?}", record);
        let (char_vec, count_vec) = simplify_record(record);
        // println!("char vec {:?}", char_vec);
        // println!("count_vec {:?}", count_vec);
        // let verbose = char_vec == vec!['?', '#', '?', '?', '?', '#', '?', '#', '?', '?', '.', '#', '.', '#', '#', '#', '.', '?'];
        let verbose = false;
        let mut num_arrangements = 0;
        if count_vec.is_empty() {
            num_arrangements = 1;
        } else {
            // we've got work to do
            let num_non_final_gaps: usize = count_vec.len();
            let count_vec_sum: u64 = count_vec.iter().sum();
            if verbose {
                println!(
                    "char vec len {}, count_vec_sum {}",
                    u64::try_from(char_vec.len()).unwrap(),
                    count_vec_sum
                );
            }
            let num_undamaged: u64 = u64::try_from(char_vec.len()).unwrap() - count_vec_sum;
            for gap_vec in gap_iterator(num_non_final_gaps, num_undamaged) {
                if verbose {
                    println!("gap_vec {:?}", gap_vec);
                }
                // check if this gap vec + this count vec matches the char vec
                // if so add 1 to iterator
                assert!(
                    count_vec_sum + gap_vec.iter().sum::<u64>()
                        <= u64::try_from(char_vec.len()).unwrap()
                );
                assert_eq!(gap_vec.len(), count_vec.len());
                let mut reconstructed: Vec<char> = Vec::new();
                let mut amount_added: usize = 0;
                for (gap, num_damaged) in zip(gap_vec.iter(), count_vec.iter()) {
                    let gap_usize = usize::try_from(*gap).unwrap();
                    let nd_usize = usize::try_from(*num_damaged).unwrap();
                    reconstructed.append(&mut vec!['.'; gap_usize]);
                    reconstructed.append(&mut vec!['#'; nd_usize]);
                    amount_added += gap_usize;
                    amount_added += nd_usize;
                }
                // append one dot for everything we didn't add
                // which is char vec len - (gap vec sum + count vec sum)
                //
                reconstructed.append(&mut vec!['.'; char_vec.len() - amount_added]);
                if verbose {
                    println!("reconstructed {:?}", reconstructed);
                }
                let mut gaps_work = true;
                for (rec_char, orig_char) in zip(reconstructed.iter(), char_vec.iter()) {
                    gaps_work = gaps_work && ([*rec_char, '?'].contains(orig_char));
                }
                if gaps_work {
                    if verbose {
                        println!("gap vec works");
                    }
                    num_arrangements += 1;
                } else if verbose {
                    println!("gap vec doesn't work");
                }
            }
        }
        // println!("num arrangements {}", num_arrangements);
        // println!("dumb_num_arrangements {}", dumb_num_arrangements);
        // assert_eq!(num_arrangements, dumb_num_arrangements);
        accum += num_arrangements;
    }
    accum
}

fn parse_line(line: &str) -> (Vec<char>, Vec<u64>) {
    let mut parts = line.split(' ');
    let first_part: Vec<char> = parts.next().unwrap().chars().collect();
    let next_part: Vec<u64> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    (first_part, next_part)
}

fn simplify_record_2(record: (Vec<char>, Vec<u64>)) -> (Vec<char>, Vec<u64>) {
    let mut new_char_vec = record.0;
    let mut new_count_vec = record.1;
    // oh what if we make new_char_vec empty
    // or new_count_vec empty
    // let's assume we always do that right
    // first, see if there are any repeated dots in the middle, and delete them
    new_char_vec.dedup_by(|&mut c1, &mut c2| c1 == c2 && c1 == '.');
    // need better continuation predicate
    let mut have_modified = true;
    while have_modified {
        let start_lens = (new_char_vec.len(), new_count_vec.len());
        simplify_start(&mut new_char_vec, &mut new_count_vec);
        new_char_vec.reverse();
        new_count_vec.reverse();
        simplify_start(&mut new_char_vec, &mut new_count_vec);
        new_char_vec.reverse();
        new_count_vec.reverse();
        let end_lens = (new_char_vec.len(), new_count_vec.len());
        have_modified = start_lens != end_lens;
    }
    (new_char_vec, new_count_vec)
}

fn simplify_start(char_vec: &mut Vec<char>, count_vec: &mut Vec<u64>) {
    // drop initial and final dots
    // println!("loop 1 {:?}", new_char_vec);
    *char_vec = char_vec
        .iter()
        .skip_while(|c| **c == '.')
        .copied()
        .collect();
    // drop initial hashes
    // println!("loop 3 {:?}", new_char_vec);
    if char_vec.first() == Some(&'#') {
        let first_count: usize = count_vec[0].try_into().unwrap();
        if first_count == char_vec.len() {
            *char_vec = Vec::new();
            *count_vec = Vec::new();
        } else {
            // split at one more than you'd think: take away one char per broken thing,
            // plus one char for the necessary gap
            *char_vec = char_vec.split_at(first_count + 1).1.to_vec();
            *count_vec = (count_vec[1..]).to_vec();
        }
    }
    // things that can let me drop entries:
    // first element of count_vec is n, one of the first (n + 1) elements is a hash
    // no: it's more like, if first element of count_vec is n, and nth element is a hash,
    // then you can treat the question marks as dots
    let first_hash = char_vec.iter().position(|&c| c == '#');
    let first_count = usize::try_from(count_vec[0]).unwrap();
    if first_hash == Some(first_count) {
        *char_vec = char_vec[(first_count * 2)..].to_vec();
        *count_vec = count_vec[1..].to_vec();
    }
    // first few counts exactly fit in before the first dot
    // gah but wait they don't have to aaaaaaaaaaaaaa
    // let first_dot = char_vec.iter().position(|&c| c == '.');
    // // get the first index in count_vec where sum counts + sum min gaps is greater than or
    // // equal to the thing
    // // no, that just lets us split
    // // need exact equality
    // let mut sum_counts: u64 = 0;
    // for (i, &count) in count_vec.iter().enumerate() {
    //     sum_counts += count;
    //     if
    // }
}

fn simplify_record(record: (Vec<char>, Vec<u64>)) -> (Vec<char>, Vec<u64>) {
    let mut new_char_vec = record.0;
    let mut new_count_vec = record.1;
    // oh what if we make new_char_vec empty
    // or new_count_vec empty
    // let's assume we always do that right
    while !new_char_vec.is_empty()
        && (new_char_vec[0] != '?' || *new_char_vec.last().unwrap() != '?')
    {
        // drop initial and final dots
        // println!("loop 1 {:?}", new_char_vec);
        new_char_vec = new_char_vec
            .iter()
            .skip_while(|c| **c == '.')
            .copied()
            .collect();
        // println!("loop 2 {:?}", new_char_vec);
        if new_char_vec.last() == Some(&'.') {
            new_char_vec = new_char_vec
                .iter()
                .rev()
                .skip_while(|c| **c == '.')
                .copied()
                .collect::<Vec<_>>();
            new_char_vec.reverse();
        }
        // drop initial hashes
        // println!("loop 3 {:?}", new_char_vec);
        if new_char_vec.first() == Some(&'#') {
            let first_count: usize = new_count_vec[0].try_into().unwrap();
            if first_count == new_char_vec.len() {
                new_char_vec = Vec::new();
                new_count_vec = Vec::new();
            } else {
                // split at one more than you'd think: take away one char per broken thing,
                // plus one char for the necessary gap
                new_char_vec = new_char_vec.split_at(first_count + 1).1.to_vec();
                new_count_vec = new_count_vec[1..].to_vec();
            }
        }
        // drop final hashes
        // println!("loop 4 {:?}", new_char_vec);
        if new_char_vec.last() == Some(&'#') {
            let last_count: usize = usize::try_from(*new_count_vec.last().unwrap()).unwrap();
            if last_count == new_char_vec.len() {
                new_char_vec = Vec::new();
                new_count_vec = Vec::new();
            } else {
                // split at one more than you'd think: take away one char per broken thing,
                // plus one char for the necessary gap
                // println!("new_char_vec len {}", new_char_vec.len());
                // println!("last count {}", last_count);
                new_char_vec = new_char_vec
                    .split_at(new_char_vec.len() - last_count - 1)
                    .0
                    .to_vec();
                new_count_vec.pop();
            }
        }
        // println!("loop 5 {:?}", new_char_vec);
    }
    (new_char_vec, new_count_vec)
}

fn gap_iterator(num_non_final_gaps: usize, num_undamaged: u64) -> Vec<Vec<u64>> {
    // inner vecs must be of len num_non_final_gaps
    if num_non_final_gaps == 1 {
        let mut return_vec = Vec::new();
        for i in 0..(num_undamaged + 1) {
            return_vec.push(vec![i]);
        }
        return_vec
    } else if num_undamaged == u64::try_from(num_non_final_gaps).unwrap() - 1 {
        let mut ones_vec = vec![1; num_non_final_gaps - 1];
        ones_vec.push(0);
        ones_vec.reverse();
        vec![ones_vec]
    } else {
        let mut return_vec = Vec::new();
        // iterate over number of things I could fill return_vec with
        // it's got to have at least num_non_final_gaps - 1 things in it
        // and it can have anything up to num_undamaged
        for gap_total in (num_non_final_gaps - 1)..(usize::try_from(num_undamaged).unwrap() + 1) {
            let max_in_first_slot: u64 = u64::try_from(gap_total + 1 - num_non_final_gaps).unwrap();
            // println!("max in first slot {}", max_in_first_slot);
            for n in 0..(max_in_first_slot + 1) {
                // println!("num in first slot {}", n);
                let mut sub_problem = slot_iterator_(
                    num_non_final_gaps - 1,
                    u64::try_from(gap_total).unwrap() - n,
                );
                for slot_alloc in sub_problem.iter_mut() {
                    slot_alloc.push(n);
                    slot_alloc.reverse();
                }
                return_vec.append(&mut sub_problem);
            }
        }
        return_vec
    }
}

fn slot_iterator_(num_slots: usize, num_items: u64) -> Vec<Vec<u64>> {
    // act under the constraint of how every slot has to have one item
    // returns everything reversed
    if num_items == num_slots.try_into().unwrap() {
        vec![vec![1; num_slots]]
    } else if num_slots == 1 {
        vec![vec![num_items]]
    } else {
        // num slots better be bigger than 1
        let mut return_vec = Vec::new();
        let max_in_first_lot: u64 = num_items - u64::try_from(num_slots).unwrap() + 1;
        for n in 1..(max_in_first_lot + 1) {
            let mut sub_problem = slot_iterator_(num_slots - 1, num_items - n);
            for slot_alloc in sub_problem.iter_mut() {
                slot_alloc.push(n);
            }
            return_vec.append(&mut sub_problem);
        }
        return_vec
    }
}
