use std::collections::{HashSet, VecDeque};

pub fn first_unique_window(line: &str, n: usize) -> u32 {
    let mut chars = line.chars();
    let mut chars = chars.by_ref().enumerate();
    // Fixed length of 4 characters. Pushing one pops the other end
    let mut window: VecDeque<char> = VecDeque::with_capacity(n);
    while let Some((i, c)) = chars.next() {
        if i < n {
            window.push_back(c);
        } else {
            window.pop_front();
            window.push_back(c);
        }
        if window
            .to_owned()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            == n
        {
            return i as u32 + 1;
        }
    }
    0
}

pub fn part_one(line: &str) -> u32 {
    return first_unique_window(line, 4);
}

pub fn part_two(line: &str) -> u32 {
    return first_unique_window(line, 14);
}
