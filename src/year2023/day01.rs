fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let first = line.chars().filter(|c| c.is_numeric()).next().unwrap() as u32 - '0' as u32;
        let last = line.chars().rev().filter(|c| c.is_numeric()).next().unwrap() as u32 - '0' as u32; 
        let number = 10 * first + last;
        sum += number;
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum = 0;
    for line in input.lines() {
        // Find first of names
        let (first_name_index, first_name_value) = names.iter()
            .enumerate()
            .filter_map(|(name_index, name)| line.find(name).and_then(|index| Some((index, name_index as u32 + 1))))
            .min_by_key(|(index, _)| *index)
            .unwrap_or((line.len(), 0));

        // Find first digit
        let (first_digit_index, first_digit_value) = line
            .find(|ch: char| ch.is_numeric())
            .and_then(|index| Some((index, line.chars().nth(index).unwrap().to_digit(10).unwrap())))
            .unwrap_or((line.len(), 0));

        // Find last of names
        let (last_name_index, last_name_value) = names.iter()
            .enumerate()
            .filter_map(|(name_index, name)| line.rfind(name).and_then(|index| Some((index, name_index as u32 + 1))))
            .max_by_key(|(index, _)| *index)
            .unwrap_or((0, 0));

        // Find last digit
        let (last_digit_index, last_digit_value) = line
            .rfind(|ch: char| ch.is_numeric())
            .and_then(|index| Some((index, line.chars().nth(index).unwrap().to_digit(10).unwrap())))
            .unwrap_or((0, 0));

        // Result
        let first_value = if first_name_index < first_digit_index { first_name_value } else { first_digit_value };
        let last_value = if last_name_index > last_digit_index { last_name_value } else { last_digit_value };
        let number = 10 * first_value + last_value;
        sum += number;
    }

    sum.to_string()
}

crate::run!();

crate::test_example_aoc!(142, 281);

crate::test_aoc!(55834, 53221);
