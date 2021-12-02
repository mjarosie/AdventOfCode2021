use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

struct SubmarineInstruction {
    direction: SubmarineInstructionDirection,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum SubmarineInstructionDirection {
    Forward,
    Down,
    Up,
}

struct BoatState {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl FromStr for SubmarineInstructionDirection {
    type Err = ();
    fn from_str(input: &str) -> Result<SubmarineInstructionDirection, Self::Err> {
        match input {
            "forward" => Ok(SubmarineInstructionDirection::Forward),
            "down" => Ok(SubmarineInstructionDirection::Down),
            "up" => Ok(SubmarineInstructionDirection::Up),
            _ => Err(()),
        }
    }
}

fn main() {
    let filename = "example_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong when reading a file");
    let lines_str = contents.lines();
    let instructions = lines_str
        .map(|l| {
            let mut line_sections = l.split(" ");
            let direction =
                SubmarineInstructionDirection::from_str(line_sections.nth(0).unwrap()).unwrap();
            let value = line_sections.nth(0).unwrap().parse::<i32>().unwrap();
            return SubmarineInstruction { direction, value };
        })
        .collect::<VecDeque<SubmarineInstruction>>();

    let result = instructions.iter().fold(
        BoatState {
            horizontal_position: 0,
            depth: 0,
            aim: 0
        },
        |acc, i| match i.direction {
            SubmarineInstructionDirection::Forward => BoatState {
                horizontal_position: acc.horizontal_position + i.value,
                depth: acc.depth + acc.aim * i.value,
                aim: acc.aim,
            },
            SubmarineInstructionDirection::Up => BoatState {
                horizontal_position: acc.horizontal_position,
                depth: acc.depth,
                aim: acc.aim - i.value,
            },
            SubmarineInstructionDirection::Down => BoatState {
                horizontal_position: acc.horizontal_position,
                depth: acc.depth,
                aim: acc.aim + i.value,
            },
        },
    );

    println!(
        "{} * {} = {}",
        result.horizontal_position,
        result.depth,
        result.horizontal_position * result.depth
    );
}
