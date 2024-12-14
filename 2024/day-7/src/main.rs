// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 7                             │
// ╰─────────────────────────────────────────────────────────╯

fn main() {
    let input = include_str!("../input.txt");

    // ── Part 1 ──────────────────────────────────────────────────────────
    let result_part1 = input.lines().map(|line| {
        let (unparsed_result, unparsed_numbers) = line.split_once(": ").expect("Invalid input");
        let result = unparsed_result.parse::<usize>().unwrap();
        let numbers = unparsed_numbers.split(" ")
            .map(|unparsed_number| unparsed_number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if try_numbers(&numbers[1..], result, numbers[0]) { result } else { 0 }
    }).sum::<usize>();
    println!("Result: {result_part1}");
    // ── Part 2 ──────────────────────────────────────────────────────────
    let result_part2 = input.lines().map(|line| {
        let (unparsed_result, unparsed_numbers) = line.split_once(": ").expect("Invalid input");
        let result = unparsed_result.parse::<usize>().unwrap();
        let numbers = unparsed_numbers.split(" ")
            .map(|unparsed_number| unparsed_number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if try_numbers2(&numbers[1..], result, numbers[0]) { result } else { 0 }
    }).sum::<usize>();
    println!("Result: {result_part2}");
}

fn try_numbers(numbers: &[usize], result: usize, current: usize) -> bool {
    if numbers.is_empty() {
        result == current
    } else if current > result {
        false
    } else {
        try_numbers(&numbers[1..], result, current + numbers[0]) || try_numbers(&numbers[1..], result, current * numbers[0])
    }
}

fn try_numbers2(numbers: &[usize], result: usize, current: usize) -> bool {
    if numbers.is_empty() {
        result == current
    } else if current > result {
        false
    } else {
        try_numbers2(&numbers[1..], result, current + numbers[0]) || try_numbers2(&numbers[1..], result, current * numbers[0]) || try_numbers2(&numbers[1..], result, current * 10usize.pow(numbers[0].ilog10() + 1) + numbers[0])
    }
}
