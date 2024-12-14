// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 2                             │
// ╰─────────────────────────────────────────────────────────╯
fn main() {
    let input = include_str!("../input.txt");

    // ── Part 1 ──────────────────────────────────────────────────────────

    let mut safe_count: u32 = 0;

    input.lines().for_each(|line| {
        let levels = line.split(" ").map(|level| level.parse::<u32>()).collect::<Result<Vec<_>, _>>().expect("Invalid input");

        if is_safe(&levels) { safe_count += 1; }
    });
    println!("Safe count: {}", safe_count);

    // ── Part 2 ──────────────────────────────────────────────────────────
    let mut dampener_safe_count: u32 = 0;
    input.lines().for_each(|line| {
        let levels = line.split(" ").map(|level| level.parse::<u32>()).collect::<Result<Vec<_>, _>>().expect("Invalid input");

        if is_safe(&levels) { dampener_safe_count += 1; }
        else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe(&new_levels) {
                    dampener_safe_count += 1;
                    break;
                }
            }
        }
    });
    println!("Safe count with Dampener: {}", dampener_safe_count);
}

fn is_safe(levels: &Vec<u32>) -> bool {
    let mut has_upwards_pattern = false;
    let mut has_downwards_pattern = false;

    let mut is_safe = false;

    for (i, level) in levels.iter().enumerate() {
        if i == 0 { continue; }
        if !(1..=3).contains(&level.abs_diff(levels[i - 1])) { break; }
        if *level > levels[i - 1] {
            if has_downwards_pattern { break; }
            has_upwards_pattern = true;
        } else if *level < levels[i - 1] {
            if has_upwards_pattern { break; }
            has_downwards_pattern = true;
        }
        if levels.len() - 1 == i {
            is_safe = true;
        }
    }
    is_safe
}
