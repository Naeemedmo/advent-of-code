use input_extractor::get::{get_input, AoCDate};

fn process_mapping(
    lines: &Vec<String>,
    index: usize,
    source_vector: &Vec<i64>,
) -> (usize, Vec<i64>) {
    let mut destination_vector: Vec<i64> = vec![];
    let mut line_index = index;
    destination_vector.resize(source_vector.len().try_into().unwrap(), 0);
    while line_index < lines.len() && !lines[line_index].is_empty() {
        // destination - source - range
        let info: Vec<i64> = lines[line_index]
            .split_whitespace()
            .map(|x| x.parse::<i64>())
            .filter_map(Result::ok)
            .collect();

        for (ind, source) in source_vector.iter().enumerate() {
            let mut destination: i64 = 0;
            if source >= &info[1] && source < &(info[1] + info[2]) {
                let map_index = source - info[1];
                destination = info[0] + map_index;
            }
            if destination_vector[ind] == 0 {
                destination_vector[ind] = destination;
            }
        }
        line_index += 1; // go next
    }
    (line_index, destination_vector)
}

fn replace_zeros_in_destination(
    source_vector: &Vec<i64>,
    destination_vector: &Vec<i64>,
) -> Vec<i64> {
    let mut new_destination: Vec<i64> = destination_vector.clone();
    for (i, d) in destination_vector.iter().enumerate() {
        if d == &0 {
            new_destination[i] = source_vector[i];
        }
    }
    new_destination
}

fn part_one_two(input: &str) -> (i64, i64) {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let input_length = lines.len();
    let mut i: usize = 0;
    let mut source: Vec<i64> = vec![];
    let mut destination: Vec<i64> = vec![];
    let map_names = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    while i < input_length {
        if lines[i].contains("seeds:") {
            let parts = lines[i].split(":").collect::<Vec<&str>>();
            source = parts[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>())
                .filter_map(Result::ok)
                .collect();
        }
        for name in map_names {
            if lines[i].contains(name) {
                i += 1; // go next
                (i, destination) = process_mapping(&lines, i, &source);
                destination = replace_zeros_in_destination(&source, &destination);
                source = destination.clone();
            }
        }
        i += 1
    }

    let min_location = *destination.iter().min().unwrap_or(&0);
    return (min_location, 0);
}
fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 5 });
    let (one, two) = part_one_two(&content);
    println!("Part 1: {} 2: {}", one, two);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parts() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_one_two(input), (35, 0));
    }
}
