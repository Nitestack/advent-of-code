// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 8                             │
// ╰─────────────────────────────────────────────────────────╯

use std::collections::HashSet;

use glam::ivec2;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let frequencies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, ivec2(x as i32, y as i32)))
                .collect::<Vec<_>>()
        })
        .into_group_map();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    // ── Part 1 ──────────────────────────────────────────────────────────
    let anti_nodes_count = frequencies
        .values()
        .flat_map(|antennas| {
            antennas.iter().flat_map(|first_pos| {
                antennas
                    .iter()
                    .filter(|&second_pos| first_pos != second_pos)
                    .filter_map(|second_pos| {
                        let diff = second_pos - first_pos;
                        let anti_node = second_pos + diff;
                        if anti_node.x >= 0
                            && anti_node.y >= 0
                            && anti_node.x < width
                            && anti_node.y < height
                        {
                            Some(anti_node)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<HashSet<_>>()
        .len();

    println!("Antinodes count: {anti_nodes_count}");

    // ── Part 2 ──────────────────────────────────────────────────────────
    let anti_nodes_count = frequencies
        .values()
        .flat_map(|antennas| {
            antennas.iter().flat_map(|first_pos| {
                antennas
                    .iter()
                    .filter(|&second_pos| first_pos != second_pos)
                    .flat_map(|second_pos| {
                        let diff = second_pos - first_pos;
                        let mut anti_nodes = vec![];
                        for i in 1.. {
                            let anti_node = first_pos + diff * i;
                            if anti_node.x >= 0
                                && anti_node.y >= 0
                                && anti_node.x < width
                                && anti_node.y < height
                            {
                                anti_nodes.push(anti_node)
                            } else {
                                break;
                            }
                        }
                        anti_nodes
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<HashSet<_>>()
        .len();

    println!("Antinodes count: {anti_nodes_count}");
}
