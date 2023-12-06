use std::collections::HashSet;

pub fn part01(input: String) {
    let size = 4;
    for (idx, slice) in input.as_bytes().windows(size).enumerate() {
        let set: HashSet<_> = slice.iter().map(|&ch| ch as char).collect();

        if set.len() == size {
            println!("Puzzle 1: {}", idx + size);
            break;
        }
    }
}

pub fn part02(input: String) {
    let size = 14;
    for (idx, slice) in input.as_bytes().windows(size).enumerate() {
        let set: HashSet<_> = slice.iter().map(|&ch| ch as char).collect();

        if set.len() == size {
            println!("Puzzle 2: {}", idx + size);
            break;
        }
    }
}
