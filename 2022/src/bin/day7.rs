use input_extractor::get::{get_input, AoCDate};
use std::str::FromStr;

pub struct FileInfo {
    name: String,
    size: i32,
}

impl FileInfo {
    fn new(name: String, size: i32) -> Self {
        FileInfo { name, size }
    }

    fn get_parent(&self, name: &str) -> Option<String> {
        let slash_pos_list: Vec<_> = name.match_indices('/').map(|(ind, _)| ind).collect();
        let slash_pos = slash_pos_list[slash_pos_list.len() - 2];
        let parent = (name[0..slash_pos + 1]).to_string();
        if parent.is_empty() {
            return Some(String::from("/"));
        }
        Some(parent)
    }
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 7 });

    let mut current_dir = String::from("");
    let mut file_system = Vec::new();

    for line in content.lines() {
        let elements: Vec<_> = line.split_whitespace().collect();

        if elements[0] == "$" {
            if elements[1] == "cd" {
                if elements[2] == ".." {
                    // get parent from last current
                    let target_parent = file_system
                        .iter()
                        .find_map(|info: &FileInfo| info.get_parent(&current_dir));
                    match target_parent {
                        Some(parent) => current_dir = parent.to_string(),
                        None => println!("Parent not found for current directory {}", current_dir),
                    }
                } else {
                    // last dir is the new dir
                    // new current
                    let dir_name = elements[2].to_string();
                    current_dir.push_str(&dir_name);
                    if !current_dir.ends_with('/') {
                        current_dir.push_str(&'/'.to_string());
                    }
                }
            }
        } else {
            let mut full_path = current_dir.clone();
            full_path.push_str(elements[1]);

            let mut size: i32 = 0;
            if elements[0] != "dir" {
                size = FromStr::from_str(elements[0]).unwrap();
            }
            let file = FileInfo::new(full_path, size);
            file_system.push(file);
        }
    }

    let directories_list: Vec<String> = file_system
        .iter()
        .filter_map(|info| {
            if info.size == 0 {
                Some(info.name.to_string())
            } else {
                None
            }
        })
        .collect();

    let limit: i32 = 100000;
    let mut sum_space_part1: i32 = 0;
    let mut candidates = Vec::new();
    let space_needed: i32 = 30000000;
    let total_space: i32 = 70000000;
    let total_taken_space: i32 = file_system.iter().map(|info| info.size).sum();
    let min_space_needed = space_needed - total_space + total_taken_space;

    for dir in directories_list {
        let sum_dir: i32 = file_system
            .iter()
            .filter_map(|info| {
                if info.name.contains(&dir) {
                    Some(info.size)
                } else {
                    None
                }
            })
            .sum();
        if sum_dir < limit {
            sum_space_part1 += sum_dir;
        }
        if sum_dir > min_space_needed {
            candidates.push(sum_dir);
        }
    }

    candidates.sort_by(|a, b| b.cmp(a));

    println!("Sum part 1 {}", sum_space_part1);
    println!(
        "Min space needed to be deleted {:?}",
        candidates[candidates.len() - 1]
    );
}
