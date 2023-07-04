use input_extractor::get::{get_input, AoCDate};
use itertools::Itertools;
use microbench::{self, Options};
use regex::Regex;

fn benchmark_base(content: &str, re: &Regex) -> (u32, u32) {
    let mut number_matches: u32 = 0;
    let mut number_overlap: u32 = 0;
    for line in content.lines() {
        let Some((a, b, c, d)) = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect_tuple() else { panic!() };

        if ((a..b + 1).contains(&c) && (a..b + 1).contains(&d))
            || ((c..d + 1).contains(&a) && (c..d + 1).contains(&b))
        {
            number_matches += 1
        }
        if (a..b + 1).contains(&c)
            || (a..b + 1).contains(&d)
            || (c..d + 1).contains(&a)
            || (c..d + 1).contains(&b)
        {
            number_overlap += 1
        }
    }
    return (number_matches, number_overlap);
}

fn benchmark_one(content: &str, re: &Regex) -> (u32, u32) {
    let mut number_matches: u32 = 0;
    let mut number_overlap: u32 = 0;
    for line in content.lines() {
        if let Some((a, b, c, d)) = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect_tuple()
        {
            let overlap = (b.min(d) - a.max(c));
            number_matches += if overlap == (b - a) || overlap == (d - c) {
                1
            } else {
                0
            };
            number_overlap += if overlap >= 0 { 1 } else { 0 };
        } else {
            panic!();
        }
    }
    return (number_matches, number_overlap);
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 4 });

    // Print the result
    let re = Regex::new(r"\b\d+\b|\b\d+-\d+\b").unwrap();

    let (number_matches, number_overlap) = benchmark_base(&content, &re);
    println!("Number of matches {}", number_matches);
    println!("Number of overlaps {}", number_overlap);
    let (number_matches, number_overlap) = benchmark_one(&content, &re);
    println!("Number of matches {}", number_matches);
    println!("Number of overlaps {}", number_overlap);

    let options = Options::default();
    microbench::bench(&options, "experiment_base", || {
        benchmark_base(&content, &re)
    });
    microbench::bench(&options, "experiment_one", || benchmark_one(&content, &re));
}
