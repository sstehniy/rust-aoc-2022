const INPUT: &str = include_str!("input.txt");

enum EvalResult {
    WIN,
    LOSE,
    DRAW,
}

pub fn solve() {
    println!("In calorie counting solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

fn solve_part_one(contents: &str) {
    let mut total_score = 0;
    for (idx, line) in contents.lines().enumerate() {
        let (opponent, mine) =
            split_line(line).expect(&format!("Wrong input on line {}", idx));
        let choice_score = get_choice_score_part_one(mine);
        let result = eval_round_part_one(opponent, mine);
        let total_round_score = choice_score.unwrap()
            + match result {
                EvalResult::WIN => 6,
                EvalResult::LOSE => 0,
                EvalResult::DRAW => 3,
            };
        total_score += total_round_score as u32;
    }

    println!("Total score Part 1: {}", total_score);
}

fn solve_part_two(contents: &str) {
    let mut total_score = 0;
    for (idx, line) in contents.lines().enumerate() {
        let (opponent, mine) =
            split_line(line).expect(&format!("Wrong input on line {}", idx));
        let choice_score = get_choice_score_part_two(opponent, mine);
        let result = eval_round_part_two(mine);
        let total_round_score = choice_score.unwrap()
            + match result {
                EvalResult::WIN => 6,
                EvalResult::LOSE => 0,
                EvalResult::DRAW => 3,
            };
        total_score += total_round_score as u32;
    }

    println!("Total score Part 2: {}", total_score);
}

fn eval_round_part_one(opponent: &str, mine: &str) -> EvalResult {
    match (opponent, mine) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => EvalResult::WIN,
        ("A", "Z") | ("B", "X") | ("C", "Y") => EvalResult::LOSE,
        ("A", "X") | ("B", "Y") | ("C", "Z") => EvalResult::DRAW,
        _ => EvalResult::DRAW,
    }
}

fn eval_round_part_two(mine: &str) -> EvalResult {
    match mine {
        "X" => EvalResult::LOSE,
        "Y" => EvalResult::DRAW,
        "Z" => EvalResult::WIN,
        _ => EvalResult::DRAW,
    }
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    let mut splitted = line.split_whitespace();

    let first_letter = match splitted.next()? {
        "A" => "A",
        "B" => "B",
        "C" => "C",
        _ => return None, // return None if the letter is not A, B, or C
    };
    let second_letter = match splitted.next()? {
        "X" => "X",
        "Y" => "Y",
        "Z" => "Z",
        _ => return None,
    };
    Some((first_letter, second_letter))
}

fn get_choice_score_part_one(choice: &str) -> Option<u8> {
    match choice {
        "X" => Some(1),
        "Y" => Some(2),
        "Z" => Some(3),
        _ => None,
    }
}

fn get_choice_score_part_two(opponent: &str, mine: &str) -> Option<u8> {
    match (opponent, mine) {
        // i get 1 for X Rock, 2 for Y Paper, 3 for Z Scissors
        // e.g when opponent selects A(Rock), to get Paper as output,
        // it means the round should end in my wind => ("A", "Z") tuple results in me getting 2 points
        // output should be rock
        ("A", "Y") | ("B", "X") | ("C", "Z") => Some(1),
        // output should be paper
        ("A", "Z") | ("B", "Y") | ("C", "X") => Some(2),
        // output should be scissors
        ("A", "X") | ("B", "Z") | ("C", "Y") => Some(3),
        _ => None,
    }
}
