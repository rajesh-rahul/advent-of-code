use std::collections::{VecDeque, BTreeMap};

fn main() {
    let input = std::fs::read_to_string("inputs/day5/day5.txt").unwrap();
    let (container_data, crane_data) = input.split_once("\n\n").unwrap();

    let mut container_data: Vec<_> = container_data.lines().collect();
    container_data.pop();

    // Puzzle 1
    let mut ship = ContainerShip::from(&container_data, CrateMover9000);
    for crane_action in crane_data.lines() {
        ship.perform_crane_action(crane_action);
    }
    println!("Puzzle 1: {}", ship.top());

    // Puzzle 2
    let mut ship = ContainerShip::from(&container_data, CrateMover9001);
    for crane_action in crane_data.lines() {
        ship.perform_crane_action(crane_action);
    }
    println!("Puzzle 2: {}", ship.top());
}

#[derive(Debug)]
struct ContainerShip<T>{
    containers: BTreeMap<usize, VecDeque<char>>,
    _crane: T
}
struct CrateMover9000;
struct CrateMover9001;

impl<T> ContainerShip<T> {
    pub fn from(container_data: &[&str], crane: T) -> Self {
        let mut ship = Self {
            _crane: crane,
            containers: Default::default()
        };

        for line in container_data {
            ship.load_line(line);
        }

        ship
    }
    pub fn top(&self) -> String {
        self.containers.values().fold(String::from(""), |mut acc, x| {
            acc.push(*x.back().unwrap());

            acc
        })
    }
    fn load_line(&mut self, line: &str) {
        for (idx, container) in line.as_bytes().chunks(4).enumerate() {
            if container[1] != b' ' {
                self.containers.entry(idx + 1).or_default().push_front(container[1] as char)
            }
        }
    }
}

trait CraneAction {
    fn perform_crane_action(&mut self, line: &str) {
        let values: Vec<_> = line.split(' ').collect();
        let parse_int = |val: &str| val.parse::<usize>().unwrap();
        let (count, from, to) =  (parse_int(values[1]), parse_int(values[3]),parse_int(values[5]));

        self.move_containers(count, from, to);
    }
    fn move_containers(&mut self, count: usize, from: usize, to: usize);
}

impl CraneAction for ContainerShip<CrateMover9000> {
    fn move_containers(&mut self, count:usize, from: usize, to: usize) {
        for _ in 0..count {
            let container = self.containers.get_mut(&from).unwrap().pop_back().unwrap();
            let to_stack = self.containers.get_mut(&to).unwrap();
    
            to_stack.push_back(container);
        };
    }
}

impl CraneAction for ContainerShip<CrateMover9001> {
    fn move_containers(&mut self, count:usize, from: usize, to: usize) {
        let from_stack = self.containers.get_mut(&from).unwrap();
        let range = (from_stack.len() - count)..from_stack.len();
        let mut containers: VecDeque<_> = from_stack.drain(range).collect();

        let to_stack = self.containers.get_mut(&to).unwrap();
        to_stack.append(&mut containers);
    }
}
