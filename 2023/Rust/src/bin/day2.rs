use input_extractor::get::{get_input, AoCDate};
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn is_round_possible(color_round: HashMap<String, u32>) -> usize {
    //only 12 red cubes, 13 green cubes, and 14 blue cubes
    let n_blue = color_round.get("blue").unwrap_or(&0);
    let n_green = color_round.get("green").unwrap_or(&0);
    let n_red = color_round.get("red").unwrap_or(&0);

    if n_blue > &14 || n_green > &13 || n_red > &12 {
        return 0;
    }
    return 1;
}

fn game_power(min_needed: HashMap<String, u32>) -> u32 {
    //only 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut n_blue = min_needed.get("blue").unwrap();
    let mut n_green = min_needed.get("green").unwrap();
    let mut n_red = min_needed.get("red").unwrap();
    n_blue = cmp::max(&1, n_blue);
    n_green = cmp::max(&1, n_green);
    n_red = cmp::max(&1, n_red);
    return n_blue * n_red * n_green;
}

fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 2 });
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    let id_re = Regex::new(r"Game (\d+)").unwrap();
    let game_re = Regex::new(r"(\d+)\s*(blue|green|red)").unwrap();

    for line in content.lines() {
        // Get Id
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_number: u32;

        if let Some(caps) = id_re.captures(parts[0]) {
            game_number = caps[1].parse::<u32>().unwrap();
        } else {
            panic!("Could not find game number!")
        }
        let rounds = parts[1].split(";").collect::<Vec<&str>>();
        let mut min_needed = HashMap::new();
        min_needed.insert("blue".to_string(), 0);
        min_needed.insert("green".to_string(), 0);
        min_needed.insert("red".to_string(), 0);
        let mut count_possibility: usize = 0;

        for round in &rounds {
            let mut color_counts = HashMap::new();
            for caps in game_re.captures_iter(round) {
                let count = caps[1].parse::<u32>().unwrap();
                let color = caps[2].to_string();
                color_counts.insert(color.clone(), count);
                let min_value = min_needed.get(&color as &str).unwrap();
                *min_needed.get_mut(&color as &str).unwrap() = cmp::max(*min_value, count);
            }
            count_possibility += is_round_possible(color_counts);
        }
        if count_possibility == rounds.len() {
            sum_part1 += game_number
        }
        sum_part2 += game_power(min_needed);
    }
    println!("Part 1 {}", sum_part1);
    println!("Part 2 {}", sum_part2);
}
