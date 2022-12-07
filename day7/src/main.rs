use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn main() {
    let input = std::fs::read_to_string("inputs/day7/day7.txt").unwrap();
    let input = input.strip_prefix("$ ").unwrap();

    let mut shell = Shell::default();

    for command in input.split("\n$ ") {
        match &command.trim()[..2] {
            "ls" => shell.parse_list_dir(command),
            "cd" => shell.parse_change_dir(command),
            _ => unreachable!(),
        };
    }

    // Puzzle 1
    println!("Puzzle 1: {}", shell.sum_dirs(100000));

    // Puzzle 2
    let space_left = 70000000 - *shell.system.get(Path::new("/")).unwrap();
    let space_to_clear = 30000000 - space_left;

    let mut sizes = shell.system.iter().collect::<Vec<_>>();
    sizes.sort_by_key(|(_, size)| **size);
    let (dir, space) = sizes
        .iter()
        .find(|(_, size)| **size >= space_to_clear)
        .unwrap();

    println!("Puzzle 2: Clear {} to gain {space}", dir.display());
}

#[derive(Default, Debug)]
struct Shell {
    current_path: PathBuf,
    pub system: HashMap<PathBuf, usize>,
}

impl Shell {
    pub fn parse_change_dir(&mut self, command: &str) {
        let (_, arg) = command.split_once(' ').unwrap();
        match arg {
            ".." => {
                self.current_path.pop();
            }
            other => self.current_path.push(other),
        };
    }

    pub fn sum_dirs(&self, max: usize) -> usize {
        self.system.values().filter(|&&sz| sz <= max).sum()
    }

    pub fn parse_list_dir(&mut self, command: &str) {
        let (_, args) = command.split_once('\n').unwrap();
        let sizes = args
            .lines()
            .map(|arg| arg.split_once(' ').unwrap().0)
            .filter(|arg| arg != &"dir")
            .map(|arg| arg.parse::<usize>().unwrap());

        for size in sizes {
            for path in self.current_path.as_path().ancestors() {
                *self.system.entry(path.into()).or_default() += size;
            }
        }
    }
}
