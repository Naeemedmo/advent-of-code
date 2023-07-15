use input_extractor::get::{get_input, AoCDate};
use std::str::FromStr;

// Values at the end of cycle
#[derive(Debug)]
pub struct RayTube {
    cycle: i32,
    value: i32,
}

impl RayTube {
    fn new(cycle: i32, value: i32) -> Self {
        RayTube { cycle, value }
    }
}

fn main() {
    let content = get_input(AoCDate {
        year: 2022,
        day: 10,
    });

    let mut instruction = Vec::new();
    let mut cycle_number: i32 = 0;
    let mut value: i32 = 1;

    for line in content.lines() {
        let elements: Vec<_> = line.split_whitespace().collect();

        if elements[0] == "addx" {
            let value_at_end: i32 = FromStr::from_str(elements[1]).unwrap();
            value += value_at_end;
            cycle_number += 2;
        } else {
            cycle_number += 1;
        }
        instruction.push(RayTube::new(cycle_number, value))
    }

    let signals = [20, 60, 100, 140, 180, 220];
    let mut strength = 0;

    for signal in signals.iter() {
        let result = instruction
            .iter()
            .filter_map(|ray| {
                if ray.cycle < *signal {
                    Some(ray.value)
                } else {
                    None
                }
            })
            .last()
            .unwrap();
        strength += *signal * result;
    }

    println!("Signal strength {:?} ", strength);
}
