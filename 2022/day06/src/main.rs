mod v1;
mod v2;

fn main() {
    let input = std::fs::read_to_string("inputs/day6/day6.txt").unwrap();

    v1::part01(input.clone());
    v1::part02(input.clone());

    v2::part01(input.clone());
    v2::part02(input.clone());
}
