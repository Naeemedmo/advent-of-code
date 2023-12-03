use input_extractor::get::{get_input, AoCDate};
use regex::Regex;


fn match_symbol(data: &char) -> bool {
    match *data {
        '0'..='9' | '.'  => false,
       _ => true,
    }
}

fn match_star(data: &char) -> bool {
    match *data {
        '*' => true,
        _ => false,
    }
}

fn left_walk(input: &str, pos: usize) -> usize {
    let mut left_pos = pos;
    while let Some(c) = input.chars().nth(left_pos) {
        match c.is_digit(10) {
            true => {if left_pos != 0 { left_pos -= 1} else {break}},
            false => {break},
        }
    }
    return left_pos
}

fn right_walk(input: &str, pos: usize) -> usize {
    let mut right_pos = pos;
    while let Some(c) = input.chars().nth(right_pos) {
        match c.is_digit(10) {
            true => {right_pos += 1},
            false => {break},
        }
    }
    return right_pos
}


fn check_below_above_star(input: &str, pos: usize, number: u32) -> u32 {
    // pos is where the star is

    let re = Regex::new(r"\d+").unwrap();
    let mut found: u32;

    if input.chars().nth(pos).unwrap().is_digit(10) { // number in the middle
        let left_pos = left_walk(input, pos);
        let right_pos = right_walk(input, pos);
        found = match re.find(&input[left_pos..right_pos]) {
            Some(m) => Some(m).unwrap().as_str().parse::<u32>().unwrap_or(0),
            _ => 0
        }
    } else { // number on the side

        // check left side
        let mut left_pos = if pos > 0 { pos - 1} else {0};
        left_pos = left_walk(input, left_pos);
        found = match re.find(&input[left_pos..pos]) {
            Some(m) => Some(m).unwrap().as_str().parse::<u32>().unwrap_or(0),
            _ => 0
        };
        if found != 0 && found != number {
            return found
        };
        // check right side
        let right_pos = right_walk(input, pos + 1);
        found = match re.find(&input[pos..right_pos]) {
            Some(m) => Some(m).unwrap().as_str().parse::<u32>().unwrap_or(0),
            _ => 0
        }
    }
    if found == number { found = 0}

    return found

}


fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 3 });

    let rows: usize = content.lines().collect::<Vec<_>>().len();
    let columns: usize = content
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>()
        .len();

    // read all lines in a vector
    let mut string_vector: Vec<String> = Vec::new();
    for line in content.lines() {
        string_vector.push(line.to_string())
    }
    let re = Regex::new(r"\d+").unwrap();
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    // Iterate over vector and decide on what to consider
    for row in 0..string_vector.len() {
        // Get all numbers in row
        for mat in re.find_iter(&string_vector[row]) {
            let number = mat.as_str().parse::<u32>().unwrap();

            let pos = mat.start();
            let end_position = mat.end();

            let p_start = match pos {
                0 => 0,
                _ => pos - 1
            };
            let mut p_end = end_position + 1;
            if p_end > columns {
                p_end = columns;
            };
            let r_start = match row {
                0 => 0,
                _ => row -1,
            };
            let mut r_end = row + 2;
            if r_end > rows {
                r_end = rows;
            };
            let mut found_symbol = false;
            for p in p_start..p_end {
                for r in r_start..r_end {
                    let character = string_vector[r].chars().nth(p).unwrap();

                    if match_star(&character){
                        let rs_start = match r {
                            0 => 0,
                            _ => r - 1
                        };
                        let mut rs_end = r + 2;
                        if rs_end > rows { rs_end = rows};

                        for rs in rs_start..rs_end {
                            let gear_number = check_below_above_star(&string_vector[rs], p, number);
                            sum_part2 += gear_number * number;
                        }
                    };
                    if match_symbol(&character) && !found_symbol{
                        sum_part1 += number;
                        found_symbol = true;
                    };
                }
            }
        }

    }
    sum_part2 = sum_part2 /2;
    println!("Part 1 {}", sum_part1);
    println!("Part 2 {}", sum_part2);

}
