use input_extractor::get::{get_input, AoCDate};
use num::Integer;
use std::io::{self};
use std::str::FromStr;

fn multiply<T: Integer + Copy>(a: T, b: T) -> T {
    a * b
}

fn square<T: Integer + Copy>(a: T, _: T) -> T {
    a * a
}

fn double<T: Integer + Copy>(a: T, _: T) -> T {
    a + a
}

fn add<T: Integer + Copy>(a: T, b: T) -> T {
    a + b
}

#[derive(Clone)]
pub struct Operation<T: Integer + Copy> {
    operation: fn(T, T) -> T,
    operator: T,
}

impl<T: Integer + Copy> Operation<T> {
    pub fn call(&self, value: T) -> T {
        (self.operation)(value, self.operator)
    }
}

#[derive(Clone)]
pub struct Monkey {
    n_items_inspected: u64,
    items_list: Vec<u64>,
    operation: Operation<u64>,
    divisor: u64,
    true_receiver: usize,
    false_receiver: usize,
}

impl Monkey {
    pub fn perform_operation(&self, stress: u64) -> u64 {
        self.operation.call(stress)
    }
    pub fn perform_test(&self, stress: u64) -> bool {
        stress % self.divisor == 0
    }
}

fn get_operation(pattern: &str) -> Result<Operation<u64>, io::Error> {
    let op_elements: Vec<_> = pattern.split_whitespace().collect();
    let last_element = op_elements[5];
    match op_elements[4] {
        "*" => match last_element {
            "old" => Ok(Operation {
                operation: square,
                operator: 1, // ignored
            }),
            value => Ok(Operation {
                operation: multiply,
                operator: FromStr::from_str(value).unwrap(),
            }),
        },
        "+" => match last_element {
            "old" => Ok(Operation {
                operation: double,
                operator: 1, // ignored
            }),
            value => Ok(Operation {
                operation: add,
                operator: FromStr::from_str(value).unwrap(),
            }),
        },
        _ => panic!("Failed to create an operation!"),
    }
}

fn monkey_business<T: Fn(u64) -> u64>(
    mut monkey_list: Vec<Monkey>,
    rounds: usize,
    relief: T,
) -> u64 {
    for _ in 0..rounds {
        for monkey_index in 0..monkey_list.len() {
            for ind in 0..monkey_list[monkey_index].items_list.len() {
                let stress = monkey_list[monkey_index].items_list[ind];
                let mut new_stress_level = monkey_list[monkey_index].perform_operation(stress);
                // bored monkey
                new_stress_level = relief(new_stress_level);
                let receiver: usize = if monkey_list[monkey_index].perform_test(new_stress_level) {
                    monkey_list[monkey_index].true_receiver
                } else {
                    monkey_list[monkey_index].false_receiver
                };
                monkey_list[monkey_index].n_items_inspected += 1;
                monkey_list[receiver].items_list.push(new_stress_level);
            }
            monkey_list[monkey_index].items_list.clear();
        }
    }
    let mut monkey_business: Vec<_> = monkey_list
        .iter()
        .map(|monkey| monkey.n_items_inspected)
        .collect();
    monkey_business.sort_by(|a, b| b.cmp(a));
    monkey_business[0] * monkey_business[1]
}

fn get_monkey_list_and_magic_number(content: &str) -> (Vec<Monkey>, u64) {
    let mut monkey_list = Vec::new();
    let content_list: Vec<&str> = content.lines().collect();
    let n_lines = content_list.len();
    let mut index = 0;
    let mut magic_number: u64 = 1;

    while index < n_lines {
        let line = content_list[index];
        if !line.is_empty() {
            // begin reading monkey information
            if line.contains("Monkey") {
                // Extract numbers from the pattern
                let items_list: Vec<u64> = content_list[index + 1]
                    .split(|c: char| !c.is_ascii_digit() && c != '-')
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let operation = match get_operation(content_list[index + 2]) {
                    Ok(op) => op,
                    Err(_) => panic!("Monkey died!"),
                };

                let test_elements: Vec<_> = content_list[index + 3].split_whitespace().collect();
                let divisor: u64 = FromStr::from_str(test_elements[3]).unwrap();
                magic_number *= divisor;

                let true_receiver: usize = content_list[index + 4]
                    .split_whitespace()
                    .find_map(|word| word.parse::<usize>().ok())
                    .unwrap();
                let false_receiver: usize = content_list[index + 5]
                    .split_whitespace()
                    .find_map(|word| word.parse::<usize>().ok())
                    .unwrap();
                monkey_list.push(Monkey {
                    n_items_inspected: 0,
                    items_list,
                    operation,
                    divisor,
                    true_receiver,
                    false_receiver,
                });
                index += 5;
            }
        }
        index += 1;
    }
    (monkey_list, magic_number)
}

fn main() {
    let content = get_input(AoCDate {
        year: 2022,
        day: 11,
    });
    let (monkey_list, magic_number) = get_monkey_list_and_magic_number(&content);

    let relief_part1 = |x| x / 3;
    let monkey_business_part1 = monkey_business(monkey_list.clone(), 20, relief_part1);
    println!("Monkey business Part1 {}", monkey_business_part1);

    let relief_part2 = |x| x % magic_number;
    let monkey_business_part2 = monkey_business(monkey_list, 10000, relief_part2);
    println!("Monkey business Part2 {}", monkey_business_part2);
}
