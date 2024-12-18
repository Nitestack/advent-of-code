// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 4                             │
// ╰─────────────────────────────────────────────────────────╯

use glam::{ivec2, IVec2};

fn main() {
    // ── Part 1 ──────────────────────────────────────────────────────────
    let input = include_str!("../input.txt");

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result_xmas = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == 'X' {
                        Some(ivec2(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .map(|x_vec| count_xmas(&map, x_vec))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Number of XMAS: {result_xmas}");

    // ── Part 2 ──────────────────────────────────────────────────────────
    let result_mas = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == 'A' {
                        Some(ivec2(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .filter(|a_vec| count_mas(&map, *a_vec))
                .count()
        })
        .sum::<usize>();

    println!("Number of X-MAS: {result_mas}");
}

fn count_mas(map: &Vec<Vec<char>>, a_vec: IVec2) -> bool {
    let top_left = a_vec + ivec2(-1, -1);
    let top_right = a_vec + ivec2(1, -1);
    let bottom_left = a_vec + ivec2(-1, 1);
    let bottom_right = a_vec + ivec2(1, 1);

    if !(is_in_bounds(map, top_left)
        && is_in_bounds(map, top_right)
        && is_in_bounds(map, bottom_left)
        && is_in_bounds(map, bottom_right))
    {
        return false;
    }

    let top_left_char = get_in_map(map, top_left);
    let top_right_char = get_in_map(map, top_right);
    let bottom_left_char = get_in_map(map, bottom_left);
    let bottom_right_char = get_in_map(map, bottom_right);

    top_left_char == 'M'
        && bottom_right_char == 'S'
        && top_right_char == 'S'
        && bottom_left_char == 'M'
        || top_left_char == 'M'
            && bottom_right_char == 'S'
            && top_right_char == 'M'
            && bottom_left_char == 'S'
        || top_left_char == 'S'
            && bottom_right_char == 'M'
            && top_right_char == 'M'
            && bottom_left_char == 'S'
        || top_left_char == 'S'
            && bottom_right_char == 'M'
            && top_right_char == 'S'
            && bottom_left_char == 'M'
}

fn count_xmas(map: &Vec<Vec<char>>, x_vec: IVec2) -> usize {
    const DIRECTIONS: [IVec2; 8] = [
        ivec2(-1, -1),
        ivec2(0, -1),
        ivec2(1, -1),
        ivec2(-1, 0),
        ivec2(1, 0),
        ivec2(-1, 1),
        ivec2(0, 1),
        ivec2(1, 1),
    ];

    DIRECTIONS
        .iter()
        .filter(|&&direction| {
            let s = x_vec + 3 * direction;
            is_in_bounds(map, s)
                && get_in_map(map, x_vec + direction) == 'M'
                && get_in_map(map, x_vec + 2 * direction) == 'A'
                && get_in_map(map, s) == 'S'
        })
        .count()
}

fn get_in_map(map: &Vec<Vec<char>>, pos: IVec2) -> char {
    map[pos.y as usize][pos.x as usize]
}

fn is_in_bounds(map: &Vec<Vec<char>>, pos: IVec2) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < map[0].len() as i32 && pos.y < map.len() as i32
}
