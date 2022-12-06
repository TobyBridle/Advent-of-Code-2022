pub mod parts;

#[cfg(test)]
pub mod tests {
    pub use super::*;
    pub use crate::parts::*;

    #[test]
    fn test_part_one() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        assert_eq!(part_one(input, 3), String::from("CMZ"));
    }

    #[test]
    fn test_part_two() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        assert_eq!(part_two(input, 3), String::from("MCD"));
    }
}
