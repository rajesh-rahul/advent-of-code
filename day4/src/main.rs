use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("inputs/day4/day4.txt").unwrap();

    // Puzzle 1
    let mut duplicates = 0;
    for assignment_pair in input.lines() {
        let (elf_one, elf_two) = split_into_range(assignment_pair);

        if contains_range_fully(elf_one, elf_two) {
            duplicates += 1
        }
    }
    println!("Puzzle 1: {duplicates}");

    // Puzzle 2
    let mut duplicates = 0;
    for assignment_pair in input.lines() {
        let (elf_one, elf_two) = split_into_range(assignment_pair);

        if contains_range(elf_one, elf_two) {
            duplicates += 1
        }
    }
    println!("Puzzle 2: {duplicates}");
}

fn split_into_range(elf_pair: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (elf_one, elf_two) = elf_pair.split_once(',').unwrap();

    (convert_to_range_type(elf_one), convert_to_range_type(elf_two))
}

fn convert_to_range_type(range: &str) -> RangeInclusive<i32> {
    let (start, end) = range.split_once('-').unwrap();

    start.parse().unwrap()..=end.parse().unwrap()
}

fn contains_range_fully(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

fn contains_range(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}
