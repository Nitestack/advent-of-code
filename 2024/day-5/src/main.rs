// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 5                             │
// ╰─────────────────────────────────────────────────────────╯

use std::cmp::Ordering;

fn main() {
    let (unparsed_rules, unparsed_updates) = include_str!("../input.txt").split_once("\n\n").expect("Invalid input");
    let rules = unparsed_rules.lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    let updates = unparsed_updates.lines()
        .map(|line| line.split(",").map(|page| page.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // ── Part 1 ──────────────────────────────────────────────────────────
    let result_part1 = updates.iter()
        .filter(|update| rules.iter().all(|rule| complies_with_rule(rule, update)))
        .map(|update| update[update.len() / 2])
        .sum::<u32>();

    println!("Result: {}", result_part1);
    // ── Part 2 ──────────────────────────────────────────────────────────
    let result_part2 = updates.iter()
        .filter(|update| !rules.iter().all(|rule| complies_with_rule(rule, update)))
        .map(|update| {
            let mut sorted_update = update.clone();
            sorted_update.sort_unstable_by(|a, b| {
                match rules.iter().find(|&(x, y)| (x == a || x == b) && (y == a || y == b)) {
                    Some((x, y)) => {
                        if a == x && b == y { Ordering::Less }
                        else { Ordering::Greater }
                    },
                    None => Ordering::Equal
                }
            });
            sorted_update
        })
        .map(|update| update[update.len() / 2])
        .sum::<u32>();
    println!("Result: {}", result_part2);
}

fn complies_with_rule(rule: &(u32, u32), update: &Vec<u32>) -> bool {
    let (a, b) = rule;

    !update.contains(&a)
        || !update.contains(&b)
        || update.iter().position(|page| page == a).unwrap() < update.iter().position(|page| page == b).unwrap()
}
