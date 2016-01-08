// 0.203s on A4-5000 (1.5 GHz) For Part One Input
// 1.845s on A4-5000 (1.5 GHz) For Part Two Input
extern crate permutohedron;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input-two.txt");

    // Get a map of all possible relationship effects.
    let mut relationships: HashMap<[&str;2], isize> = HashMap::new();
    let mut people: Vec<&str> = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let key = [words[0], words[10].split('.').next().unwrap()];
        let value = get_happiness(words.clone());
        relationships.insert(key, value);
        if !people.contains(&words[0]) { people.push(words[0]); }
    }
    let seats = people.len();

    // Find the best seating arrangement by checking all possible arrangements.
    let mut max_happiness: isize = 0;
    let mut best_arrangement: Vec<&str> = Vec::new();
    for permutation in permutohedron::Heap::new(&mut people) {
        let happiness = calculate_total_happiness(&permutation, &relationships, seats);
        if happiness > max_happiness {
            max_happiness = happiness;
            best_arrangement = permutation;
        }
    }

    print!("The highest happiness is {} with the following arrangement: ", max_happiness);
    print!("{}", best_arrangement[0]);
    for x in best_arrangement.iter().skip(1) { print!(" -> {}", x); }
    print!("\n");
}

/// Parses each line for the amount of happiness to {in/de}crease.
fn get_happiness(input: Vec<&str>) -> isize {
    match(input[2] {
        "gain" =>(input[3].parse::<isize>().unwrap(),
        "lose" =>(input[3].parse::<isize>().unwrap() * -1,
        _      => panic!("Invalid")
    }
}

/// Calculate the total happiness of a seating arrangement.
fn calculate_total_happiness(input: &Vec<&str>, relations: &HashMap<[&str;2], isize>,
        seats: usize) -> isize {
    input.iter().enumerate().map(|x| {
        let (left_pos, right_pos) = (
            if x.0 as isize - 1isize == -1 { seats - 1 } else { x.0 - 1 },
            if x.0 as isize + 1isize == seats as isize { 0 } else { x.0 + 1 },
        );
        let (left_key, right_key) = (
            [x.1.clone(), input[left_pos]],
            [x.1.clone(), input[right_pos]]
        );
        relations.get(&left_key).unwrap() + relations.get(&right_key).unwrap()
    }).fold(0, |total, next| total + next)
}
