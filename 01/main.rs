// 0.003s on A4-5000 (1.5GHz)
fn main() {
    let directions = include_str!("input.txt");
    let (destination, first_basement) = get_floor_information(directions);
    println!("Which Floor:    {}", destination);
    println!("First Basement: {}", first_basement);
}

/// Takes a string of directions as input and calculates both the destination floor and the number
/// of steps required to reach the first basement floor.
fn get_floor_information(input: &str) -> (isize, isize) {
    let mut floor = 0;
    let mut first_basement_not_found = true;
    let mut first_basement = 0;
    for character in input.chars() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if first_basement_not_found {
            first_basement = first_basement + 1;
            if floor == -1 { first_basement_not_found = false; }
        }
    }
    (floor, first_basement)
}
