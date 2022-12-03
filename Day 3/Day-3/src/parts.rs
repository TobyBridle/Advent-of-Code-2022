pub fn part_one(lines: Vec<&str>) -> u32 {
    let mut priority = 0;
    for line in lines {
        let len = line.len();
        let halves = vec![
            line.chars().take(len / 2).collect::<String>(),
            line.chars().skip(len / 2).take(len / 2).collect::<String>(),
        ];

        let get_priority = |c: char| {
            // Convert lowercase to between 1 and 26
            // Convert uppercase to between 27 and 52
            if c.is_lowercase() {
                c as u32 - 96
            } else {
                c as u32 - 64 + 26
            }
        };

        // Find the characters that both have in common.
        // Convert the strings to sets and intersect them.
        let common = halves[0]
            .chars()
            .collect::<std::collections::HashSet<char>>();
        common
            .intersection(
                &halves[1]
                    .chars()
                    .collect::<std::collections::HashSet<char>>(),
            )
            .for_each(|c| {
                priority += get_priority(*c);
            });
    }
    priority
}

pub fn part_two(lines: Vec<&str>) -> u32 {
    let mut priority = 0;
    // We want to take 3 lines at a time.
    let get_priority = |c: char| {
        // Convert lowercase to between 1 and 26
        // Convert uppercase to between 27 and 52
        if c.is_lowercase() {
            c as u32 - 96
        } else {
            c as u32 - 64 + 26
        }
    };
    for chunk in lines.chunks(3) {
        let mut lines = chunk.to_vec();
        // We want to make a HashSet of each line and then take the intersection.
        let mut sets = Vec::new();
        for line in lines {
            sets.push(line.chars().collect::<std::collections::HashSet<char>>());
        }
        // Work out the intersection of all three sets
        let mut intersection = sets[0].clone();
        intersection = intersection
            .intersection(&sets[1])
            .cloned()
            .collect::<std::collections::HashSet<char>>();
        intersection = intersection
            .intersection(&sets[2])
            .cloned()
            .collect::<std::collections::HashSet<char>>();
        intersection.iter().for_each(|c| {
            priority += get_priority(*c);
        });
    }
    priority
}
