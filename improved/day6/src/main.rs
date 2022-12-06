use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day6/day6.txt").unwrap();

    // Puzzle 1
    let size = 4;
    for (idx, slice) in input.as_bytes().windows(size).enumerate() {
        let set: HashSet<_> = slice.iter().map(|&ch| ch as char).collect();
        
        if set.len() == size {
            println!("Puzzle 1: {}", idx + size);
            break;
        }
    }
    
    // Puzzle 2
    let size = 14;
    for (idx, slice) in input.as_bytes().windows(size).enumerate() {
        let set: HashSet<_> = slice.iter().map(|&ch| ch as char).collect();

        if set.len() == size {
            println!("Puzzle 2: {}", idx + size);
            break;
        }
    }
}