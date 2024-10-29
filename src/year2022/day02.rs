#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandShape {
    Rock,
    Scissor,
    Paper,
}

impl HandShape {
    fn win_against(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Scissor,
            HandShape::Scissor => HandShape::Paper,
            HandShape::Paper => HandShape::Rock,
        }
    }

    fn lose_against(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Scissor => HandShape::Rock,
            HandShape::Paper => HandShape::Scissor,
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'A' | 'X' => HandShape::Rock,
            'B' | 'Y' => HandShape::Paper,
            'C' | 'Z' => HandShape::Scissor,
            _ => panic!("Unknown letter {}", letter),
        }
    }

    fn get_score(&self, opponent: HandShape) -> u32 {
        let fight_points = if *self == opponent.win_against() {
            0
        } else if *self == opponent {
            3
        } else {
            6
        };

        let hand_points = match self {
            HandShape::Rock => 1,
            HandShape::Scissor => 3,
            HandShape::Paper => 2,
        };

        fight_points + hand_points
    }
}

fn preprocess(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line.chars().nth(2).unwrap()))
        .collect()
}

fn part1(input: &str) -> String {
    let rounds = preprocess(input);
    rounds
        .iter()
        .map(|round| {
            let opponent = HandShape::from_letter(round.0);
            let player = HandShape::from_letter(round.1);
            player.get_score(opponent)
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let rounds = preprocess(input);
    rounds
        .iter()
        .map(|round| {
            let opponent = HandShape::from_letter(round.0);
            let player = match round.1 {
                'X' => opponent.win_against(),
                'Y' => opponent,
                'Z' => opponent.lose_against(),
                letter => panic!("Unknown letter {}", letter),
            };
            player.get_score(opponent)
        })
        .sum::<u32>()
        .to_string()
}

crate::run!();

crate::test_example_aoc!(15, 12);

crate::test_aoc!(14375, 10274);
