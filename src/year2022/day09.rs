use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_touching(&self, other: &Position) -> bool {
        self.x >= other.x - 1
            && self.x <= other.x + 1
            && self.y >= other.y - 1
            && self.y <= other.y + 1
    }
}

fn simulate_rope(actions: &[(char, u32)], rope_length: usize) -> usize {
    let mut visited = HashSet::new();
    let mut rope: Vec<Position> = vec![Default::default(); rope_length];
    visited.insert(rope[0]);
    for (direction, amount) in actions {
        for _ in 0..*amount {
            // Move head
            match direction {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'R' => rope[0].x += 1,
                'L' => rope[0].x -= 1,
                _ => panic!("Unknown direction"),
            }

            // Move rest of the rope
            for index in 1..rope.len() {
                if !rope[index].is_touching(&rope[index - 1]) {
                    let difference = (
                        rope[index - 1].x - rope[index].x,
                        rope[index - 1].y - rope[index].y,
                    );
                    match difference {
                        (0, dy) => rope[index].y += dy.signum(),
                        (dx, 0) => rope[index].x += dx.signum(),
                        (dx, dy) => {
                            rope[index].x += dx.signum();
                            rope[index].y += dy.signum();
                        }
                    }
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn preprocess(input: &str) -> Vec<(char, u32)> {
    input
        .lines()
        .map(|line| line.split(' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(direction, amount)| {
            (
                direction.chars().next().unwrap(),
                amount.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    let actions = preprocess(input);
    let visited = simulate_rope(&actions, 2);
    visited.to_string()
}

fn part2(input: &str) -> String {
    let actions = preprocess(input);
    let visited = simulate_rope(&actions, 10);
    visited.to_string()
}

crate::run!();
crate::test_example_aoc!(13, 1);

crate::test_aoc!(6332, 2511);
