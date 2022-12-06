fn part_one(lines: Vec<&str>) -> (i32, i32) {
    let mut highest_calorie = 0; // We will set this to the highest number of calories in a line
    let mut current_calories = 0;

    let mut line_tracker = 0;
    let mut highest_at_line = 0;
    lines.iter().for_each(|line| {
        if line.is_empty() {
            // We have reached the end of a calorie group
            if current_calories > highest_calorie {
                highest_calorie = current_calories;
                highest_at_line = line_tracker;
            }
            current_calories = 0;
        } else {
            // We are still in the same calorie group
            current_calories += line.parse::<i32>().unwrap();
        }
        line_tracker += 1;
    });
    return (highest_at_line, highest_calorie);
}

fn part_two(lines: Vec<&str>) -> i128 {
    // Call part one 3 times but remove the corresponding lines on each
    // call
    let mut lines_copy = lines.clone();

    // When we find the `elf` whitespace, we want to remove all of the lines until
    // the next whitespace
    let (mut line, mut calorie) = part_one(lines_copy.to_owned());
    let mut top_three = vec![calorie as i128];
    println!("Line: {}, Calorie: {}", line, calorie);
    for _ in 0..2 {
        println!("Calories: {:?}. Next best was found at {}", top_three, line);
        // Line is the line at the end of calorie group.
        // We must remove all lines from this until the previous whitespace
        for i in (0..line).rev() {
            if lines_copy[i as usize].is_empty() {
                // We have found the whitespace
                lines_copy.remove(i as usize);
                line = i;
                break;
            } else {
                lines_copy.remove(i as usize);
            }
        }
        (line, calorie) = part_one(lines_copy.to_owned());
        top_three.push(calorie as i128);
    }

    top_three.iter().sum()
}

fn main() {
    let mut input = Vec::new();
    // Grab the lines and place them into input
    // `input.txt` is the file containing the puzzle problem.
    let file = std::fs::read_to_string("input.txt").unwrap();
    for line in file.lines() {
        input.push(line);
    }

    let (line, highest_calorie) = part_one(input.clone());
    println!("Greatest Calories: {} at Line {}", highest_calorie, line);

    let top_three = part_two(input.clone());
    println!("Top three: {}", top_three);
}
