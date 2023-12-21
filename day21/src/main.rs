mod part1;
// mod part2;
mod utils;

fn main() {
    println!(
        "The result for part 1 is {}",
        part1::solve_puzzle("input", 64)
    );
    // println!(
    //     "The result for part 2 is {}",
    //     part2::solve_puzzle("test_data", 100)
    // );
    // println!("The result for part 2 is {}", part2::solve_puzzle("input", 26501365));
}
