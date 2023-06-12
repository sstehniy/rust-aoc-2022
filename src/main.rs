#[path = "calorie_counting/solution.rs"]
mod calorie_counting;
#[path = "camp_cleanup/solution.rs"]
mod camp_cleanup;
#[path = "rock_paper_scissors/solution.rs"]
mod rock_paper_scissors;
#[path = "rucksack_reorganization/solution.rs"]
mod rucksack_reorganization;
#[path = "supply_stacks/solution.rs"]
mod supply_stacks;
#[path = "tuning_trouble/solution.rs"]
mod tuning_trouble;

fn main() {
    calorie_counting::solve();
    println!("-------------------------------------------");
    rock_paper_scissors::solve();
    println!("-------------------------------------------");
    rucksack_reorganization::solve();
    println!("-------------------------------------------");
    camp_cleanup::solve();
    println!("-------------------------------------------");
    supply_stacks::solve();
    println!("-------------------------------------------");
    tuning_trouble::solve();
}
