// 0.060s on A4-5000 (1.5 GHz)
mod grid;
mod instruction;
use instruction::{Instruction, Operation};

fn main() {
    let input = include_str!("input.txt");
    let mut grid = grid::Grid{ grid: [[false; 1000]; 1000] };
    let instructions = input.lines().
        map(|line| Instruction::new(line)).collect::<Vec<Instruction>>();
    for instruction in instructions.iter() {
        match instruction.operation {
            Operation::On => grid.turn_on(instruction.start, instruction.end),
            Operation::Off => grid.turn_off(instruction.start, instruction.end),
            Operation::Toggle => grid.toggle(instruction.start, instruction.end),
        }
    }

    println!("Lights Turned On: {}", grid.turned_on_count());
}
