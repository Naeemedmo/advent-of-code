use input_extractor::get::{get_input, AoCDate};
extern crate regex;
use regex::{Captures, Regex};

fn get_calibration_value(line: &str) -> u32 {
    const RADIX: u32 = 10;

    let only_digit: String = line.chars().filter(|c| c.is_digit(10)).collect();
    let first: u32 = only_digit.chars().nth(0).unwrap().to_digit(RADIX).unwrap();
    let last: u32 = only_digit.chars().last().unwrap().to_digit(RADIX).unwrap();
    first * 10 + last
}

fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 1 });
    let mut sum_part1: u32 = 0;

    for line in content.lines() {
        // First remove all non digits
        let calibration_value: u32 = get_calibration_value(line);
        sum_part1 += calibration_value;
    }
    println!("Sum Part 1 {}", sum_part1);

    let mut sum_part2: u32 = 0;
    let re = Regex::new("(twone|fiveight|oneight|threeight|nineight|eightwo|eighthree|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in content.lines() {
        // replace numbers
        let replaced_line = re.replace_all(line, |cap: &Captures| {
            match &cap[0] {
                "oneight" => "18",
                "one" => "1",
                "twone" => "21",
                "two" => "2",
                "threeight" => "38",
                "three" => "3",
                "four" => "4",
                "fiveight" => "58",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eightwo" => "82",
                "eighthree" => "83",
                "eight" => "8",
                "nineight" => "98",
                "nine" => "9",
                _ => panic!("We should never get here!"),
            }
            .to_string()
        });
        let calibration_value: u32 = get_calibration_value(&replaced_line);
        sum_part2 += calibration_value;
    }
    println!("Sum Part 2 {}", sum_part2);
}
