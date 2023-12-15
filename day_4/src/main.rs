use std::fs::read_to_string;

fn main() {
    let result = part_2("./input.txt");
    println!("{}", result);
}

// get list of things before the bar
// get list of things after the bar
// iterate thru things after, check if they're in the thing before.
// that's quadratic, can i think of a faster way?
// not quickly

// ok split is tricky because there's multiple spaces
// gotta filter

fn part_1(file_path: &str) -> i32 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut sum = 0;
    for line in lines {
        let mut sections = line.split(&[':', '|']);
        sections.next();
        let winners_str = sections.next().unwrap();
        let winners_iter = winners_str.split(' ').filter(|s| !s.is_empty());
        let winners_vec: Vec<&str> = winners_iter.collect();
        let ticket_iter = sections.next().unwrap().split(' ');
        let mut num_matches = 0;
        for ticket in ticket_iter {
            if winners_vec.contains(&ticket) { num_matches += 1; }
        }
        if num_matches > 0 {
            let two: i32 = 2;
            sum += two.pow(num_matches - 1);
        }
    }
    sum
}


fn part_2(file_path: &str) -> i32 {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let num_lines = lines.clone().collect::<Vec<_>>().len();
    let mut num_cards_vec: Vec<i32> = vec![1; num_lines];
    for (line_num, line) in lines.enumerate() {
        let mut sections = line.split(&[':', '|']);
        sections.next();
        let winners_str = sections.next().unwrap();
        let winners_iter = winners_str.split(' ').filter(|s| !s.is_empty());
        let winners_vec: Vec<&str> = winners_iter.collect();
        let ticket_iter = sections.next().unwrap().split(' ');
        let mut num_matches = 0;
        for ticket in ticket_iter {
            if winners_vec.contains(&ticket) {
                num_matches += 1;
            }
        }
        let current_multiple = num_cards_vec[line_num];
        // println!("line_num {}", line_num);
        // println!("current_multiple {}", current_multiple);
        // println!("num_matches {}", num_matches);
        if num_matches > 0 {
            for j in (line_num + 1)..(line_num + num_matches + 1) {
                num_cards_vec[j] += current_multiple;
            }
            // println!("Num cards in line 3 {}", num_cards_vec[3]);
        }
    }
    num_cards_vec.iter().sum()
}
