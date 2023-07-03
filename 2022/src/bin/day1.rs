use input_extractor::get::{get_input, AoCDate};
use itertools::Itertools;
use microbench::{self, Options};

fn benchmark_one(content: &str) -> (i32, i32) {
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
    let sum_top_three: i32 = -elves.clone().take(3).sum::<i32>();
    return (max_calories, sum_top_three);
}

fn benchmark_two(content: &str) -> (i32, i32) {
    let elves = content
        .split("\n\n")
        .map(|elf_string| {
            elf_string
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev();
    let max_calories: i32 = elves.clone().take(1).sum::<i32>();
    let sum_top_three: i32 = elves.clone().take(3).sum::<i32>();
    return (max_calories, sum_top_three);
}

fn benchmark_base(content: &str) -> (i32, i32) {
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
    elves.sort_by(|a, b| b.cmp(a));
    let max_calories = elves[0];
    let sum_top_three = elves[0] + elves[1] + elves[2];
    return (max_calories, sum_top_three);
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 1 });

    let (max_calories, sum_top_three) = benchmark_base(&content);
    println!("Elf with max calories {}", max_calories);
    println!("Sum top 3 elves {}", sum_top_three);

    let (max_calories, sum_top_three) = benchmark_one(&content);
    println!("Elf with max calories {}", max_calories);
    println!("Sum top 3 elves {}", sum_top_three);

    let (max_calories, sum_top_three) = benchmark_two(&content);
    println!("Elf with max calories {}", max_calories);
    println!("Sum top 3 elves {}", sum_top_three);

    let options = Options::default();
    microbench::bench(&options, "experiment_base", || benchmark_base(&content));
    microbench::bench(&options, "experiment_one", || benchmark_one(&content));
    microbench::bench(&options, "experiment_two", || benchmark_two(&content));
}
