use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("inputs/day6/day6.txt").unwrap();

    // Puzzle 1
    let mut recent = Recent::new(4);
    for (idx, char) in input.char_indices() {
        recent.insert(char);

        if recent.is_marker() {
            println!("Puzzle 1: {}", idx + 1);
            break;
        }
    }

    // Puzzle 2
    let mut recent = Recent::new(14);
    for (idx, char) in input.char_indices() {
        recent.insert(char);

        if recent.is_marker() {
            println!("Puzzle 2: {}", idx + 1);
            break;
        }
    }
}

#[derive(Default)]
pub struct Recent {
    data: VecDeque<char>,
    len: usize,
}

impl Recent {
    fn new(len: usize) -> Self {
        Self {
            len,
            ..Default::default()
        }
    }

    fn insert(&mut self, ch: char) {
        self.data.push_back(ch);

        if self.data.len() > self.len {
            self.data.pop_front();
        }
    }

    fn is_marker(&self) -> bool {
        self.data.iter().collect::<HashSet<_>>().len() == self.len // I'm lazy
    }
}
