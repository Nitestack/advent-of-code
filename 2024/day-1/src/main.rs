// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 1                             │
// ╰─────────────────────────────────────────────────────────╯
fn main() {
    let input = include_str!("../input.txt");

    let mut left_numbers: Vec<usize> = vec![];
    let mut right_numbers: Vec<usize> = vec![];

    // ── Part 1 ──────────────────────────────────────────────────────────

    input.lines().for_each(|line| {
        let (left_part, right_part) = line.split_once(" ").expect("An unexpected error occurred");
        left_numbers.push(left_part.trim().parse::<usize>().expect("An unexpected error occurred while coercing a string to a number"));
        right_numbers.push(right_part.trim().parse::<usize>().expect("An unexpected error occurred while coercing a string to a number"));
    });

    left_numbers.sort();
    right_numbers.sort();

    let mut differences: usize = 0;

    for(&left, &right) in left_numbers.iter().zip(right_numbers.iter()) {
        differences += if left > right { left - right } else { right - left };
    }

    println!("Sum of differences: {}", differences);

    // ── Part 2 ──────────────────────────────────────────────────────────

    let mut similarity_score: usize = 0;

    left_numbers.dedup();

    for &left in left_numbers.iter() {
        similarity_score += right_numbers.iter().filter(|&&right| left == right).count() * &left;
    }

    println!("Similarity score: {}", similarity_score);
}
