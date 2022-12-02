fn main() {
    let input = std::fs::read_to_string("inputs/day2/day2.txt").unwrap();

    // Puzzle 1
    let rounds_iter = input.trim().split('\n'); // Iterator over each round

    let score = rounds_iter.fold(0, |total_score, round| {
        let (elf_hand, my_hand) = split(round);
        let (elf_hand, my_hand) = (Hand::from(elf_hand), Hand::from(my_hand));

        total_score + calculate_round_score(elf_hand, my_hand)
    });
    println!("Puzzle 1: {score}");

    // Puzzle 2
    let rounds_iter = input.trim().split('\n'); // Iterator over each round

    let score = rounds_iter.fold(0, |total_score, round| {
        let (elf_hand, my_outcome) = split(round);
        let (elf_hand, my_outcome) = (Hand::from(elf_hand), Outcome::from(my_outcome));

        let my_hand = calculate_round_hand(elf_hand, my_outcome);
        total_score + calculate_round_score(elf_hand, my_hand)
    });
    println!("Puzzle 2: {score}")
}

/// Split each round into the two portions. For ex. "A Y" -> ('A', 'Y')
fn split(round: &str) -> (char, char) {
    let mut round = round.char_indices();
    let (_, elf_hand) = round.next().unwrap();
    round.next(); // skip space
    let (_, my_hand) = round.next().unwrap();

    (elf_hand, my_hand)
}

fn calculate_round_score(elf_hand: Hand, my_hand: Hand) -> i32 {
    use Hand::*;
    use Outcome::*;

    if elf_hand == my_hand {
        return Draw as i32 + my_hand as i32;
    }

    let outcome_score = match (elf_hand, my_hand) {
        (Rock, Scissors) => Defeat as i32,
        (Scissors, Paper) => Defeat as i32,
        (Paper, Rock) => Defeat as i32,
        (_, _) => Victory as i32,
    };

    outcome_score + my_hand as i32
}

fn calculate_round_hand(elf_hand: Hand, my_outcome: Outcome) -> Hand {
    use Hand::*;
    use Outcome::*;

    match (elf_hand, my_outcome) {
        (_, Draw) => elf_hand,
        (Scissors, Defeat) => Paper,
        (Paper, Defeat) => Rock,
        (Rock, Defeat) => Scissors,
        (Rock, Victory) => Paper,
        (Paper, Victory) => Scissors,
        (Scissors, Victory) => Rock,
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    pub fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("unexpected hand: {}", ch),
        }
    }
}

enum Outcome {
    Defeat = 0,
    Draw = 3,
    Victory = 6,
}

impl Outcome {
    pub fn from(ch: char) -> Self {
        match ch {
            'X' => Outcome::Defeat,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Victory,
            _ => panic!("unexpected outcome: {}", ch),
        }
    }
}
