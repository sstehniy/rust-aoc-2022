const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In calorie counting solver");

    let max_carried = solve_part_one(&INPUT);

    println!("Max carried: {}", max_carried);

    let top_three_carried = solve_part_two(&INPUT);

    println!("Top three carried: {}", top_three_carried);
}

fn solve_part_one(contents: &str) -> u32 {
    let mut max = 0;
    let mut current_sum = 0;

    for line in contents.lines() {
        if let Ok(num) = line.trim().parse::<u32>() {
            current_sum += num;
        } else if current_sum > 0 {
            max = max.max(current_sum);
            current_sum = 0;
        }
    }

    max
}

fn solve_part_two(contents: &str) -> u32 {
    let mut all_carried: Vec<u32> = contents
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.parse::<u32>().ok())
                .fold(0, |acc, num| acc + num)
        })
        .collect();

    all_carried.sort_unstable();
    all_carried.reverse();
    all_carried.truncate(3);

    all_carried.iter().sum()
}
