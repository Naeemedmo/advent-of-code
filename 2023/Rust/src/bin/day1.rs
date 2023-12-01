use input_extractor::get::{get_input, AoCDate};
extern crate regex;
use regex::{Captures, Regex};

fn get_first_last(signal: &str) -> (u32, u32) {
    const RADIX: u32 = 10;

    let only_digit: String = signal.chars().filter(|c| c.is_digit(10)).collect();
    let first: u32 = only_digit.chars().nth(0).unwrap().to_digit(RADIX).unwrap();
    let last: u32 = only_digit.chars().last().unwrap().to_digit(RADIX).unwrap();
    (first, last)
}


fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn copy_string(input: &str) -> String {
    input.to_string()
}


fn replace_string_with_numbers(line: &str, action: impl Fn(&str) -> String ) -> String {

    let pattern = format!("({})", action("one|two|three|four|five|six|seven|eight|nine"));

    let re = Regex::new(&pattern).unwrap();
    let replaced_line = re.replace_all(&action(line), |cap: &Captures| {
        match action(&cap[0]).as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
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
        let (first, last)= get_first_last(line);
        sum_part1 += first * 10 + last;
    }
    println!("Sum Part 1 {}", sum_part1);

    let mut sum_part2: u32 = 0;

    for line in content.lines() {
        // replace numbers
        let (first, _) = get_first_last(&replace_string_with_numbers(line, copy_string));
        let (last, _) = get_first_last(&replace_string_with_numbers(line, reverse_string));
        sum_part2 += first * 10 + last;
    }
    println!("Sum Part 2 {}", sum_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string(&"abf"), "fba");
    }
    #[test]
    fn test_copy_string() {
        assert_eq!(copy_string(&"abf"), "abf");
    }

    #[test]
    fn test_replace_string_with_numbers() {
        assert_eq!(replace_string_with_numbers(&"abfive12cd34oneightf", reverse_string), "f8no43dc215ba");
        assert_eq!(replace_string_with_numbers(&"abfive12cd34oneightf", copy_string), "ab512cd341ightf");

    }

    #[test]
    fn test_get_first_last() {
        assert_eq!(get_first_last(&"ab12cd34ef"), (1,4));
    }
}
