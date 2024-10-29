use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Job {
    Num(f64),
    Add(String, String),
    Sub(String, String),
    Div(String, String),
    Mul(String, String),
}

fn preprocess(input: &str) -> HashMap<String, Job> {
    input
        .lines()
        .map(|line| {
            let name = line[..4].to_owned();
            let job = if line.chars().nth(6).unwrap().is_ascii_digit() {
                Job::Num(line[6..].parse::<f64>().unwrap())
            } else {
                let op1 = line[6..10].to_owned();
                let op2 = line[13..17].to_owned();
                match line.chars().nth(11).unwrap() {
                    '+' => Job::Add(op1, op2),
                    '-' => Job::Sub(op1, op2),
                    '*' => Job::Mul(op1, op2),
                    '/' => Job::Div(op1, op2),
                    _ => panic!("Invalid operation"),
                }
            };
            (name, job)
        })
        .collect()
}

fn get_value(monkeys: &HashMap<String, Job>, monkey: &str) -> f64 {
    match &monkeys[monkey] {
        Job::Num(n) => *n,
        Job::Add(op1, op2) => get_value(monkeys, op1) + get_value(monkeys, op2),
        Job::Sub(op1, op2) => get_value(monkeys, op1) - get_value(monkeys, op2),
        Job::Div(op1, op2) => get_value(monkeys, op1) / get_value(monkeys, op2),
        Job::Mul(op1, op2) => get_value(monkeys, op1) * get_value(monkeys, op2),
    }
}

fn part1(input: &str) -> String {
    let monkeys = preprocess(input);
    let root = get_value(&monkeys, "root");
    root.to_string()
}

fn part2(input: &str) -> String {
    let mut monkeys = preprocess(input);
    let (root_left, root_right) = match &monkeys["root"] {
        Job::Num(_) => unreachable!(),
        Job::Add(op1, op2) => (op1.clone(), op2.clone()),
        Job::Sub(op1, op2) => (op1.clone(), op2.clone()),
        Job::Div(op1, op2) => (op1.clone(), op2.clone()),
        Job::Mul(op1, op2) => (op1.clone(), op2.clone()),
    };

    // Binary search
    let mut result = 0.0;

    let mut min = 0.0;
    let mut max = 100000000000000.0;

    monkeys.insert("humn".to_string(), Job::Num(min));
    let right_min = get_value(&monkeys, &root_right);
    let left_min = get_value(&monkeys, &root_left);

    monkeys.insert("humn".to_string(), Job::Num(max));
    let right_max = get_value(&monkeys, &root_right);
    let left_max = get_value(&monkeys, &root_left);

    let increasing = (right_min < right_max) || (left_min < left_max);

    while min < max {
        let mid = (min + max) / 2.0;
        monkeys.insert("humn".to_string(), Job::Num(mid));
        let right = get_value(&monkeys, &root_right);
        let left = get_value(&monkeys, &root_left);

        if left == right {
            result = mid;
            break;
        } else if (left < right) ^ increasing {
            max = mid;
        } else {
            min = mid;
        }
    }

    result.to_string()
}

crate::run!();

crate::test_example_aoc!(152, 301);

crate::test_aoc!(256997859093114f64, 3952288690726u64);
