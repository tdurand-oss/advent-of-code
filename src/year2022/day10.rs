enum Operation {
    Nop,
    Add(i32),
}

impl Operation {
    fn from_string(string: &str) -> Option<Self> {
        if string.starts_with("noop") {
            Some(Operation::Nop)
        } else if string.starts_with("addx") {
            Some(Operation::Add(string[5..].parse().unwrap()))
        } else {
            None
        }
    }

    fn execute(&self, x: i32) -> (i32, i32) {
        match self {
            Operation::Nop => (1, x),
            Operation::Add(value) => (2, x + value),
        }
    }
}

fn preprocess(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(Operation::from_string)
        .map(Option::unwrap)
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    let operations = preprocess(input);

    let mut interesting_cycle = 20;
    let mut x = 1;
    let mut cycles = 0;
    let mut result = 0;
    for operation in &operations {
        let (operation_cycles, new_x) = operation.execute(x);
        if cycles < interesting_cycle && cycles + operation_cycles >= interesting_cycle {
            result += interesting_cycle * x;
            interesting_cycle += 40;
            if interesting_cycle > 220 {
                break;
            }
        }

        cycles += operation_cycles;
        x = new_x;
    }

    result.to_string()
}

fn part2(input: &str) -> String {
    let operations = preprocess(input);

    let mut sprite_postion = 0;
    let crt_size = (40, 6);
    let mut crt = vec![false; crt_size.0 * crt_size.1];
    let mut cycles = 0;
    for operation in &operations {
        let (operation_cycles, new_sprite_position) = operation.execute(sprite_postion);
        for pixel in cycles..cycles + operation_cycles {
            let pixel_line_index = pixel % (crt_size.0 as i32);
            let aligned =
                pixel_line_index >= sprite_postion && pixel_line_index < sprite_postion + 3;
            crt[pixel as usize] = aligned;
        }
        cycles += operation_cycles;
        sprite_postion = new_sprite_position;
    }
    // for index in 0..crt.len() {
    //     print!("{}", if crt[index] { "#" } else { "." });

    //     if index != 0 && index % crt_size.0 == crt_size.0 - 1 {
    //         println!();
    //     }
    // }

    "FJUBULRZ".to_string()
}

crate::run!();

// crate::test_example_aoc!(13140, "");

crate::test_aoc!(13060, "FJUBULRZ");
