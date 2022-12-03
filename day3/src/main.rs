use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day3/day3.txt").unwrap();

    // Puzzle 1
    let mut priority_sum = 0;
    for rucksack in input.trim().split('\n') {
        let compartment_len = rucksack.len() / 2;
        let mut rucksack_items = rucksack.chars();

        let compartment_1: String = rucksack_items.by_ref().take(compartment_len).collect();
        let compartment_2: String = rucksack_items.by_ref().take(compartment_len).collect();
        assert!(rucksack_items.next().is_none());

        let common_item = find_common_item(&[&compartment_1, &compartment_2]);

        priority_sum += calc_item_priority(common_item)
    }
    println!("Puzzle 1: {priority_sum}");

    // Puzzle 2
    let rucksacks: Vec<_> = input.trim().split('\n').collect();
    let priority_sum = rucksacks.chunks(3).fold(0, |acc, group| {
        let common_item = find_common_item(group);

        acc + calc_item_priority(common_item)
    });
    println!("Puzzle 2: {priority_sum}")
}

fn find_common_item(input: &[&str]) -> char {
    let mut set_list = input
        .iter()
        .map(|rucksack| rucksack.chars().collect::<HashSet<_>>());

    let initial_value = set_list.next().unwrap();
    let common_item = set_list.fold(initial_value, |acc, x| {
        acc.intersection(&x).copied().collect()
    });
    assert!(common_item.len() == 1);

    common_item.into_iter().next().unwrap()
}

fn calc_item_priority(item: char) -> i32 {
    const LOWER_CASE_CLAMP: i32 = 96;
    const UPPER_CASE_CLAMP: i32 = 64;
    const UPPER_CASE_PRIORITY_START: i32 = 26;

    assert!(item.is_ascii_alphabetic());
    if item.is_ascii_lowercase() {
        item as i32 - LOWER_CASE_CLAMP
    } else {
        item as i32 - UPPER_CASE_CLAMP + UPPER_CASE_PRIORITY_START
    }
}
