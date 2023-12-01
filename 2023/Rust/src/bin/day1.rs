use input_extractor::get::{get_input, AoCDate};
extern crate regex;
use regex::{Captures, Regex};

fn get_calibration_value(signal: &str) -> u32 {
    const RADIX: u32 = 10;

    let only_digit: String = signal.chars().filter(|c| c.is_digit(10)).collect();
    let first: u32 = only_digit.chars().nth(0).unwrap().to_digit(RADIX).unwrap();
    let last: u32 = only_digit.chars().last().unwrap().to_digit(RADIX).unwrap();
    first * 10 + last
}

fn replace_string_with_numbers(line: &str) -> String {
    // ugly pattern because regex is left-to-right
    let pattern = "(twone|fiveight|oneight|threeight|nineight|eightwo|eighthree|one|two|three|four|five|six|seven|eight|nine)";
    let re = Regex::new(pattern).unwrap();
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
    }).to_string();
    replaced_line
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

    for line in content.lines() {
        // replace numbers
        let replaced_line = replace_string_with_numbers(line);
        let calibration_value: u32 = get_calibration_value(&replaced_line);
        sum_part2 += calibration_value;
    }
    println!("Sum Part 2 {}", sum_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_string_with_numbers() {
        assert_eq!(replace_string_with_numbers(&"abfive12cd34ef"), "ab512cd34ef");
        assert_eq!(replace_string_with_numbers(&"abfiveight12cd34ef"), "ab5812cd34ef");
        assert_eq!(replace_string_with_numbers(&"abfive12cd34oneeightf"), "ab512cd3418f");

    }

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value(&"ab12cd34ef"), 14);
    }
}
