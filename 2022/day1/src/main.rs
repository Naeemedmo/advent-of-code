use std::env;
extern crate input_extractor;
use input_extractor::get::get_input;
use input_extractor::build::build_cookie;
use input_extractor::build::build_url;


fn main() {
    let year: i32 = 2022;
    let day: i32 = 1;
    let input_url = build_url(&year, &day);
    let cookie = env::var("AOC_SESSION_COOKIE").expect("Error: AOC_SESSION_COOKIE must be set!");
    let session_cookie = build_cookie(&cookie);

    let content = get_input(&input_url, &session_cookie);
    let mut elves = Vec::new();
    let mut sum_calories: i32 = 0;

    for line in content.lines() {
        if line.is_empty() {
            elves.push(sum_calories);
            sum_calories = 0
        } else {
            sum_calories += line.parse::<i32>().unwrap();
        }
    }
    let max_value = *elves.iter().max().unwrap();
    let index = elves.iter().position(|&r| r == max_value).unwrap();
    println!( "Elf number {} with max calories {}", index, max_value );
    elves.sort_by(|a, b| b.cmp(a));
    println!( "Sum top 3 elves {}", elves[0] + elves[1] + elves[2] );
}
