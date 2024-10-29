fn search_start_of_message(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find_map(|(index, chars)| {
            let chars = chars.iter().collect::<std::collections::HashSet<_>>();
            if chars.len() == window_size {
                Some(index + window_size)
            } else {
                None
            }
        })
        .unwrap()
}

fn part1(input: &str) -> String {
    search_start_of_message(input, 4).to_string()
}

fn part2(input: &str) -> String {
    search_start_of_message(input, 14).to_string()
}

crate::run!();

crate::test_example_aoc!(11, 26);

crate::test_aoc!(1850, 2823);
