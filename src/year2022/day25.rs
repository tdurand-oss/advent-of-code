fn snafu_to_int(snafu: &str) -> i64 {
    let mut n = 0;
    for ch in snafu.chars() {
        let d = match ch {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid digit"),
        };

        n = 5 * n + d;
    }

    n
}

fn int_to_snafu(n: i64) -> String {
    let mut n = n;

    let mut snafu = String::new();
    while n > 0 {
        let d = n % 5;
        n /= 5;
        let (d, c) = match d {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => panic!("Invalid digit"),
        };
        n += c;

        snafu.push(d);
    }

    snafu.chars().rev().collect()
}

fn part1(input: &str) -> String {
    let sum = input.lines().map(snafu_to_int).sum::<i64>();

    int_to_snafu(sum)
}

fn part2(_input: &str) -> String {
    0.to_string()
}

crate::run!();

crate::test_example_aoc!("2=-1=0", 0);

crate::test_aoc!("20=02=120-=-2110-0=1", 0);
