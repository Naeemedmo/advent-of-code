use input_extractor::get::{get_input, AoCDate};
use std::collections::HashSet;

// Since HashSet only stores unique elements, if the length of the set is equal
// to the length of the string, it means all characters are unique.
fn are_characters_unique(string: &str) -> bool {
    let char_set: HashSet<_> = string.chars().collect();
    char_set.len() == string.len()
}

fn find_first_position(content: &str, msg_length: i32) -> i32 {
    let (mut base, _) = content.split_at(msg_length as usize);
    let mut new_string = base.to_owned();
    if !are_characters_unique(base) {
        for i in 4..content.len() {
            (_, base) = new_string.split_at(1);
            new_string = base.to_owned();

            let new_char = content.chars().nth(i).unwrap().to_string();
            new_string.push_str(&new_char);

            if are_characters_unique(&new_string) {
                return (i + 1) as i32;
            }
        }
    }
    msg_length
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 6 });

    let part1: i32 = find_first_position(&content, 4);
    let part2: i32 = find_first_position(&content, 14);

    println!("Solution part 1 {}", part1);
    println!("Solution part 2 {}", part2);
}
