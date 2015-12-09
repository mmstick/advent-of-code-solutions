// 0.006s on A4-5000 (1.5 GHz)

fn main() {
    let input = include_str!("input.txt");
    let nice_count = input.lines().filter(|string| is_nice(string)).count();
    println!("{}", nice_count);
}

/// Returns true if the string contains three vowels, double characters and is not a bad string.
fn is_nice(input: &str) -> bool {
    contains_three_vowels(input) && contains_double_character(input) && !is_bad_string(input)
}

/// Returns true if the string contains three vowels.
fn contains_three_vowels(x: &str) -> bool {
    x.chars().filter(|&x| x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u').count() >= 3
}

/// Returns true if the string contains double characters.
fn contains_double_character(x: &str) -> bool {
    let mut iterator = x.chars();
    let mut double_char = false;
    let mut previous_char: char;
    let mut current_char: char = ' ';
    loop {
        previous_char = current_char;
        current_char = match iterator.next() {
            Some(x) => x,
            None => break,
        };
        if previous_char == current_char { double_char = true; }
    }
    return double_char;
}

/// Returns true if the string contains bad substrings.
fn is_bad_string(x: &str) -> bool {
    x.contains("ab") || x.contains("cd") || x.contains("pq") || x.contains("xy")
}
