use input_extractor::get::{get_input, AoCDate};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CamelHand {
    hand: String,
    bid: u32,
}

impl CamelHand {
    fn new(hand: String, bid: u32) -> Self {
        CamelHand { hand, bid }
    }
}

fn count_uniqueness(s: &str) -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let counter = char_counts.entry(c).or_insert(0);
        *counter += 1;
    }

    char_counts
}

fn char_to_int(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unknown character in hand! {}", c),
    }
}

fn joker_replacement(unique: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut updated_unique: HashMap<char, usize> = unique.clone();
    if let Some(&number_j) = updated_unique.get(&'J') {
        updated_unique.remove(&'J');
        if let Some((&max_key, &max_value)) = updated_unique.iter().max_by_key(|&(_, value)| value)
        {
            let keys_with_max_value: Vec<_> = updated_unique
                .iter()
                .filter(|&(_, &value)| value == max_value)
                .map(|(&key, _)| key)
                .collect();
            let mut best_key = max_key;
            if keys_with_max_value.len() > 1 {
                // we need to choose the best key
                for &current_key in &keys_with_max_value {
                    if char_to_int(current_key) > char_to_int(best_key) {
                        best_key = current_key;
                    }
                }
                *updated_unique.entry(best_key).or_insert(0) += number_j;
            } else {
                *updated_unique.entry(max_key).or_insert(0) += number_j;
            }
        }
    }
    updated_unique
}

fn rank_games_part2(left_hand: &CamelHand, right_hand: &CamelHand) -> Ordering {
    let mut unique_left = count_uniqueness(&left_hand.hand);
    let mut unique_right = count_uniqueness(&right_hand.hand);

    // if one of the hands in all joker
    if unique_left.len() == 1 && unique_left.get(&'J').unwrap_or(&0) > &0 {
        unique_right = joker_replacement(&unique_right);
        if unique_right.len() > 1 {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    if unique_right.len() == 1 && unique_right.get(&'J').unwrap_or(&0) > &0 {
        unique_left = joker_replacement(&unique_left);
        if unique_left.len() > 1 {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

    unique_left = joker_replacement(&unique_left);
    unique_right = joker_replacement(&unique_right);

    if unique_left.len() > unique_right.len() {
        return Ordering::Less;
    } else if unique_left.len() < unique_right.len() {
        return Ordering::Greater;
    } else {
        //equal scenarios
        if unique_left.values().max().unwrap() > unique_right.values().max().unwrap() {
            return Ordering::Greater; // 4 of kind > full house or 3 of kind > two pairs
        } else if unique_left.values().max().unwrap() < unique_right.values().max().unwrap() {
            return Ordering::Less;
        } else {
            // rank
            for (left, right) in left_hand.hand.chars().zip(right_hand.hand.chars()) {
                if left == 'J' && right != 'J' {
                    return Ordering::Less;
                } else if left != 'J' && right == 'J' {
                    return Ordering::Greater;
                } else if char_to_int(left) > char_to_int(right) {
                    return Ordering::Greater;
                } else if char_to_int(left) < char_to_int(right) {
                    return Ordering::Less;
                }
            }
        }
    }
    // should never happen
    return Ordering::Equal;
}

fn rank_games(left_hand: &CamelHand, right_hand: &CamelHand) -> Ordering {
    let unique_left = count_uniqueness(&left_hand.hand);
    let unique_right = count_uniqueness(&right_hand.hand);
    if unique_left.len() > unique_right.len() {
        return Ordering::Less;
    } else if unique_left.len() < unique_right.len() {
        return Ordering::Greater;
    } else {
        //equal scenarios
        if unique_left.values().max().unwrap() > unique_right.values().max().unwrap() {
            return Ordering::Greater; // 4 of kind > full house or 3 of kind > two pairs
        } else if unique_left.values().max().unwrap() < unique_right.values().max().unwrap() {
            return Ordering::Less;
        } else {
            // rank
            for (left, right) in left_hand.hand.chars().zip(right_hand.hand.chars()) {
                if char_to_int(left) > char_to_int(right) {
                    return Ordering::Greater;
                } else if char_to_int(left) < char_to_int(right) {
                    return Ordering::Less;
                }
            }
        }
    }
    // should never happen
    return Ordering::Equal;
}

fn part_one_two(input: &str) -> (u32, u32) {
    let mut game: Vec<CamelHand> = Vec::new();
    for line in input.lines() {
        let elements: Vec<_> = line.split_whitespace().collect();
        let bid = elements[1].parse::<u32>().unwrap_or(0);
        game.push(CamelHand::new(elements[0].to_string(), bid));
    }
    game.sort_by(|a, b| rank_games(a, b));
    let game_score: u32 = game
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 + 1) * value.bid)
        .sum();

    game.sort_by(|a, b| rank_games_part2(a, b));
    let game_score_p2: u32 = game
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 + 1) * value.bid)
        .sum();

    return (game_score, game_score_p2);
}
fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 7 });
    let (one, two) = part_one_two(&content);
    println!("Sum part 1: {} 2: {}", one, two);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parts_advent() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_one_two(input), (6440, 5905));
    }
    #[test]
    fn test_parts_reddit() {
        let input = "2345A 1
        Q2KJJ 13
        Q2Q2Q 19
        T3T3J 17
        T3Q33 11
        2345J 3
        J345A 2
        32T3K 5
        T55J5 29
        KK677 7
        KTJJT 34
        QQQJA 31
        JJJJJ 37
        JAAAA 43
        AAAAJ 59
        AAAAA 61
        2AAAA 23
        2JJJJ 53
        JJJJ2 41";
        assert_eq!(part_one_two(input), (6592, 6839));
    }
}
