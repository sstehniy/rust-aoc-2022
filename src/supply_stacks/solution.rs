use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In supply stack solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

fn solve_part_one(contents: &str) {
    let splitted: Vec<&str> = contents.lines().map(|x| x).collect();

    let splitline_idx = splitted
        .iter()
        .position(|&s| s.is_empty())
        .unwrap_or_else(|| {
            println!("Wrong input file format");
            std::process::exit(1);
        });
    let count_line = &splitted[splitline_idx - 1];
    let total_stacks_count = count_line
        .trim()
        .as_bytes()
        .last()
        .map(|&b| (b - b'0') as usize)
        .expect("Something bad happened");
    let mut arr: Vec<Vec<char>> = vec![Vec::new(); total_stacks_count];

    let mut char_idxs: Vec<usize> = vec![];
    for (idx, ch) in count_line.chars().enumerate() {
        match ch.to_digit(10) {
            Some(_) => char_idxs.push(idx),
            None => {
                continue;
            }
        }
    }

    for line in &splitted[..(splitline_idx - 1)] {
        for (idx, val) in (&char_idxs).iter().enumerate() {
            if line.chars().nth(*val).unwrap().is_ascii_alphabetic() {
                arr[idx].push(line.chars().nth(*val).unwrap())
            }
        }
    }

    for task in &splitted[(splitline_idx + 1)..] {
        let nums = extract_numbers(*task);
        let count = nums[0] as usize;
        let from = nums[1] - 1;
        let to = nums[2] - 1;
        let mut to_move: Vec<char> = vec![];
        for el in &arr[from as usize][0..(count)] {
            to_move.push(*el);
        }
        arr[from as usize] = arr[from as usize][count..].to_vec();
        to_move.reverse();
        to_move.extend(arr[to as usize].to_vec());
        arr[to as usize] = to_move;
    }

    println!(
        "Solution part one: {:?}",
        arr.iter().map(|stack| stack[0]).collect::<String>()
    );
}

fn extract_numbers(line: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    return re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
}

fn solve_part_two(contents: &str) {
    let splitted: Vec<&str> = contents.lines().map(|x| x).collect();

    let splitline_idx = splitted
        .iter()
        .position(|&s| s.is_empty())
        .unwrap_or_else(|| {
            println!("Wrong input file format");
            std::process::exit(1);
        });
    let count_line = &splitted[splitline_idx - 1];
    let total_stacks_count = count_line
        .trim()
        .as_bytes()
        .last()
        .map(|&b| (b - b'0') as usize)
        .expect("Something bad happened");
    let mut arr: Vec<Vec<char>> = vec![Vec::new(); total_stacks_count];

    let mut char_idxs: Vec<usize> = vec![];
    for (idx, ch) in count_line.chars().enumerate() {
        match ch.to_digit(10) {
            Some(_) => char_idxs.push(idx),
            None => {
                continue;
            }
        }
    }

    for line in &splitted[..(splitline_idx - 1)] {
        for (idx, val) in (&char_idxs).iter().enumerate() {
            if line.chars().nth(*val).unwrap().is_ascii_alphabetic() {
                arr[idx].push(line.chars().nth(*val).unwrap())
            }
        }
    }

    for task in &splitted[(splitline_idx + 1)..] {
        let nums = extract_numbers(*task);
        let count = nums[0] as usize;
        let from = nums[1] - 1;
        let to = nums[2] - 1;
        let mut to_move: Vec<char> = vec![];
        for el in &arr[from as usize][0..(count)] {
            to_move.push(*el);
        }
        arr[from as usize] = arr[from as usize][count..].to_vec();
        to_move.extend(arr[to as usize].to_vec());
        arr[to as usize] = to_move;
    }

    println!(
        "Solution part two: {:?}",
        arr.iter().map(|stack| stack[0]).collect::<String>()
    );
}
