pub use Day_5::parts::*;
pub const AMOUNT: usize = 8;
fn main() {
    let mut input = Vec::new();
    // Grab the lines and place them into input
    //  is the file containing the puzzle problem.
    let file = std::fs::read_to_string("input.txt").unwrap();
    for line in file.lines() {
        input.push(line);
    }

    println!("Part 1: {}", part_one(input.to_owned(), AMOUNT));
    println!("Part 2: {}", part_two(input.to_owned(), AMOUNT));
}
