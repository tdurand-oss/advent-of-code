mod year2021;
mod year2022;
mod year2023;

#[cfg(test)]
fn get_stem_name(path: &std::path::Path) -> String {
    path
    .file_stem()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned()
}

#[macro_export]
macro_rules! test_aoc {
    ($star1:expr, $star2:expr) => {
        #[cfg(test)]
        mod test {
            #[test]
            fn run() {
                let current_file_path = std::path::PathBuf::from(file!());
                let day = crate::get_stem_name(&current_file_path);
                let year = crate::get_stem_name(&current_file_path.parent().unwrap());

                let input = std::fs::read_to_string(format!("rsc/{}/input/{}.txt", year, day)).unwrap();
                assert_eq!(super::run(&input, &input), ($star1.to_string(), $star2.to_string()));
            }
        }
    };
}

#[macro_export]
macro_rules! test_example_aoc {
    ($star1:expr, $star2:expr) => {
        #[cfg(test)]
        mod example {
            #[test]
            fn run() {
                let current_file_path = std::path::PathBuf::from(file!());
                let day = crate::get_stem_name(&current_file_path);
                let year = crate::get_stem_name(&current_file_path.parent().unwrap());

                let path = format!("rsc/{}/example/{}.txt", year, day);
                let (input1, input2) = match std::fs::read_to_string(&path) {
                    Ok(input) => (input.clone(), input),
                    Err(_) => {
                        let input1 = std::fs::read_to_string(format!("rsc/{}/example/{}-1.txt", year, day)).unwrap();
                        let input2 = std::fs::read_to_string(format!("rsc/{}/example/{}-2.txt", year, day)).unwrap();
                        (input1, input2)
                    }
                };
                assert_eq!(super::run(&input1, &input2), ($star1.to_string(), $star2.to_string()));
            }
        }
    };
}

#[macro_export]
macro_rules! run {
    () => {
        pub fn run(input1: &str, input2: &str) -> (String, String) {
            (part1(&input1), part2(&input2))
        }
    };
}


#[macro_export]
macro_rules! run_challenge {
    ($year:expr) => {
        pub fn run_challenges() {
            for day in 1..26 {
                run_challenge(day)
            }
        }

        pub fn run_challenge(day: u32) {
            let input = std::fs::read_to_string(format!("rsc/year{}/input/day{:02}.txt", $year, day)).unwrap();
        
            let start = std::time::Instant::now();
        
            let (star1, star2) = match day {
                1 => day01::run(&input, &input),
                2 => day02::run(&input, &input),
                3 => day03::run(&input, &input),
                4 => day04::run(&input, &input),
                5 => day05::run(&input, &input),
                6 => day06::run(&input, &input),
                7 => day07::run(&input, &input),
                8 => day08::run(&input, &input),
                9 => day09::run(&input, &input),
                10 => day10::run(&input, &input),
                11 => day11::run(&input, &input),
                12 => day12::run(&input, &input),
                13 => day13::run(&input, &input),
                14 => day14::run(&input, &input),
                15 => day15::run(&input, &input),
                16 => day16::run(&input, &input),
                17 => day17::run(&input, &input),
                18 => day18::run(&input, &input),
                19 => day19::run(&input, &input),
                20 => day20::run(&input, &input),
                21 => day21::run(&input, &input),
                22 => day22::run(&input, &input),
                23 => day23::run(&input, &input),
                24 => day24::run(&input, &input),
                25 => day25::run(&input, &input),
                _ => panic!("Unknown day"),
            };
        
            let elapsed_time = start.elapsed();
        
            println!(
                "| {:04} | {:02} | {:>20} | {:>20} | {:>8} ms|",
                $year,
                day,
                star1,
                star2,
                elapsed_time.as_millis()
            );
        }
        
    };
}

#[derive(argh::FromArgs)]
#[argh(description = "Advent Of Code")]
struct Args {
    #[argh(positional, default = "String::new()")]
    #[argh(description = "year or year.day")]
    test_filter: String,
}

fn main() {
    let args: Args = argh::from_env();
    let day_regex = regex::Regex::new(r"^(?P<y>\d{4}).(?P<d>\d{2})$").unwrap();
    let year_regex = regex::Regex::new(r"^(?P<y>\d{4})$").unwrap();

    println!("            +----------------------+----------------------+------------+");
    println!("            |        Star 1        |        Star 2        |    Time    |");
    println!("+------+----+----------------------+----------------------+------------+");
    let start = std::time::Instant::now();
    
    if args.test_filter.len() == 0 {
        year2022::run_challenges();
        year2023::run_challenges();
    }
    else if let Some(cap) = year_regex.captures(&args.test_filter) {
        let year = cap.name("y").unwrap().as_str().parse::<u32>().unwrap();
        match year {
            2021 => year2021::run_challenges(),
            2022 => year2022::run_challenges(),
            2023 => year2023::run_challenges(),
            _ => panic!("Invalid year"),
        }
    }
    else if let Some(cap) = day_regex.captures(&args.test_filter) {
        let year = cap.name("y").unwrap().as_str().parse::<u32>().unwrap();
        let day = cap.name("d").unwrap().as_str().parse::<u32>().unwrap();
        match year {
            2021 => year2021::run_challenge(day),
            2022 => year2022::run_challenge(day),
            2023 => year2023::run_challenge(day),
            _ => panic!("Invalid year"),
        }
    }
    else {
        panic!("Invalid test filter");
    }

    let elapsed_time = start.elapsed();
    println!("+------+----+----------------------+----------------------+------------+");
    println!("+      |    |                      |                      | {:>8} ms|", elapsed_time.as_millis());
    println!("+------+----+----------------------+----------------------+------------+");
}
