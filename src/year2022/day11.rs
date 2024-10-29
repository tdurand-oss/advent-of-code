#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl Operation {
    fn execute(&self, input: i64) -> i64 {
        match self {
            Operation::Add(operand) => input + operand,
            Operation::Multiply(operand) => input * operand,
            Operation::Square => input * input,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisble_by: i64,
    monkey_true: usize,
    monkey_false: usize,
}

impl Monkey {
    fn parse(lines: &[&str]) -> Self {
        let items = lines[1][18..]
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let operation = {
            let op = &lines[2][23..];
            if op == "* old" {
                Operation::Square
            } else {
                let operand = op[2..].parse::<i64>().unwrap();
                if op.starts_with('*') {
                    Operation::Multiply(operand)
                } else {
                    Operation::Add(operand)
                }
            }
        };
        let divisble_by = lines[3][21..].parse().unwrap();
        let monkey_true = lines[4][29..].parse().unwrap();
        let monkey_false = lines[5][30..].parse().unwrap();

        Self {
            items,
            operation,
            divisble_by,
            monkey_true,
            monkey_false,
        }
    }
}

fn run_rounds<F>(monkeys: Vec<Monkey>, rounds: usize, worry_modification: F) -> usize
where
    F: Fn(i64) -> i64,
{
    let mut monkeys = monkeys;
    let mut inspects = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            inspects[index] += monkeys[index].items.len();
            while !monkeys[index].items.is_empty() {
                let worry = monkeys[index].items.remove(0);
                let worry = monkeys[index].operation.execute(worry);
                let worry = worry_modification(worry);
                if worry % monkeys[index].divisble_by == 0 {
                    let throw_monkey = monkeys[index].monkey_true;
                    monkeys[throw_monkey].items.push(worry);
                } else {
                    let throw_monkey = monkeys[index].monkey_false;
                    monkeys[throw_monkey].items.push(worry);
                }
            }
        }
    }

    inspects.sort();
    inspects[inspects.len() - 1] * inspects[inspects.len() - 2]
}

fn preprocess(input: &str) -> Vec<Monkey> {
    let lines = input.lines().collect::<Vec<_>>();
    let monkeys = lines.chunks(7).map(Monkey::parse).collect::<Vec<_>>();
    monkeys
}

fn part1(input: &str) -> String {
    let monkeys = preprocess(input);
    let monkey_business = run_rounds(monkeys, 20, |worry| worry / 3);
    monkey_business.to_string()
}

fn part2(input: &str) -> String {
    let monkeys = preprocess(input);
    let factor = monkeys
        .iter()
        .map(|monkey| monkey.divisble_by)
        .product::<i64>();
    let monkey_business = run_rounds(monkeys, 10000, |worry| worry % factor);
    monkey_business.to_string()
}

crate::run!();

crate::test_example_aoc!(10605, 2713310158usize);

crate::test_aoc!(112815, 25738411485usize);
