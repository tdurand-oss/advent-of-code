fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    let numbers = parse_input(input);
    let mut higher = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] {
            higher += 1;
        }
    }
    higher.to_string()
}

fn part2(input: &str) -> String {
    let numbers = parse_input(input);
    let mut higher = 0;
    for i in 3..numbers.len() {
        if numbers[i] > numbers[i - 3] {
            higher += 1;
        }
    }
    higher.to_string()
}

crate::run!();

crate::test_example_aoc!(7, 5);

crate::test_aoc!(1759, 1805);