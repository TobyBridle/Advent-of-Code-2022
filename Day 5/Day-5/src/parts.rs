use std::collections::VecDeque;

pub fn part_one(mut lines: Vec<&str>, crate_count: usize) -> String {
    let parse_line = |line: &str| {
        // Consume either 3 whitespace or a character
        let mut tokens: Vec<Option<char>> = Vec::new();
        let mut buffer = String::new();
        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            buffer.push(c);
            if buffer.len() == 3 {
                let token = match buffer.chars().nth(1) {
                    Some(' ') => None,
                    Some(c) => Some(c),
                    _ => panic!("Invalid token"),
                };
                tokens.push(token);
                buffer.clear();
                chars.next(); // Skip whitespace
            }
        }

        return tokens;
    };

    let crates = lines
        .drain(..crate_count)
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Option<char>>>>();
    // Get max length of crates
    let max = crates.iter().map(|c| c.len()).max().unwrap();
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(max);
    crates.iter().for_each(|horizontal| {
        horizontal.iter().enumerate().for_each(|(i, c)| {
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            if c.is_some() {
                stacks[i].push_back(c.unwrap());
            }
        })
    });

    let mut move_crate = |amount: usize, from: usize, to: usize| {
        let from = from - 1;
        let to = to - 1;
        for _ in 0..amount {
            let c = stacks[from].pop_front().unwrap();
            stacks[to].push_front(c);
        }
    };

    // The next two lines are irrelevant
    let lines = lines.drain(2..);
    for instruction in lines {
        let mut parts = instruction
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        let amount = parts.next().unwrap();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        move_crate(amount, from, to);
    }

    stacks.iter_mut().map(|s| s.pop_front().unwrap()).collect()
}

pub fn part_two(mut lines: Vec<&str>, crate_count: usize) -> String {
    let parse_line = |line: &str| {
        // Consume either 3 whitespace or a character
        let mut tokens: Vec<Option<char>> = Vec::new();
        let mut buffer = String::new();
        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            buffer.push(c);
            if buffer.len() == 3 {
                let token = match buffer.chars().nth(1) {
                    Some(' ') => None,
                    Some(c) => Some(c),
                    _ => panic!("Invalid token"),
                };
                tokens.push(token);
                buffer.clear();
                chars.next(); // Skip whitespace
            }
        }

        return tokens;
    };

    let crates = lines
        .drain(..crate_count)
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Option<char>>>>();
    // Get max length of crates
    let max = crates.iter().map(|c| c.len()).max().unwrap();
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(max);
    crates.iter().for_each(|horizontal| {
        horizontal.iter().enumerate().for_each(|(i, c)| {
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            if c.is_some() {
                stacks[i].push_back(c.unwrap());
            }
        })
    });

    let mut move_crate = |amount: usize, from: usize, to: usize| {
        let from = from - 1;
        let to = to - 1;
        let mut temp = VecDeque::new();
        for _ in 0..amount {
            let c = stacks[from].pop_front().unwrap();
            temp.push_back(c);
        }
        temp.append(&mut stacks[to]);
        stacks[to] = temp;
    };

    // The next two lines are irrelevant
    let lines = lines.drain(2..);
    for instruction in lines {
        let mut parts = instruction
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        let amount = parts.next().unwrap();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        move_crate(amount, from, to);
    }

    stacks.iter_mut().map(|s| s.pop_front().unwrap()).collect()
}
