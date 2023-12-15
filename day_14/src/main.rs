use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = part_2("./input.txt");
    println!("{:?}", result);
    let elapsed = start.elapsed();
    println!("{:?} elapsed", elapsed);
}


// ok can't brute force this shit

fn part_2(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    let num_cycles = 1000000000;
    // let num_cycles = 100;
    // println!("original grid: {:?}", grid);
    let mut all_grids: Vec<Vec<Vec<char>>> = Vec::new();
    for k in 0..num_cycles {
        // check if rolling grid doesn't change anything
        let new_grid = spin_grid(grid);
        if let Some(i) = all_grids.iter().position(|g| *g == new_grid) {
            println!("repeat found at index {}", k);
            println!("repeat of index {}", i);
            let cycle_length = k - i;
            let spins_left = num_cycles - k - 1;
            let reduced_spins_left = spins_left % cycle_length;
            grid = all_grids[i + reduced_spins_left].clone();
            break;
        } else {
            all_grids.push(new_grid.clone());
            grid = new_grid;
        }
    }
    get_grid_weight(&grid)
}

fn spin_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid_n = tilt_grid(grid, true, true);
    let grid_w = tilt_grid(grid_n, true, false);
    let grid_s = tilt_grid(grid_w, false, true);
    tilt_grid(grid_s, false, false)
}

fn get_grid_weight(grid: &Vec<Vec<char>>) -> usize {
    let num_rows = grid.len();
    let mut accum = 0;
    for i in 0..num_rows {
        let num_os = grid[i]
            .iter()
            .filter(|&&c| c == 'O')
            .collect::<Vec<_>>()
            .len();
        accum += num_os * (num_rows - i);
    }
    accum
}

fn tilt_grid(grid: Vec<Vec<char>>, to_start: bool, ns: bool) -> Vec<Vec<char>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    let max_thing = if ns { num_cols } else { num_rows };
    for j in 0..max_thing {
        let line: Vec<char> = if ns {
            (0..num_rows).map(|i| grid[i][j]).collect()
        } else {
            grid[j].clone()
        };
        // println!("line {:?}", line);
        let new_line_segments: Vec<Vec<char>> = line
            .split(|&c| c == '#')
            .map(|s| my_move(s.to_vec(), to_start))
            .collect();
        let new_line = glue_segments(new_line_segments);
        // println!("new line {:?}", new_line);
        new_grid.push(new_line);
    }
    if ns {
        // gotta transpose
        // println!("pre-transposed grid {:?}", new_grid);
        let mut new_grid_transpose = Vec::new();
        // each thing in the new grid is a col
        // for each row, get that index from each column
        for i in 0..num_rows {
            new_grid_transpose.push((0..num_cols).map(|j| new_grid[j][i]).collect());
        }
        // println!("transposed grid {:?}", new_grid_transpose);
        new_grid_transpose
    } else {
        new_grid
    }
    
}

fn my_move(segment: Vec<char>, to_start: bool) -> Vec<char> {
    let length = segment.len();
    let num_os = segment.iter().filter(|&&c| c == 'O').collect::<Vec<_>>().len();
    let mut os_segment = vec!['O'; num_os];
    let mut dots_segment = vec!['.'; length - num_os];
    if to_start {
        os_segment.append(&mut dots_segment);
        os_segment
    } else {
        dots_segment.append(&mut os_segment);
        dots_segment
    }
}

fn glue_segments(mut segments: Vec<Vec<char>>) -> Vec<char> {
    let mut new_line = Vec::new();
    let num_segments = segments.len();
    for (i, s) in segments.iter_mut().enumerate() {
        new_line.append(s);
        if i != num_segments - 1 {
            new_line.push('#');
        }
    }
    new_line
}

// move rounded rocks to north
// calculate total load, which is how high the rock is if the rock is a O

// ok so for each column you store how many rocks it has / where the barriers are
// yeah divide it into groups based on the hashes, and how many Os per group
// then you can do it easily

fn part_1(file_path: &str) -> usize {
    let file_string = read_to_string(file_path).unwrap();
    let lines = file_string.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut accum = 0;
    for j in 0..num_cols {
        let col: Vec<char> = (0..num_rows).map(|i| grid[i][j]).collect();
        let (_, col_weight) = col
            .split(|&c| c == '#')
            .map(|s| (s.iter().filter(|&&c| c == 'O').collect::<Vec<_>>().len(), s.len()))
            .fold((0, 0), |(index, accum), (num_os, length)| {
                column_accumulate(num_rows, index, accum, num_os, length)
            });
        // println!("weight of column {} is {}", j, col_weight);
        accum += col_weight;
    }
    accum
}

fn column_accumulate(
    num_rows: usize,
    index: usize,
    accum: usize,
    num_os: usize,
    length: usize,
) -> (usize, usize) {
    let next_index = index + length + 1;
    let start_count = num_rows - index;
    let group_weight = if num_os == 0 {
        0
    } else {
        start_count * num_os - num_os * (num_os - 1) / 2
    };
    // println!("next_index {}", next_index);
    // println!("start_count {}", start_count);
    // println!("group_weight {}", group_weight);
    // println!("num_os {}", num_os);
    (next_index, accum + group_weight)
}
