const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In tuning trouble solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

fn solve_part_one(contents: &str) {
    let chars: Vec<char> = contents.chars().collect();
    let mut start = 0;
    let mut end = 1;
    while end - start < 4 && end <= chars.len() {
        if !chars[start..end].contains(&chars[end]) {
            end += 1;
        } else {
            start = chars.len().min(
                start
                    + chars[start..end]
                        .iter()
                        .position(|ch| ch == &chars[end])
                        .unwrap()
                    + 1,
            );
            end = chars.len().min(start + 1);
        }
    }

    println!("Solution part 1: {}", start + 4);
}

fn solve_part_two(contents: &str) {
    let chars: Vec<char> = contents.chars().collect();
    let mut start = 0;
    let mut end = 1;
    while end - start < 14 && end <= chars.len() {
        if !chars[start..end].contains(&chars[end]) {
            end += 1;
        } else {
            start = chars.len().min(
                start
                    + chars[start..end]
                        .iter()
                        .position(|ch| ch == &chars[end])
                        .unwrap()
                    + 1,
            );
            end = chars.len().min(start + 1);
        }
    }

    println!("Solution part 2: {}", start + 14);
}
