// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 3                             │
// ╰─────────────────────────────────────────────────────────╯

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").replace("\n", "");
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    // ── Part 1 ──────────────────────────────────────────────────────────
    let mut sum: u32 = parse_and_multiply(&input);
    println!("Sum of all multiplications: {}", sum);

    // ── Part 2 ──────────────────────────────────────────────────────────
    for (i, unparsed_excluded_part) in input.split("don't()").enumerate() {
        if i == 0 {
            continue;
        }
        let excluded_part = unparsed_excluded_part.split_once("do()");
        if excluded_part.is_none() {
            if i != input.split("dont'()").collect::<Vec<_>>().len() {
                sum -= parse_and_multiply(unparsed_excluded_part);
            }
        } else {
            sum -= parse_and_multiply(excluded_part.expect("Error parsing excluded part").0);
        }
    }
    println!(
        "Sum of all multiplications excluding the excluded parts: {}",
        sum
    );
}

fn parse_and_multiply(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex");
    let mut multiplications: Vec<(u32, u32)> = vec![];
    for (_, [first_value, second_value]) in regex.captures_iter(input).map(|c| c.extract()) {
        multiplications.push((
            first_value.parse::<u32>().expect("Error parsing"),
            second_value.parse::<u32>().expect("Error parsing"),
        ));
    }
    multiplications.iter().map(|(a, b)| a * b).sum::<u32>()
}
