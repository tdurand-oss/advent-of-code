fn preprocess(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                (
                    cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                ),
            )
        })
        .collect()
}

fn part1(input: &str) -> String {
    let pairs = preprocess(input);
    pairs
        .iter()
        .map(|(pair1, pair2)| {
            u32::from(
                (pair1.0 >= pair2.0 && pair1.1 <= pair2.1)
                    || (pair2.0 >= pair1.0 && pair2.1 <= pair1.1),
            )
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let pairs = preprocess(input);
    pairs
        .iter()
        .map(|(pair1, pair2)| u32::from((pair1.0 <= pair2.1) && (pair1.1 >= pair2.0)))
        .sum::<u32>()
        .to_string()
}

crate::run!();

crate::test_example_aoc!(2, 4);

crate::test_aoc!(530, 903);
