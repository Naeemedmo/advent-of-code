use input_extractor::get::{get_input, AoCDate};
use std::str::FromStr;

pub struct Monkey {
    n_items_inspected: u64,
    items_list: Vec<u64>,
    operation: Box<dyn Fn(&Monkey, &u64) -> u64>,
    test: Box<dyn Fn(&Monkey, &u64) -> bool>,
    true_receiver: usize,
    false_receiver: usize,
}

impl Monkey {
    pub fn new(
        items_list: Vec<u64>,
        operation: Box<dyn Fn(&Monkey, &u64) -> u64>,
        test: Box<dyn Fn(&Monkey, &u64) -> bool>,
        true_receiver: usize,
        false_receiver: usize,
    ) -> Monkey {
        let n_items_inspected = 0;
        Monkey {
            n_items_inspected,
            items_list,
            operation,
            test,
            true_receiver,
            false_receiver,
        }
    }

    pub fn perform_operation(&self, stress: &u64) -> u64 {
        (self.operation)(&self, &stress)
    }
    pub fn perform_test(&self, stress: &u64) -> bool {
        (self.test)(&self, &stress)
    }
}

fn get_custom_function(pattern: &str) -> Box<dyn Fn(&Monkey, &u64) -> u64> {
    let op_elements: Vec<_> = pattern.split_whitespace().collect();
    if op_elements.iter().any(|s| s.contains("*")) {
        if op_elements[5].contains("old") {
            Box::new(|_, stress| stress * stress)
        } else {
            let value: u64 = op_elements[5].parse().unwrap();
            Box::new(move |_, stress| stress * value)
        }
    } else if op_elements.iter().any(|s| s.contains("+")) {
        if op_elements[5].contains("old") {
            Box::new(|_, stress| stress + stress)
        } else {
            let value: u64 = op_elements[5].parse().unwrap();
            Box::new(move |_, stress| stress + value)
        }
    } else {
        Box::new(|_, stress| stress.clone()) // Default operation
    }
}

fn monkey_business(content: &str, rounds: usize, relief: Option<u64>) -> u64 {
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
                    .split(|c: char| !c.is_digit(10) && c != '-')
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let custom_operation = get_custom_function(&content_list[index + 2]);

                let test_elements: Vec<_> = content_list[index + 3].split_whitespace().collect();
                let test_value: u64 = FromStr::from_str(test_elements[3]).unwrap();
                magic_number *= test_value;
                let custom_test =
                    move |_: &Monkey, stress: &u64| -> bool { (stress % test_value) == 0 };

                let true_receiver: usize = content_list[index + 4]
                    .split_whitespace()
                    .find_map(|word| word.parse::<usize>().ok())
                    .unwrap();
                let false_receiver: usize = content_list[index + 5]
                    .split_whitespace()
                    .find_map(|word| word.parse::<usize>().ok())
                    .unwrap();

                monkey_list.push(Monkey::new(
                    items_list,
                    custom_operation,
                    Box::new(custom_test),
                    true_receiver,
                    false_receiver,
                ));

                index += 5;
            }
        }
        index += 1;
    }

    for _ in 0..rounds {
        for monkey_index in 0..monkey_list.len() {
            for ind in 0..monkey_list[monkey_index].items_list.len() {
                let stress = &monkey_list[monkey_index].items_list[ind];
                let mut new_stress_level = monkey_list[monkey_index].perform_operation(&stress);
                // bored monkey
                match relief {
                    Some(value) => new_stress_level = new_stress_level / value,
                    None => new_stress_level = new_stress_level % magic_number,
                }
                let receiver: usize = if monkey_list[monkey_index].perform_test(&new_stress_level) {
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

fn main() {
    let content = get_input(AoCDate {
        year: 2022,
        day: 11,
    });
    let monkey_business_part1 = monkey_business(&content, 20, Some(3));
    println!("Monkey business Part1 {}", monkey_business_part1);
    let monkey_business_part2 = monkey_business(&content, 10000, None);
    println!("Monkey business Part2 {}", monkey_business_part2);
}
