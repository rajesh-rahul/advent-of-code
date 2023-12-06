// Thank you chat gpt
fn quadratic_formula(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real roots
        None
    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        Some((root, root))
    } else {
        // Two real roots
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
        Some((root1, root2))
    }
}

fn total_options(time: f64, distance: f64) -> i64 {
    // boatSpeed * (allotedTime - boatSpeed) = distance
    // boatSpeed * allottedTime - boatSpeed^2 = distance
    // -x^2 + x*times[0] - distance[0] = 0 // example
    let curr_record_boat_speeds = quadratic_formula(-1.0, time, -distance).unwrap();
    let (low_speed, high_speed) = curr_record_boat_speeds;

    let low_speed_target: i64 = if low_speed == low_speed.ceil() {
        (low_speed + 1.0) as i64
    } else {
        low_speed.ceil() as i64
    };

    let high_speed_target: i64 = if high_speed == high_speed.floor() {
        (high_speed - 1.0) as i64
    } else {
        high_speed.floor() as i64
    };

    return high_speed_target - low_speed_target + 1;
}

fn part01(input: &str) {
    let (times, distances) = input.split_once("\n").unwrap();
    let times: Vec<_> = times
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<f64>)
        .collect();

    let distances: Vec<_> = distances
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<f64>)
        .collect();

    assert!(times.len() == distances.len());

    let multiplier = times
        .iter()
        .zip(distances)
        .fold(1, |acc, (&time, distance)| {
            acc * total_options(time, distance)
        });

    println!("Part01: {multiplier}");
}

fn part02(input: &str) {
    let (times, distances) = input.split_once("\n").unwrap();
    let time: f64 = times
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: f64 = distances
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let total_options = total_options(time, distance);
    println!("Part02: {total_options}");
}

fn main() {
    let input = include_str!("./input.txt");

    part01(input);
    part02(input);
}
