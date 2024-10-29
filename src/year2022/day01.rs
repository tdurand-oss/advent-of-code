fn preprocess(input: &str) -> Vec<u32> {
    let mut current = 0;
    let mut elves = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    elves.push(current);
    elves.sort();
    elves
}

fn part1(input: &str) -> String {
    let elves = preprocess(input);
    (*elves.last().unwrap()).to_string()
}

fn part2(input: &str) -> String {
    let elves = preprocess(input);
    elves[elves.len() - 3..].iter().sum::<u32>().to_string()
}

crate::run!();

crate::test_example_aoc!(24000, 45000);

crate::test_aoc!(71300, 209691);
