use std::collections::HashMap;

struct NumberInfo {
    row: isize,
    cols: std::ops::Range<isize>,
    value: u32,
}

struct Schematic {
    grid: Vec<Vec<char>>,
    number_infos: Vec<NumberInfo>,
}

impl Schematic {
    fn from_str(input: &str) -> Self {
        let grid = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        // Find numbers
        let mut number_infos = Vec::new();
        for row in 0..grid.len() {
            let mut start = None;
            let mut end = None;
            let mut number = 0;
            for col in 0..grid[row].len() {
                if grid[row][col].is_digit(10) {
                    if start.is_none() {
                        start = Some(col);
                    }
                    end = Some(col);
                    number = 10 * number + grid[row][col].to_digit(10).unwrap();
                }
                else {
                    if start.is_some() {
                        number_infos.push(NumberInfo { row: row as isize, cols: start.unwrap() as isize .. end.unwrap() as isize + 1, value: number });
                        start = None;
                        end = None;
                        number = 0;
                    }
                }
            }
            if start.is_some() {
                number_infos.push(NumberInfo { row: row as isize, cols: start.unwrap() as isize .. end.unwrap() as isize + 1, value: number });
            }
        }

        Self {
            grid,
            number_infos,
        }
    }

    fn get(&self, row: isize, col: isize) -> char {
        if row < 0 || col < 0 {
            return '.';
        }
        match self.grid.get(row as usize) {
            Some(line) => *line.get(col as usize).unwrap_or(&'.'),
            None => '.'
        }
    }
}

fn part1(input: &str) -> String {
    let schematic = Schematic::from_str(input); 

    let is_symbol = |ch: char| { !ch.is_digit(10) && ch != '.'};

    let mut sum = 0;
    for number in &schematic.number_infos {
        let mut valid = false;
        valid |= is_symbol(schematic.get(number.row - 1, number.cols.start - 1));
        valid |= is_symbol(schematic.get(number.row + 0, number.cols.start - 1));
        valid |= is_symbol(schematic.get(number.row + 1, number.cols.start - 1));

        for col in number.cols.clone() {
            valid |= is_symbol(schematic.get(number.row - 1, col));
            valid |= is_symbol(schematic.get(number.row + 1, col));
        }
        valid |= is_symbol(schematic.get(number.row - 1, number.cols.end));
        valid |= is_symbol(schematic.get(number.row + 0, number.cols.end));
        valid |= is_symbol(schematic.get(number.row + 1, number.cols.end));

        if valid {
            sum += number.value;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let schematic = Schematic::from_str(input); 

    let mut gears: HashMap<(isize, isize), Vec<u32>> = HashMap::new();
    let mut check_gear = |row: isize, col: isize, value: u32| {
        if schematic.get(row, col) == '*' {
            gears.entry((row, col)).or_default().push(value);
        }
    };

    for number in &schematic.number_infos {
        check_gear(number.row - 1, number.cols.start - 1, number.value);
        check_gear(number.row + 0, number.cols.start - 1, number.value);
        check_gear(number.row + 1, number.cols.start - 1, number.value);

        for col in number.cols.clone() {
            check_gear(number.row - 1, col, number.value);
            check_gear(number.row + 1, col, number.value);
        }

        check_gear(number.row - 1, number.cols.end, number.value);
        check_gear(number.row + 0, number.cols.end, number.value);
        check_gear(number.row + 1, number.cols.end, number.value);
    }
    
    let sum: u32 = gears.iter()
        .map(|(_, numbers)| if numbers.len() == 2 { numbers[0] * numbers[1]} else { 0 })
        .sum();

    sum.to_string()
}

crate::run!();

crate::test_example_aoc!(4361, 467835);

crate::test_aoc!(556367, 89471771);