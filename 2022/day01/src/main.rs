fn main() {
    let input = include_str!("./input.txt");

    // we trim here so that we can split nicely without empty strs giving us edge cases to handle
    let elves_calories_list = input.trim().split("\n\n");

    let mut calories_count: Vec<_> = elves_calories_list.map(sum_elf_calories).collect();
    calories_count.sort_by(|a, b| b.cmp(a)); // Sort desc

    println!("Puzzle 1: {:?}", &calories_count[0]);
    println!("Puzzle 2: {:?}", calories_count.iter().take(3).sum::<i32>()) // Take top 3 and sum
}

/// Sum calorie list str of an elf. Using flatmap here may not be ideal because it removes errors:
/// This means we won't know if we missed a line that should've been part of the sum because of some bug
fn sum_elf_calories(elf_calories_str: &str) -> i32 {
    elf_calories_str
        .split('\n')
        .flat_map(str::parse::<i32>)
        .sum()
}
