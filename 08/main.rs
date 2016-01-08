// 0.003s on A4-5000 (1.5GHz)
fn main() {
    let input = include_str!("input.txt");
    let (mut total_part_one, mut total_part_two) = (0 as usize, 0 as usize);
    for line in input.lines() {
        part_one(line, &mut total_part_one);
        part_two(line, &mut total_part_two);
    }
    println!("Part One: {}", total_part_one);
    println!("Part Two: {}", total_part_two);
}

fn part_one(line: &str, difference: &mut usize) {
    let mut trimmed = String::from(line.trim());
    *difference += trimmed.matches("\\\\").count();
    trimmed = trimmed.replace("\\\\", "$");
    *difference += 2 + trimmed.matches("\\\"").count() + trimmed.matches("\\x").count() * 3;
}

fn part_two(line: &str, difference: &mut usize) {
    let trimmed = line.trim();
    *difference += 2 + trimmed.matches("\"").count() + trimmed.matches("\\").count();
}
