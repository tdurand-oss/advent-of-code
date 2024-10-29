use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    maps: Vec<Vec<Vec<bool>>>,
}

impl Map {
    fn can_move(&self, position: Position, time: u32) -> bool {
        if position == self.get_target() || position == self.get_source() {
            return true;
        }

        if position.0 < 0 || position.0 >= self.height || position.1 < 0 || position.1 >= self.width
        {
            return false;
        }

        !self.maps[time as usize][position.0 as usize][position.1 as usize]
    }

    fn get_target(&self) -> Position {
        (self.height, self.width - 1)
    }

    fn get_source(&self) -> Position {
        (-1, 0)
    }
}

fn preprocess(input: &str) -> Map {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;

    let mut blizzards = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            let blizzard = match ch {
                '>' => Some(Blizzard {
                    position: (row as isize - 1, col as isize - 1),
                    direction: Direction::Right,
                }),
                '<' => Some(Blizzard {
                    position: (row as isize - 1, col as isize - 1),
                    direction: Direction::Left,
                }),
                'v' => Some(Blizzard {
                    position: (row as isize - 1, col as isize - 1),
                    direction: Direction::Down,
                }),
                '^' => Some(Blizzard {
                    position: (row as isize - 1, col as isize - 1),
                    direction: Direction::Up,
                }),
                _ => None,
            };

            if let Some(blizzard) = blizzard {
                blizzards.push(blizzard);
            }
        }
    }

    let mut maps = Vec::new();
    for time in 0..800 {
        let map = compute_map(&blizzards, time, width, height);
        maps.push(map);
    }

    Map {
        width: width as isize,
        height: height as isize,
        maps,
    }
}

fn compute_map(
    blizzards: &Vec<Blizzard>,
    time: i32,
    width: usize,
    height: usize,
) -> Vec<Vec<bool>> {
    let mut map = vec![vec![false; width as usize]; height as usize];
    for blizzard in blizzards {
        let pos = match blizzard.direction {
            Direction::Up => (
                (blizzard.position.0 - time as isize).rem_euclid(height as isize),
                blizzard.position.1,
            ),
            Direction::Right => (
                blizzard.position.0,
                (blizzard.position.1 + time as isize).rem_euclid(width as isize),
            ),
            Direction::Down => (
                (blizzard.position.0 + time as isize).rem_euclid(height as isize),
                blizzard.position.1,
            ),
            Direction::Left => (
                blizzard.position.0,
                (blizzard.position.1 - time as isize).rem_euclid(width as isize),
            ),
        };
        map[pos.0 as usize][pos.1 as usize] = true;
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    time: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(map: &Map, start: Position, end: Position, start_time: u32) -> u32 {
    let mut to_process = BinaryHeap::new();
    to_process.push(State {
        pos: start,
        time: start_time,
    });

    let mut processed = HashSet::new();

    while !to_process.is_empty() {
        let state = to_process.pop().unwrap();

        if processed.contains(&state) {
            continue;
        }
        processed.insert(state);

        if state.pos == end {
            return state.time;
        }

        let mut try_move = |pos| {
            if map.can_move(pos, state.time + 1) {
                to_process.push(State {
                    pos,
                    time: state.time + 1,
                });
            }
        };
        try_move((state.pos.0 + 1, state.pos.1));
        try_move((state.pos.0 - 1, state.pos.1));
        try_move((state.pos.0, state.pos.1 + 1));
        try_move((state.pos.0, state.pos.1 - 1));
        try_move((state.pos.0, state.pos.1));
    }

    0
}

fn part1(input: &str) -> String {
    let map = preprocess(input);

    let time = find_path(&map, map.get_source(), map.get_target(), 0);

    time.to_string()
}

fn part2(input: &str) -> String {
    let map = preprocess(input);

    let time = find_path(&map, map.get_source(), map.get_target(), 0);
    let time = find_path(&map, map.get_target(), map.get_source(), time);
    let time = find_path(&map, map.get_source(), map.get_target(), time);

    time.to_string()
}

crate::run!();

crate::test_example_aoc!(18, 54);

crate::test_aoc!(232, 715);
