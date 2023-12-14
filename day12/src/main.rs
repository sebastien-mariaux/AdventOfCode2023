mod part1;
mod part1_improved;
mod part2;
mod utils;

fn main() {
    println!("The result for part 1 is {}", part1_improved::solve_puzzle("input"));
    println!("The result for part 2 is {}", part2::solve_puzzle("input"));
}
