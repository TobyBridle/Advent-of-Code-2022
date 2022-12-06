pub use Day_6::parts::*;
fn main() {
    let mut input: &str;
    // Grab the lines and place them into input
    //  is the file containing the puzzle problem.
    let file = std::fs::read_to_string("input.txt").unwrap();
    input = file.as_str();
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}
