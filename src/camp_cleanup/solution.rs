const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In camp cleanup solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

fn solve_part_one(contents: &str) {
    let mut fully_overlapping_count: u32 = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(",").collect();
        let first = parts[0];
        let second = parts[1];
        let first_range: Vec<u32> = first
            .split("-")
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        let second_range: Vec<u32> = second
            .split("-")
            .map(|str| str.parse::<u32>().unwrap())
            .collect();

        if (first_range[0] <= second_range[0]
            && first_range[1] >= second_range[1])
            || (second_range[0] <= first_range[0]
                && second_range[1] >= first_range[1])
        {
            fully_overlapping_count += 1;
        }
    }

    println!("Solution part 1: {}", &fully_overlapping_count);
}

fn solve_part_two(contents: &str) {
    let mut overlapping_count = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(",").collect();
        let first = parts[0];
        let second = parts[1];
        let first_range: Vec<u32> = first
            .split("-")
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        let second_range: Vec<u32> = second
            .split("-")
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        if (first_range[0] <= second_range[0]
            && first_range[1] >= second_range[0])
            || (second_range[0] <= first_range[0]
                && second_range[1] >= first_range[0])
        {
            overlapping_count += 1;
        }
    }
    println!("Solution part 2: {}", &overlapping_count);
}
