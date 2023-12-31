use input_extractor::get::{get_input, AoCDate};
use std::collections::HashMap;

const fn next(index: u32) -> u32 {
    const NEXT_ELEMENT: [u32; 3] = [2, 3, 1];
    NEXT_ELEMENT[(index - 1) as usize]
}

fn match_result(elf_choice: u32, my_choice: u32) -> u32 {
    // If both numbers are the same, no one wins
    // If both numbers are consecutive, the bigger one wins
    // If both numbers aren’t consecutive, the smaller one wins
    if elf_choice == my_choice {
        return my_choice + 3;
    }
    if next(elf_choice) == my_choice {
        return my_choice + 6;
    }
    my_choice
}

fn inverse_match_result(outcome: &str, elf_choice: u32) -> u32 {
    // X means you need to lose,
    // Y means you need to end the round in a draw,
    // and Z means you need to win. Good luck!"
    if outcome == "Y" {
        return elf_choice + 3;
    }
    let winning_choice: u32 = next(elf_choice);
    if outcome == "Z" {
        return winning_choice + 6;
    }
    next(winning_choice)
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 2 });

    // score
    let elf_result = HashMap::from([
        ("A", 1), //Rock
        ("B", 2), //Paper
        ("C", 3), //Scissors
    ]);
    let my_result = HashMap::from([
        ("X", 1), //Rock
        ("Y", 2), //Paper
        ("Z", 3), //Scissors
    ]);

    let mut sum_score: u32 = 0;
    let mut sum_score_part2: u32 = 0;
    for line in content.lines() {
        if !line.is_empty() {
            let result: Vec<&str> = line.split(' ').collect();
            let elf_choice = elf_result.get(&result[0]).unwrap();
            let my_choice = my_result.get(&result[1]).unwrap();
            let score: u32 = match_result(*elf_choice, *my_choice);
            let score_part2: u32 = inverse_match_result(result[1], *elf_choice);
            sum_score += score;
            sum_score_part2 += score_part2;
        }
    }
    println!("Match result {}", sum_score);
    println!("Match result Part2 {}", sum_score_part2);
}
