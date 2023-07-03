use input_extractor::get::{get_input, AoCDate};

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 3 });

    let mut sum_priority: u32 = 0;
    for line in content.lines() {
        let length = line.chars().count();
        let (one, two) = line.split_at(length / 2);

        for (pos, item) in ASCII_LOWER.iter().enumerate() {
            if one.matches(*item).count() > 0 && two.matches(*item).count() > 0 {
                sum_priority += (pos as u32) + 1
            }
        }
        for (pos, item) in ASCII_UPPER.iter().enumerate() {
            if one.matches(*item).count() > 0 && two.matches(*item).count() > 0 {
                sum_priority += 26 + (pos as u32) + 1
            }
        }
    }
    println!("Priority result {}", sum_priority);
    let mut lines = content.lines();
    let mut sum_priority_group: u32 = 0;

    while let (Some(one), Some(two), Some(three)) = (lines.next(), lines.next(), lines.next()) {
        for (pos, item) in ASCII_LOWER.iter().enumerate() {
            if one.matches(*item).count() > 0
                && two.matches(*item).count() > 0
                && three.matches(*item).count() > 0
            {
                sum_priority_group += (pos as u32) + 1
            }
        }
        for (pos, item) in ASCII_UPPER.iter().enumerate() {
            if one.matches(*item).count() > 0
                && two.matches(*item).count() > 0
                && three.matches(*item).count() > 0
            {
                sum_priority_group += 26 + (pos as u32) + 1
            }
        }
    }
    println!("Group Priority result {}", sum_priority_group);
}
