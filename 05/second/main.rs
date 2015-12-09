// 0.018s on A4-5000 (1.5 GHz)
fn main() {
    let input = include_str!("input.txt");
    let nice_count = input.lines().filter(|string| is_nice(string)).count();
    println!("{}", nice_count);
}

/// Returns true if the string is nice
fn is_nice(x: &str) -> bool {
    let mut double_char = false;
    let mut double_pair = false;
    let mut previous_char = ' ';
    let mut middle_char = ' ';
    let mut current_char = ' ';
    for character in x.chars() {
        previous_char = middle_char;
        middle_char = current_char;
        current_char = character;
        if previous_char == current_char { double_char = true; }
        if x.matches(&vec![middle_char,current_char].into_iter().collect::<String>())
            .count() > 1 { double_pair = true; }
    }
    return double_char && double_pair;
}
