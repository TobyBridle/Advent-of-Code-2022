pub mod parts;

#[cfg(test)]
pub mod tests {
    pub use super::*;
    pub use crate::parts::*;

    #[test]
    fn test_part_one() {
        let input = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"
        ];
        assert_eq!(part_two(input), 4);
    }
}
