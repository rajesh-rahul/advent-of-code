fn main() {
    let result = std::fs::read_to_string("inputs/day1/day1.txt").unwrap();

    let split = result.split("\n\n").map(|each_elf_food| {
        if each_elf_food.contains('\n') {
            each_elf_food
                .trim()
                .split('\n')
                .map(|calorie_str| calorie_str.parse::<i32>().unwrap())
                .sum()
        } else {
            each_elf_food.parse().unwrap()
        }
    });

    let highest_calorie_count: i32 = split.max().unwrap();

    println!("{:?}", highest_calorie_count)
}
