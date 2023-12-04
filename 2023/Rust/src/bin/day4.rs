use input_extractor::get::{get_input, AoCDate};
use std::collections::HashSet;

fn part_one_two(input: &str) -> (i32, i32) {
    let mut total: i32 = 0;
    let base: i32 = 2;
    let rows: usize = input.lines().collect::<Vec<_>>().len();
    let mut cards: Vec<i32> = vec![1; rows];
    for (index, line) in input.lines().enumerate() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let numbers = parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers: HashSet<_> = numbers[0].split_whitespace().collect();
        let elf_numbers: HashSet<_> = numbers[1].split_whitespace().collect();

        let n_matches = elf_numbers.intersection(&winning_numbers).count() as u32;
        if n_matches > 0 {
            total += base.pow(n_matches - 1)
        }
        for i in 0..n_matches {
            cards[index + (i + 1) as usize] += cards[index];
        }
    }

    return (total, cards.iter().sum())
}

fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 4 });
    let (one, two) = part_one_two(&content);
    println!("Sum part 1: {} 2: {}", one, two);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parts() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_one_two(input), (13, 30));
    }
}
