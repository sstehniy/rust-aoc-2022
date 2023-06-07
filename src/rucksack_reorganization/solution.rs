const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In rucksack reorganization solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

struct LineDupe(usize, char);

impl Default for LineDupe {
    fn default() -> Self {
        LineDupe(0, ' ')
    }
}

struct GroupDupe(usize, usize, char);

impl Default for GroupDupe {
    fn default() -> Self {
        GroupDupe(0, 0, ' ')
    }
}

fn solve_part_one(contents: &str) {
    let mut all_dupes_by_line: Vec<LineDupe> = vec![];
    for (idx, line) in contents.lines().enumerate() {
        for i in 0..line.len() / 2 {
            let char = line.chars().nth(i).unwrap();
            if line[line.len() / 2..].contains(char)
                && all_dupes_by_line.get(idx).unwrap_or(&LineDupe::default()).1
                    != char
            {
                all_dupes_by_line.push(LineDupe(idx, char));
                continue;
            }
        }
    }
    let prio_sum: u32 = all_dupes_by_line.iter().map(|x| get_prio(x.1)).sum();
    println!("Solution Part 1: {}", prio_sum);
}
fn solve_part_two(contents: &str) {
    let chunks = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<&str>>>();
    let mut all_dupes: Vec<char> = vec![];
    for chunk in chunks {
        'inner: for ch in chunk[0].chars() {
            if chunk[1].contains(ch) && chunk[2].contains(ch) {
                all_dupes.push(ch);
                break 'inner;
            }
        }
    }
    let prio_sum: u32 = all_dupes.iter().map(|ch| get_prio(*ch)).sum();
    println!("Solution Part 1: {}", prio_sum);
}

fn get_prio(ch: char) -> u32 {
    match ch.is_ascii_lowercase() {
        true => ch as u32 - b'a' as u32 + 1,
        false => ch as u32 - b'A' as u32 + 27,
    }
}
