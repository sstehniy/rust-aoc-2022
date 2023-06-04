#[path = "calorie_counting/solution.rs"]
mod calorie_counting;
#[path = "rock_paper_scissors/solution.rs"]
mod rock_paper_scissors;
#[path = "rucksack_reorganization/solution.rs"]
mod rucksack_reorganization;

fn main() {
    calorie_counting::solve();
    println!("-------------------------------------------");
    rock_paper_scissors::solve();
    println!("-------------------------------------------");
    rucksack_reorganization::solve();
}
