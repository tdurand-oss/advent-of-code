use std::collections::HashSet;

use itertools::Itertools;

fn preprocess(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    let droplets = preprocess(input);

    let touching_sides = droplets
        .iter()
        .cartesian_product(&droplets)
        .map(|(droplet1, droplet2)| {
            let distance = (droplet2.0 - droplet1.0).abs()
                + (droplet2.1 - droplet1.1).abs()
                + (droplet2.2 - droplet1.2).abs();

            u32::from(distance == 1)
        })
        .sum::<u32>();

    let surface = 6 * droplets.len() as u32 - touching_sides;

    surface.to_string()
}

fn part2(input: &str) -> String {
    let droplets = preprocess(input);

    let all = droplets.iter().collect::<HashSet<_>>();

    let n = 20;
    let mut space = vec![vec![vec![0; n]; n]; n];

    let mut to_process = vec![(0, 0, 0)];
    let mut total = 0;

    while !to_process.is_empty() {
        let current = to_process.pop().unwrap();
        if space[current.0 as usize][current.1 as usize][current.2 as usize] == 1 {
            continue;
        }

        space[current.0 as usize][current.1 as usize][current.2 as usize] = 1;

        if all.contains(&(current.0, current.1, current.2)) {
            continue;
        }

        let mut visit = |pos: (i32, i32, i32)| {
            if pos.0 < 0
                || pos.0 >= n as i32
                || pos.1 < 0
                || pos.1 >= n as i32
                || pos.2 < 0
                || pos.2 >= n as i32
            {
                return;
            }
            if all.contains(&(pos.0 as i32, pos.1 as i32, pos.2 as i32)) {
                total += 1;
            } else {
                to_process.push(pos);
            }
        };

        visit((current.0 - 1, current.1, current.2));
        visit((current.0 + 1, current.1, current.2));
        visit((current.0, current.1 - 1, current.2));
        visit((current.0, current.1 + 1, current.2));
        visit((current.0, current.1, current.2 - 1));
        visit((current.0, current.1, current.2 + 1));
    }

    // Add droplets on the side of the space
    let sides = droplets
        .iter()
        .filter(|d| {
            d.0 == 0
                || d.1 == 0
                || d.2 == 0
                || d.0 == n as i32 - 1
                || d.1 == n as i32 - 1
                || d.2 == n as i32 - 1
        })
        .count();

    total += sides;

    total.to_string()
}

crate::run!();

crate::test_example_aoc!(64, 58);

crate::test_aoc!(3466, 2012);
