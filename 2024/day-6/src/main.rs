// ╭─────────────────────────────────────────────────────────╮
// │ Advent of Code 2024 - Day 6                             │
// ╰─────────────────────────────────────────────────────────╯

use std::collections::HashSet;

use glam::{ ivec2, IVec2 };

const LEFT: IVec2 = ivec2(-1, 0);
const RIGHT: IVec2 = ivec2(1, 0);
const UP: IVec2 = ivec2(0, -1);
const DOWN: IVec2 = ivec2(0, 1);

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Obstruction
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}
impl Direction {
    fn vec(&self) -> IVec2 {
        match self {
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
            Direction::Up => UP,
            Direction::Down => DOWN
        }
    }
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

struct Guard {
    pos: IVec2,
    initial_pos: IVec2,
    direction: Direction,
}
impl Guard {
    fn new(pos: IVec2) -> Self {
        Guard { pos, initial_pos: pos, direction: Direction::Up }
    }
    fn reset(&mut self) -> &Self {
        self.pos = self.initial_pos;
        self.direction = Direction::Up;
        self
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut guard = Guard::new(ivec2(0, 0));

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(|(x, tile)| match tile {
            '^' => {
                guard = Guard::new(ivec2(x as i32, y as i32));
                Tile::Empty
            },
            '#' => Tile::Obstruction,
            '.' => Tile::Empty,
            _ => panic!("Invalid input")
        }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut positions = HashSet::from([guard.initial_pos]);
    // ── Part 1 ──────────────────────────────────────────────────────────
    loop {
        let next_pos = guard.pos + guard.direction.vec();
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= map[0].len() as i32 || next_pos.y >= map.len() as i32 { break; }
        if map[next_pos.y as usize][next_pos.x as usize] == Tile::Obstruction {
            guard.direction = guard.direction.rotate_right();
            continue;
        }
        guard.pos = next_pos;
        positions.insert(next_pos);
    }
    println!("Number of positions visited: {}", positions.len());
    // ── Part 2 ──────────────────────────────────────────────────────────
    let mut obstruction_positions: usize = 0;
    for pos in positions {
        guard.reset();
        let mut new_map = map.clone();
        new_map[pos.y as usize][pos.x as usize] = Tile::Obstruction;

        let mut new_positions = HashSet::new();
        loop {
            if !new_positions.insert((guard.pos, guard.direction)) {
                obstruction_positions += 1;
                break;
            }
            let next_pos = guard.pos + guard.direction.vec();
            if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= map[0].len() as i32 || next_pos.y >= map.len() as i32 { break; }
            if new_map[next_pos.y as usize][next_pos.x as usize] == Tile::Obstruction {
                guard.direction = guard.direction.rotate_right();
                continue;
            }
            guard.pos = next_pos;
        }
    }
    println!("Different obstruction positions: {obstruction_positions}");
}
