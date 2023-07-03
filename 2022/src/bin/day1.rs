use input_extractor::get::{get_input, AoCDate};

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 1 });
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
    println!("Elf number {} with max calories {}", index, max_value);
    elves.sort_by(|a, b| b.cmp(a));
    println!("Sum top 3 elves {}", elves[0] + elves[1] + elves[2]);
}
