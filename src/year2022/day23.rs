use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

type Position = (isize, isize);

fn preprocess(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| (i as isize, j as isize, ch))
        })
        .filter_map(|(i, j, ch)| if ch == '#' { Some((i, j)) } else { None })
        .collect::<HashSet<_>>()
}

fn neighbor(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::NorthWest => (pos.0 - 1, pos.1 - 1),
        Direction::West => (pos.0, pos.1 - 1),
        Direction::SouthWest => (pos.0 + 1, pos.1 - 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::SouthEast => (pos.0 + 1, pos.1 + 1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::NorthEast => (pos.0 - 1, pos.1 + 1),
    }
}

fn has_neighbor(elves: &HashSet<Position>, elf: Position) -> bool {
    elves.contains(&neighbor(elf, Direction::North))
        || elves.contains(&neighbor(elf, Direction::NorthWest))
        || elves.contains(&neighbor(elf, Direction::West))
        || elves.contains(&neighbor(elf, Direction::SouthWest))
        || elves.contains(&neighbor(elf, Direction::South))
        || elves.contains(&neighbor(elf, Direction::SouthEast))
        || elves.contains(&neighbor(elf, Direction::East))
        || elves.contains(&neighbor(elf, Direction::NorthEast))
}

fn can_move(elves: &HashSet<Position>, elf: Position, dir: Direction) -> bool {
    match dir {
        Direction::North => {
            !elves.contains(&neighbor(elf, Direction::North))
                && !elves.contains(&neighbor(elf, Direction::NorthWest))
                && !elves.contains(&neighbor(elf, Direction::NorthEast))
        }
        Direction::West => {
            !elves.contains(&neighbor(elf, Direction::West))
                && !elves.contains(&neighbor(elf, Direction::SouthWest))
                && !elves.contains(&neighbor(elf, Direction::NorthWest))
        }
        Direction::South => {
            !elves.contains(&neighbor(elf, Direction::South))
                && !elves.contains(&neighbor(elf, Direction::SouthWest))
                && !elves.contains(&neighbor(elf, Direction::SouthEast))
        }
        Direction::East => {
            !elves.contains(&neighbor(elf, Direction::East))
                && !elves.contains(&neighbor(elf, Direction::SouthEast))
                && !elves.contains(&neighbor(elf, Direction::NorthEast))
        }
        _ => unreachable!(),
    }
}

fn simulate_round(elves: &HashSet<Position>, round: usize) -> HashSet<Position> {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut proposals: HashMap<Position, HashSet<Position>> = HashMap::new();

    for elf in elves {
        if !has_neighbor(elves, *elf) {
            proposals.entry(*elf).or_default().insert(*elf);
            continue;
        }

        let mut could_move = false;
        for k in 0..DIRECTIONS.len() {
            let dir = DIRECTIONS[(round + k) % DIRECTIONS.len()];
            if can_move(elves, *elf, dir) {
                could_move = true;
                proposals
                    .entry(neighbor(*elf, dir))
                    .or_default()
                    .insert(*elf);
                break;
            }
        }

        if !could_move {
            proposals.entry(*elf).or_default().insert(*elf);
        }
    }

    let mut new_elves = HashSet::new();
    for (next, original) in &proposals {
        if original.len() == 1 {
            new_elves.insert(*next);
        } else {
            new_elves.extend(original);
        }
    }

    new_elves
}

fn part1(input: &str) -> String {
    let mut elves = preprocess(input);

    for round in 0..10 {
        elves = simulate_round(&elves, round);
    }

    let min_i = elves.iter().map(|e| e.0).min().unwrap();
    let max_i = elves.iter().map(|e| e.0).max().unwrap();
    let min_j = elves.iter().map(|e| e.1).min().unwrap();
    let max_j = elves.iter().map(|e| e.1).max().unwrap();
    let empty = (max_i - min_i + 1) * (max_j - min_j + 1) - elves.len() as isize;
    empty.to_string()
}

fn part2(input: &str) -> String {
    let mut elves = preprocess(input);

    let mut round = 0;
    loop {
        let new_elves = simulate_round(&elves, round);
        if new_elves == elves {
            break;
        }
        round += 1;
        elves = new_elves;
    }

    let round = round + 1;
    round.to_string()
}

crate::run!();

crate::test_example_aoc!(110, 20);

crate::test_aoc!(4068, 968);
