fn parse_input(input: &str) -> Vec<Vec<[u32; 3]>> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut game = Vec::new();
        let records = line.split(':').nth(1).unwrap();
        for record in records.split(';') {
            let mut game_cubes = [0, 0, 0];
            for cubes in record.split(", ") {
                let mut split = cubes.trim().split(' ');
                let count = split.next().unwrap().parse::<u32>().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => game_cubes[0] = count,
                    "green" => game_cubes[1] = count,
                    "blue" => game_cubes[2] = count,
                    _ => panic!(),
                }
            }
            game.push(game_cubes);
        }
        games.push(game);
    }
    games
}

fn part1(input: &str) -> String {
    let games = parse_input(input);
    let bag = [12, 13, 14];
    let sum: u32 = games.iter()
        .enumerate()
        .filter_map(|(game_id, game)| {
            if game.iter().all(|cubes| cubes[0] <= bag[0] && cubes[1] <= bag[1] && cubes[2] <= bag[2]) {
                Some(game_id as u32 + 1)
            }
            else {
                None
            }
        })
        .sum();

    sum.to_string()
}

fn part2(input: &str) -> String {
    let games = parse_input(input);
    let sum: u32 = games.iter()
        .map(|game| {
            let mut bag = [0, 0, 0];
            for record in game {
                bag[0] = bag[0].max(record[0]);
                bag[1] = bag[1].max(record[1]);
                bag[2] = bag[2].max(record[2]);
            }
            bag[0] * bag[1] * bag[2]
        })
        .sum();

    sum.to_string()
}

crate::run!();

crate::test_example_aoc!(8, 2286);

crate::test_aoc!(1853, 72706);
