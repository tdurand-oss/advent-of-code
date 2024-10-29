use itertools::Itertools;

fn preprocess(input: &str) -> Vec<(usize, i64)> {
    let numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .into_iter()
        .enumerate()
        .collect();
    numbers
}

fn decrypt(numbers: Vec<(usize, i64)>, key: i64, rounds: usize) -> i64 {
    let mut numbers = numbers.iter().map(|n| (n.0, n.1 * key)).collect::<Vec<_>>();

    for _ in 0..rounds {
        for original_index in 0..numbers.len() {
            let current_index = numbers
                .iter()
                .find_position(|n| n.0 == original_index)
                .unwrap()
                .0;

            let new_index = (current_index as i64 + numbers[current_index].1)
                .rem_euclid(numbers.len() as i64 - 1) as usize;

            if new_index > current_index {
                numbers.insert(new_index + 1, numbers[current_index]);
                numbers.remove(current_index);
            } else {
                numbers.insert(new_index, numbers[current_index]);
                numbers.remove(current_index + 1);
            }
        }
    }

    let index0 = numbers.iter().find_position(|n| n.1 == 0).unwrap().0;
    let n1000 = numbers[(index0 + 1000) % numbers.len()];
    let n2000 = numbers[(index0 + 2000) % numbers.len()];
    let n3000 = numbers[(index0 + 3000) % numbers.len()];
    n1000.1 + n2000.1 + n3000.1
}

fn part1(input: &str) -> String {
    let numbers = preprocess(input);
    decrypt(numbers, 1, 1).to_string()
}

fn part2(input: &str) -> String {
    let numbers = preprocess(input);
    decrypt(numbers, 811589153, 10).to_string()
}

crate::run!();

crate::test_example_aoc!(3, 1623178306);

crate::test_aoc!(15297, 2897373276210usize);
