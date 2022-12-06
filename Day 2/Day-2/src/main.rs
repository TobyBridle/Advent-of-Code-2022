enum MoveType {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    move_type: MoveType,
    value: u16,
}

impl Move {
    fn new(move_type: MoveType) -> Move {
        let value = match move_type {
            MoveType::Rock => 1,
            MoveType::Paper => 2,
            MoveType::Scissors => 3,
        };
        Move { move_type, value }
    }

    // Find the move we must use to force a certain condition
    fn to_force_condition(&self, condition: char) -> Move {
        // We will use this to provide a move for the player
        // to cause a certain condition when played against the enemy move.
        match condition {
            'Z' => {
                // What move we need to play to win.
                match self.move_type {
                    MoveType::Rock => Move::new(MoveType::Paper),
                    MoveType::Paper => Move::new(MoveType::Scissors),
                    MoveType::Scissors => Move::new(MoveType::Rock),
                }
            }
            'X' => {
                // What move we need to play to lose.
                match self.move_type {
                    MoveType::Rock => Move::new(MoveType::Scissors),
                    MoveType::Paper => Move::new(MoveType::Rock),
                    MoveType::Scissors => Move::new(MoveType::Paper),
                }
            }
            'Y' => {
                // We want to draw
                let move_type = match self.move_type {
                    MoveType::Rock => MoveType::Rock,
                    MoveType::Paper => MoveType::Paper,
                    MoveType::Scissors => MoveType::Scissors,
                };
                Move::new(move_type)
            }
            _ => panic!("Invalid condition"),
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Move {
        match c {
            'A' | 'X' => Move::new(MoveType::Rock),
            'B' | 'Y' => Move::new(MoveType::Paper),
            'C' | 'Z' => Move::new(MoveType::Scissors),
            _ => panic!("Invalid move {}", c as u8),
        }
    }
}

fn main() {
    let mut input = Vec::new();
    // Grab the lines and place them into input
    // `input.txt` is the file containing the puzzle problem.
    let file = std::fs::read_to_string("input.txt").unwrap();
    for line in file.lines() {
        input.push(line);
    }
    println!("Part 1: {}", part_one(input.to_owned()));
    println!("Part 2: {}", part_two(input.to_owned()));
}
fn part_one(lines: Vec<&str>) -> u16 {
    let mut score: u16 = 0;
    // Get the moves using the line halves
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let enemy_move = Move::from(parts[0].chars().nth(0).unwrap());
        let player_move = Move::from(parts[1].chars().nth(0).unwrap());
        // Compare the moves
        match (player_move.move_type, enemy_move.move_type) {
            // Win
            (MoveType::Rock, MoveType::Scissors)
            | (MoveType::Paper, MoveType::Rock)
            | (MoveType::Scissors, MoveType::Paper) => score += player_move.value + 6,
            // Draw
            (MoveType::Rock, MoveType::Rock)
            | (MoveType::Paper, MoveType::Paper)
            | (MoveType::Scissors, MoveType::Scissors) => score += player_move.value + 3,
            // Loss
            _ => score += player_move.value,
        }
    }
    score
}
fn part_two(lines: Vec<&str>) -> u16 {
    let mut score = 0;
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let enemy_move = Move::from(parts[0].chars().nth(0).unwrap());
        let condition = parts[1].chars().nth(0).unwrap();
        let player_move = enemy_move.to_force_condition(condition);

        let additional = match condition {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Invalid condition"),
        };
        score += player_move.value + additional;
    }
    score
}
