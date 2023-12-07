use input_extractor::get::{get_input, AoCDate};
use num_traits::{FromPrimitive, PrimInt};
use std::ops::{AddAssign, MulAssign};


fn ways_to_beat<T>(times: &[T], distances: &[T]) -> T
where
    T: Copy + Clone + PrimInt + FromPrimitive + MulAssign +  AddAssign,
{
    let mut n_ways_to_beat = T::from(1).unwrap(); // Initialize with 1 for multiplication

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut speed = T::from(0).unwrap();
        let mut n_possibility = T::from(0).unwrap();

        while speed < time {
            speed += T::from(1).unwrap();

            if (time - speed) * speed > distance {
                n_possibility += T::from(1).unwrap();
            }
        }

        n_ways_to_beat *= n_possibility;
    }

    n_ways_to_beat
}


fn part_one_two(input: &str) -> (u32, u64) {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let times: Vec<u32> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
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
