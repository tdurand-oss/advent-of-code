type Actions = Vec<(usize, usize, usize)>;
type Stacks = Vec<Vec<char>>;

fn parse_stacks(lines: &[&str]) -> Stacks {
    let mut stacks = Vec::new();
    for line in lines {
        if line.starts_with(" 1") {
            break;
        }

        let mut index = 0;
        while let Some(letter) = line.chars().nth(index * 4 + 1) {
            if letter != ' ' {
                if index >= stacks.len() {
                    stacks.resize(index + 1, Vec::new());
                }
                stacks[index].insert(0, letter);
            }
            index += 1;
        }
    }

    stacks
}

fn parse_actions(lines: &[&str]) -> Actions {
    let mut moves = Vec::new();
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        if !line.starts_with("move") {
            continue;
        }

        let captures = re.captures(line).unwrap();
        let count: usize = captures[1].parse().unwrap();
        let from: usize = captures[2].parse::<usize>().unwrap() - 1;
        let to: usize = captures[3].parse::<usize>().unwrap() - 1;

        moves.push((count, from, to))
    }

    moves
}

fn preprocess(input: &str) -> (Stacks, Actions) {
    let lines = input.lines().collect::<Vec<_>>();

    let stacks = parse_stacks(&lines);
    let actions = parse_actions(&lines);

    (stacks, actions)
}

fn part1(input: &str) -> String {
    let (mut stacks, actions) = preprocess(input);

    for action in actions {
        for _ in 0..action.0 {
            match stacks[action.1].pop() {
                Some(element) => stacks[action.2].push(element),
                None => {
                    panic!("{:?}", action);
                }
            }
        }
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let (mut stacks, actions) = preprocess(input);

    for action in actions {
        let split_index = stacks[action.1].len() - action.0;
        let moved = stacks[action.1].split_off(split_index);
        stacks[action.2].extend_from_slice(&moved);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

crate::run!();

crate::test_example_aoc!("CMZ", "MCD");

crate::test_aoc!("VJSFHWGFT", "LCTQFBVZV");
