use input_extractor::get::{get_input, AoCDate};

fn ways_to_beat(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut ways_to_beat = 1;
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut speed = 0;
        let mut n_possibility = 0;
        while speed < time {
            speed += 1;
            if (time - speed) * speed > distance {
                n_possibility += 1;
            }
        }
        ways_to_beat *= n_possibility;
    }
    ways_to_beat
}

fn part_one_two(input: &str) -> (u64, u64) {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let times: Vec<u64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let sum_part1 = ways_to_beat(&times, &distances);

    let times: Vec<u64> = vec![lines[0].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()];

    let distances: Vec<u64> = vec![lines[1].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()];
    let sum_part2 = ways_to_beat(&times, &distances);

    return (sum_part1, sum_part2);
}
fn main() {
    let content: String = get_input(AoCDate { year: 2023, day: 6 });
    let (one, two) = part_one_two(&content);
    println!("Sum part 1: {} 2: {}", one, two);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parts() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(part_one_two(input), (288, 71503));
    }
}
