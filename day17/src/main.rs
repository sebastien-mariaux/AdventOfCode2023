mod part1;
mod part1_unoptimized;
mod part2;
mod utils;

fn main() {
    println!("The result for part 1 is {}", part1::solve_puzzle("input"));
    println!(
        "The result for part 1 (on test data - without optim) is {}",
        part1_unoptimized::solve_puzzle("test_data")
    );
    println!("The result for part 2 is {}", part2::solve_puzzle("input"));
}
