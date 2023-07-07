use input_extractor::get::{get_input, AoCDate};

fn check_if_contains_only_numbers(s: &str) -> bool {
    let without_whitespace: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    without_whitespace.parse::<f64>().is_ok()
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 5 });

    let mut found_n_crate: bool = false;

    let mut intermediate: String = Default::default();
    let mut n_items: usize = 0;
    let mut crate_matrix_part1: Vec<Vec<String>> = Vec::new();
    let mut crate_matrix_part2: Vec<Vec<String>> = Vec::new();
    let mut crate_size: usize = 0;

    for line in content.lines() {
        if check_if_contains_only_numbers(line) {
            found_n_crate = true;

            crate_size = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .max()
                .unwrap() as usize;
            let mut crates_pos = Vec::new();
            let mut crate_matrix: Vec<Vec<String>> =
                vec![vec!["".to_string(); n_items]; crate_size];

            // find position of crates in text
            for n in 1..crate_size + 1 {
                crates_pos.push(line.find(&n.to_string()).unwrap());
            }
            // Lets read the information about the crates
            let mut n = n_items;
            for info in intermediate.lines() {
                for (i, x) in crates_pos.iter().enumerate() {
                    let alphabet = info.chars().nth(*x).unwrap();
                    if !alphabet.is_whitespace() {
                        crate_matrix[i][n - 1] = alphabet.to_string();
                    }
                }
                n -= 1;
            }
            for matrix in crate_matrix.iter().take(crate_size) {
                let mut row: Vec<String> = Vec::new();
                for col in (1..n_items + 1).rev() {
                    let value = &matrix[col - 1];
                    if !value.is_empty() {
                        row.push(value.to_string());
                    }
                }
                row.reverse();
                crate_matrix_part1.push(row);
            }
            crate_matrix_part2 = crate_matrix_part1.clone();
        } else if !found_n_crate {
            n_items += 1;
            intermediate.push_str(line);
            intermediate.push('\n');
        } else if line.contains("move") {
            let split_parts: Vec<&str> = line.split_whitespace().collect();
            let value: usize = split_parts[1].parse().unwrap();
            let from: usize = split_parts[3].parse().unwrap();
            let to: usize = split_parts[5].parse().unwrap();

            // part 1

            for _ in 0..value {
                let value_to_move: String = crate_matrix_part1[from - 1].pop().unwrap().to_string();
                crate_matrix_part1[to - 1].push(value_to_move);
            }

            // part 2
            let length = crate_matrix_part2[from - 1].len();

            let mut tail = crate_matrix_part2[from - 1].split_off(length - value);
            crate_matrix_part2[to - 1].append(&mut tail);

        }
    }
    let mut result: String = "".to_owned();
    let mut result_part2: String = "".to_owned();
    for row in 0..crate_size {
        result.push_str(crate_matrix_part1[row].last().unwrap());
        result_part2.push_str(crate_matrix_part2[row].last().unwrap());
    }
    println!("Result Part 1 {} Result Part 2 {} ", result, result_part2);
}
