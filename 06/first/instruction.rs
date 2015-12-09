/// Contains an individual lighting instruction.
/// - operation selects the action: {turn on, turn off, toggle}.
/// - start contains the starting x,y position.
/// - end contains the ending x,y position.
pub struct Instruction {
    pub operation: Operation,
    pub start:     [usize; 2],
    pub end:       [usize; 2],
}

/// Determines whether we should turn on, turn off or toggle lights
pub enum Operation { On, Off, Toggle }

impl Instruction {
    // Create a new instruction from a line of text.
    pub fn new(line: &str) -> Instruction {
        let words: Vec<&str> = line.split_whitespace().collect();

        // Matches the operation defined in the string.
        let operation = match words[0] {
            "toggle" => Operation::Toggle,
            "turn" => if words[1] == "on" { Operation::On } else { Operation::Off },
            _ => panic!("Operation Not Supported: {}", words[0]),
        };

        // Collects the positions from the string as [usize;2] arrays.
        let (start, end) = {
            let (start, end) = match operation {
                Operation::Toggle => (
                    words[1].split(',').collect::<Vec<&str>>(),
                    words[3].split(',').collect::<Vec<&str>>(),
                ),
                _ => (
                    words[2].split(',').collect::<Vec<&str>>(),
                    words[4].split(',').collect::<Vec<&str>>(),
                ),
            };
            (
                [start[0].parse().unwrap(), start[1].parse().unwrap()],
                [end[0].parse().unwrap(), end[1].parse().unwrap()],
            )
        };

        // Returning a new instruction.
        Instruction { operation: operation, start: start, end: end }
    }
}
