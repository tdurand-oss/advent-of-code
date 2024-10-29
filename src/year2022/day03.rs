use std::collections::HashSet;

fn letter_score(letter: char) -> u32 {
    match letter {
        'a'..='z' => 1 + letter as u32 - ('a' as u32),
        'A'..='Z' => 27 + letter as u32 - ('A' as u32),
        _ => 0,
    }
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let size = line.len() / 2;
            let bag1 = line[..size].chars().collect::<HashSet<char>>();
            let bag2 = line[size..].chars().collect::<HashSet<char>>();
            bag1.intersection(&bag2).cloned().next().unwrap()
        })
        .map(letter_score)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let elf1 = lines[0].chars().collect::<HashSet<char>>();
            let elf2 = lines[1].chars().collect::<HashSet<char>>();
            let elf3 = lines[2].chars().collect::<HashSet<char>>();
            let common = elf1.intersection(&elf2).cloned().collect::<HashSet<_>>();
            let common = common.intersection(&elf3);
            common.cloned().next().unwrap()
        })
        .map(letter_score)
        .sum::<u32>()
        .to_string()
}

crate::run!();

crate::test_example_aoc!(157, 70);

crate::test_aoc!(8105, 2363);
