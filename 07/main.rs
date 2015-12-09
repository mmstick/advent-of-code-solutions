// 0.011s on A4-5000 (1.5 GHz)
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, Not};

fn main() {
    let input = include_str!("input.txt");
    let mut keychain = HashMap::new();
    let mut instructions: Vec<Instruction> = input.lines().map(|x| Instruction::new(x)).collect();
    compute_value_of_a(&mut keychain, &mut instructions);
    println!("The value of a is {}", keychain.get("a").unwrap())
}

/// Loop across the instructions until the value of 'a' is found. When the value of a new
/// variable is discovered, it is added to the hash map. The value of an output variable can
/// only be found when the values of the input arguments are known. As new variables are found,
/// more instructions can be computed.
fn compute_value_of_a(keychain: &mut HashMap<String,u32>, instructions: &mut Vec<Instruction>) {
    let mut id = 0; while !keychain.contains_key("a") {
        if id == instructions.len() { id = 0; }
        if instructions[id].is_solvable(&keychain) {
            let (key, value) = (instructions[id].output, instructions[id].compute(&keychain));
            keychain.insert(String::from(key), value);
            instructions.remove(id);
        } else {
            id += 1;
        }
    }
}

/// Parses the value of an argument and returns it's value. If the argument is already a number,
/// that number is returned. If the argument is a variable, the variable is looked up in the
/// hash map, named `keychain`, and it's value is returned.
macro_rules! get_value {
    ($input:expr, $keychain:expr) => {
        match $input.parse() {
            Ok(number) => number,
            Err(_) => *$keychain.get($input).unwrap(),
        }
    }
}

/// Reduces boilerplate code by parsing the values of the first and second arguments using the
/// pre-existing `get_value!()` macro.
macro_rules! get_values {
    ($arg:expr, $keys:expr) => { (get_value!($arg[0], $keys), get_value!($arg[1], $keys)) }
}

/// Contains an instruction mapping input arguments and their operation to their outputs.
struct Instruction<'a> {
    input:      Vec<&'a str>,
    operation:  Operation,
    output:     &'a str,
}

/// Defines whether we want to perform an AND, OR, NOT, RSHIFT, LSHIFT operation, or to simply copy
/// values without apply any operation.
enum Operation { AND, OR, NOT, RSHIFT, LSHIFT, EQ }

impl<'a> Instruction<'a> {
    /// Parses a `&str` containing the instruction into a new `Instruction` variable.
    fn new(input: &'a str) -> Instruction<'a> {
        let words: Vec<&'a str> = input.split_whitespace().collect();
        let (operation, input, output) = match words[1] {
            "AND"    => (Operation::AND,    vec![words[0], words[2]], words[4]),
            "OR"     => (Operation::OR,     vec![words[0], words[2]], words[4]),
            "RSHIFT" => (Operation::RSHIFT, vec![words[0], words[2]], words[4]),
            "LSHIFT" => (Operation::LSHIFT, vec![words[0], words[2]], words[4]),
            "->"     => (Operation::EQ,     vec![words[0]], words[2]),
            _        => (Operation::NOT,    vec![words[1]], words[3]),
        };
        Instruction { input: input, operation: operation, output: output }
    }

    /// Determines whether the current instruction can be solved at the moment. An instruction can
    /// only be solved when all values for input arguments are known.
    fn is_solvable(&self, keychain: &HashMap<String,u32>) -> bool {
        if self.input.len() == 1 {
            self.input[0].parse::<u32>().is_ok() || keychain.contains_key(self.input[0])
        } else {
            (keychain.contains_key(self.input[0]) || self.input[0].parse::<u32>().is_ok()) &&
            (keychain.contains_key(self.input[1]) || self.input[1].parse::<u32>().is_ok())
        }
    }

    /// If the instruction is solvable, the `compute()` method will compute value of the output.
    /// To compute the output, the instruction's operation is matched and performed after
    /// obtaining the values of the input arguments from the hash map.
    fn compute(&self, keychain: &HashMap<String,u32>) -> u32 {
        match self.operation {
            Operation::AND => {
                let (first, second) = get_values!(self.input, keychain);
                first.bitand(second)
            },
            Operation::OR => {
                let (first, second) = get_values!(self.input, keychain);
                first.bitor(second)
            },
            Operation::LSHIFT => {
                let (first, second) = get_values!(self.input, keychain);
                first.rotate_left(second)
            },
            Operation::RSHIFT => {
                let (first, second) = get_values!(self.input, keychain);
                first.rotate_right(second)
            },
            Operation::NOT => get_value!(self.input[0], keychain).not(),
            Operation::EQ => get_value!(self.input[0], keychain),
        }
    }
}
