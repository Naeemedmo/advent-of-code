use input_extractor::get::{get_input, AoCDate};
use itertools::Itertools;

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 1 });

    let elves = content
        .split("\n\n")
        .map(|elf_string| {
            -elf_string
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .k_smallest(3);
    let max_calories: i32 = -elves.clone().take(1).sum::<i32>();
    println!("Elf with max calories {}", max_calories);
    let sum_top_three: i32 = -elves.clone().take(3).sum::<i32>();
    println!("Sum top 3 elves {}", sum_top_three);
}
