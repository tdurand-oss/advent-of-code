use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type Position = (isize, isize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn parse(input: &str) -> (Self, Position, Position) {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            start = (x as isize, y as isize);
                            0
                        }
                        'E' => {
                            end = (x as isize, y as isize);
                            b'z' - b'a'
                        }
                        ch => ch as u8 - b'a',
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (Self { map }, start, end)
    }

    fn get(&self, position: Position) -> Option<u8> {
        if position.0 < 0
            || position.0 >= self.map.len() as isize
            || position.1 < 0
            || position.1 >= self.map[position.0 as usize].len() as isize
        {
            None
        } else {
            Some(self.map[position.0 as usize][position.1 as usize])
        }
    }
}

fn find_steps<S, V>(start_position: Position, stop_condition: S, visit_condition: V) -> usize
where
    S: Fn(Position) -> bool,
    V: Fn(Position, Position) -> bool,
{
    let mut cost = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(State {
        cost: 0,
        position: start_position,
    });

    while let Some(current) = to_visit.pop() {
        if stop_condition(current.position) {
            return current.cost;
        }

        if cost.contains_key(&current.position) {
            continue;
        }

        cost.insert(current.position, current.cost);

        let mut visit_neighbor = |neighbor| {
            if visit_condition(current.position, neighbor) {
                to_visit.push(State {
                    cost: current.cost + 1,
                    position: neighbor,
                })
            }
        };

        visit_neighbor((current.position.0 - 1, current.position.1));
        visit_neighbor((current.position.0 + 1, current.position.1));
        visit_neighbor((current.position.0, current.position.1 - 1));
        visit_neighbor((current.position.0, current.position.1 + 1));
    }

    panic!("Path not found");
}

pub fn part1(input: &str) -> String {
    // Input
    let (map, start, end) = Map::parse(&input);

    // Star 1
    let stop_condition_1 = |position| position == end;
    let visit_condition_1 = |current, neighbor| {
        let current_elevation = map.get(current).unwrap();
        if let Some(neighbor_elevation) = map.get(neighbor) {
            neighbor_elevation <= current_elevation + 1
        } else {
            false
        }
    };

    let steps1 = find_steps(start, stop_condition_1, visit_condition_1);
    steps1.to_string()
}


pub fn part2(input: &str) -> String {
    // Input
    let (map, _, end) = Map::parse(&input);

    // Star 2
    let stop_condition_2 = |position| map.get(position).unwrap() == 0;
    let visit_condition_2 = |current, neighbor| {
        let current_elevation = map.get(current).unwrap();
        if let Some(neighbor_elevation) = map.get(neighbor) {
            neighbor_elevation >= current_elevation - 1
        } else {
            false
        }
    };

    let steps2 = find_steps(end, stop_condition_2, visit_condition_2);
    steps2.to_string()
}

crate::run!();

crate::test_example_aoc!(31, 29);

crate::test_aoc!(420, 414);
