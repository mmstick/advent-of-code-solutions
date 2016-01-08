// 0.096s on A4-5000 (1.5 GHz)
extern crate permutohedron;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    // Obtain locations and route possibilities
    let mut locations: Vec<&str> = Vec::new();
    let mut routes: HashMap<[&str;2], usize> = HashMap::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let (first, second) = ([words[0], words[2]], [words[2], words[0]]);
        let distance = words[4].parse::<usize>().unwrap();
        routes.insert(first, distance);
        routes.insert(second, distance);
        if !locations.contains(&words[0]) { locations.push(words[0]); }
        if !locations.contains(&words[2]) { locations.push(words[2]); }
    }

    // Calculate the shortest and longest distances/routes
    let mut shortest_distance: usize = 65535;
    let mut shortest_route: Vec<&str> = Vec::new();
    let mut longest_distance: usize = 0;
    let mut longest_route: Vec<&str> = Vec::new();
    for permutation in permutohedron::Heap::new(&mut locations) {
        let distance = calculate_distance(&permutation, &routes);
        if distance < shortest_distance {
            shortest_route = permutation.clone();
            shortest_distance = distance;
        }
        if distance > longest_distance {
            longest_route = permutation.clone();
            longest_distance = distance;
        }
    }

    // Print the results
    print!("The shortest route is {} miles: ", shortest_distance);
    print_route(&shortest_route);
    print!("\nThe longest route is {} miles: ", longest_distance);
    print_route(&longest_route);
}

/// Calculates the distance that must be traveled for a given input route.
fn calculate_distance(input: &Vec<&str>, map: &HashMap<[&str;2],usize>) -> usize {
    let mut distance: usize = 0;
    let mut iterator = input.iter();
    let mut previous = iterator.next().unwrap();
    for next in iterator {
        distance += *map.get(&[previous.clone(),next.clone()]).unwrap();
        previous = next;
    }
    return distance;
}

/// Prints the given route to stdout.
fn print_route(locations: &Vec<&str>) {
    let mut iterator = locations.iter();
    print!("{}", iterator.next().unwrap());
    for x in iterator { print!(" -> {}", x); }
    print!("\n");
}
