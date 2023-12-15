use std::cmp;
use std::fs::read_to_string;

fn main() {
    let result = part_2("./input.txt");
    println!("result {:?}", result);
}

// ok, actually have ranges of seed numbers
// can't afford to explicitly do everything in the range
// but I can split ranges up as necessary
// then will get the smallest thing in one of the ranges


fn part_2(file_path: &str) -> Option<u64> {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    // get seeds
    // break lines up into sections
    // for each seed for each section, do the obvious thing
    let seed_line = lines.next();
    let mut words = seed_line.unwrap().split(' ').peekable();
    words.next();
    let mut my_ranges: Vec<(u64, u64)> = Vec::new();
    while words.peek().is_some() {
        let start: u64 = words.next().unwrap().parse().unwrap();
        let range: u64 = words.next().unwrap().parse().unwrap();
        my_ranges.push((start, range));
    }
    // println!("{:?}", my_ranges);
    lines.next();
    let mut peekable_lines = lines.peekable();
    while peekable_lines.peek().is_some() {
        // get map
        // get thru map name
        peekable_lines.next();
        let mut map: Vec<(u64, u64, u64)> = Vec::new();
        while ![Some(&""), None].contains(&peekable_lines.peek()) {
            let mut nums = peekable_lines
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap());
            let dest_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let range_len = nums.next().unwrap();
            map.push((dest_start, src_start, range_len))
        }
        // println!("map {:?}", map);
        // sort the map
        // add ranges as needed
        map.sort_by_key(|&(_, s, _)| s);
        let mut next_ranges: Vec<(u64, u64)> = Vec::new();
        for &(my_start, my_len) in &my_ranges {
            // println!(
            //     "dealing with range starting {} with length {}",
            //     my_start, my_len
            // );
            // filter for the bits of the map where there's overlap
            let overlapping_map = map.iter().filter(|&(_, map_start, map_len)| {
                ranges_overlap(my_start, my_len, *map_start, *map_len)
            });
            // don't forget: some portion of the range may be unmapped
            let mut covered_to = my_start;
            for &(dest, map_start, map_len) in overlapping_map {
                if covered_to < map_start {
                    next_ranges.push((covered_to, map_start - covered_to));
                }
                let mapped_range_start = cmp::max(map_start, my_start);
                let mapped_range_end = cmp::min(map_start + map_len, my_start + my_len);
                let mapped_range_len = mapped_range_end - mapped_range_start;
                let next_range_start = dest + mapped_range_start - map_start;
                next_ranges.push((next_range_start, mapped_range_len));
                // println!(
                //     "portion from {} to {} gets pushed to {}, {}",
                //     mapped_range_start, mapped_range_len, next_range_start, mapped_range_len);
                covered_to = mapped_range_end;
            }
            // check if there wasn't anything overlapping
            if covered_to < my_start + my_len {
                next_ranges.push((covered_to, my_start + my_len - covered_to - 1));
            }
        }
        my_ranges = next_ranges;
        // println!("{:?}", my_ranges);
        // no empty line at the end of the file
        if peekable_lines.peek().is_some() {
            peekable_lines.next();
        }
    }
    my_ranges.into_iter().map(|(s, _)| s).min()
}

fn ranges_overlap(my_start: u64, my_len: u64, map_start: u64, map_len: u64) -> bool {
    let mine_bigger = my_start <= map_start && my_start + my_len >= map_start + map_len;
    let map_bigger = map_start <= my_start && map_start + map_len >= my_start + my_len;
    let map_overlaps_my_start = map_start <= my_start && map_start + map_len > my_start;
    let map_overlaps_my_end =
        map_start < my_start + my_len && map_start + map_len >= my_start + my_len;
    mine_bigger || map_bigger || map_overlaps_my_start || map_overlaps_my_end
}

// map format
// (destination start) (source start) (range length)
// unmapped numbers go to the same destination number
// find lowest location number that corresponds to an initial seed

// obvious plan: for every initial seed, get the location number via a series of maps
// next-obvious plan: figure out a way to compose the maps

// note that for the real puzzle the numbers are ginormous so you're going to want to do this implicitly

// so: for each seed, find out which range it's in, then use the obvious mapping.

fn part_1(file_path: &str) -> Option<u64> {
    let file_string = read_to_string(file_path).unwrap();
    let mut lines = file_string.lines();
    // get seeds
    // break lines up into sections
    // for each seed for each section, do the obvious thing
    let seed_line = lines.next();
    let mut words = seed_line.unwrap().split(' ');
    words.next();
    let mut my_numbers: Vec<u64> = Vec::new();
    for word in words {
        my_numbers.push(word.parse().unwrap());
    }
    lines.next();
    let mut peekable_lines = lines.peekable();
    while peekable_lines.peek().is_some() {
        // get map
        // get thru map name
        peekable_lines.next();
        let mut map: Vec<(u64, u64, u64)> = Vec::new();
        while ![Some(&""), None].contains(&peekable_lines.peek()) {
            let mut nums = peekable_lines
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap());
            let dest_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let range_len = nums.next().unwrap();
            map.push((dest_start, src_start, range_len))
        }
        // println!("map {:?}", map);
        let mut next_numbers: Vec<u64> = Vec::new();
        for number in my_numbers {
            let mut next_number = number;
            for (dest, src, range) in &map {
                if number >= *src && number < *src + *range {
                    next_number = *dest + number - *src;
                }
            }
            next_numbers.push(next_number);
        }
        my_numbers = next_numbers;
        // no empty line at the end of the file
        if peekable_lines.peek().is_some() {
            peekable_lines.next();
        }
    }
    my_numbers.into_iter().min()
}
