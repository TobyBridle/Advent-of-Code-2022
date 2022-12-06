pub mod parts;

#[cfg(test)]
pub mod tests {
    pub use super::*;
    pub use crate::parts::*;

    #[test]
    fn test_part_one() {
        let cases = vec![
            "mjqp",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        assert_eq!(part_one(cases[0]), 4);
        println!("Part 1 succeeded");
        assert_eq!(part_one(cases[1]), 5);
        assert_eq!(part_one(cases[2]), 6);
        assert_eq!(part_one(cases[3]), 10);
        assert_eq!(part_one(cases[4]), 11);
    }

    // #[test]
    // fn test_part_two() {
    //     let mut input = vec![];
    //     assert_eq!(part_two(input), 0);
    // }
}
