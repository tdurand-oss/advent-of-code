use std::collections::{HashSet, HashMap};

fn parse_input(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let values = line.split(':').nth(1).unwrap();
        let winning_numbers = values.split('|').next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let numbers = values.split('|').nth(1).unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>();
        cards.push((winning_numbers, numbers));
    }
    cards
}

fn part1(input: &str) -> String {
    let games = parse_input(input);
    let sum = games.iter()
        .map(|(winning, number)| {
            let count  = number.intersection(winning).count();
            if count > 0 { 
                2_u32.pow(count as u32 - 1)
            }
            else {
                0
            }
        } )
        .sum::<u32>();
    sum.to_string()
}

fn part2(input: &str) -> String {
    let games = parse_input(input);
    let mut cards: HashMap<usize, u32> = HashMap::new();
    for (game_id, (winning, numbers)) in games.iter().enumerate() {
        *cards.entry(game_id + 1).or_default() += 1;
        let count  = numbers.intersection(winning).count();
        for card in 0..count {
            *cards.entry(game_id + 1 + card + 1).or_default() += *cards.entry(game_id + 1).or_default();
        }
    }

    let sum = cards.iter().map(|(_, count)| count).sum::<u32>();
    sum.to_string()
}

crate::run!();

crate::test_example_aoc!(13, 30);

crate::test_aoc!(27059, 5744979);