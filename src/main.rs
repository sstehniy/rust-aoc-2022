#[path = "calorie_counting/solution.rs"]
mod calorie_counting;
#[path = "rock_paper_scissors/solution.rs"]
mod rock_paper_scissors;

fn main() {
    calorie_counting::solve();
    // rock_paper_scissors::solve();
    println!("Hello, world!");
}