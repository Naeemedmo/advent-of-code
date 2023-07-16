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

    for line in content.lines() {
        let elements: Vec<_> = line.split_whitespace().collect();
        let mut value: i32 = 0;
        if elements[0] == "addx" {
            value = FromStr::from_str(elements[1]).unwrap();
            cycle_number += 2;
        } else {
            cycle_number += 1;
        }
        instruction.push(RayTube::new(cycle_number, value))
    }

    let signals = [20, 60, 100, 140, 180, 220];
    let mut strength = 0;

    for signal in signals.iter() {
        let result: i32 = 1 + instruction
            .iter()
            .filter_map(|ray| {
                if ray.cycle < *signal {
                    Some(ray.value)
                } else {
                    None
                }
            })
            .sum::<i32>();
        strength += *signal * result;
    }

    println!("Signal strength {:?} ", strength);

    let n_cycles = instruction.iter().map(|ray| ray.cycle).last().unwrap();

    let mut screen: String = Default::default();
    let mut screen_width: String = Default::default();
    let mut cursor = 1; //always keep the middle
    let mut position: i32 = 0;
    for n in 1..n_cycles + 2 {
        if screen_width.len() == 40 {
            screen.push_str(&screen_width);
            screen.push_str("\n");
            screen_width = "".to_string();
            position = 0;
        };
        let value: i32 = instruction
            .iter()
            .find_map(|ray| {
                if ray.cycle == n {
                    Some(ray.value)
                } else {
                    None
                }
            })
            .unwrap_or(0);
        if (cursor - 1..cursor + 2).contains(&position) {
            screen_width.push_str("#")
        } else {
            screen_width.push_str(".")
        }
        cursor += value;
        position += 1;
    }
    println!("{}", screen)
}
