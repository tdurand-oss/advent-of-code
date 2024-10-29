#[derive(Debug, Clone, Copy)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Shape {
    fn get_tiles(self) -> Vec<(isize, isize)> {
        match self {
            Shape::Horizontal => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Shape::Cross => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Shape::Corner => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            Shape::Vertical => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}

struct Cave {
    cave: Vec<Vec<bool>>,
    jets: Vec<char>,
    jet_index: usize,
    start_x: usize,
}

impl Cave {
    fn new(jets: Vec<char>) -> Self {
        Self {
            cave: Vec::new(),
            jets,
            jet_index: 0,
            start_x: 4,
        }
    }

    fn add_block(&mut self, shape: Shape) {
        let mut position = (self.start_x as isize, 2);
        self.cave.resize(self.start_x + 3, vec![false; 7]);
        // println!("init {:?}", position);
        loop {
            // Blocks falls
            let fall = (position.0 - 1, position.1);
            if !self.valid_block_position(shape, fall) {
                break;
            }

            // Jet pushes the block
            let shifted = if self.jets[self.jet_index] == '<' {
                (fall.0, fall.1 - 1)
            } else {
                (fall.0, fall.1 + 1)
            };

            self.jet_index = (self.jet_index + 1) % self.jets.len();
            position = if self.valid_block_position(shape, shifted) {
                // println!("shifted {:?}", shifted);
                shifted
            } else {
                // println!("fall {:?}", fall);
                fall
            }
        }

        // Add block and update start position
        for tile in shape.get_tiles() {
            self.cave[(position.0 + tile.0) as usize][(position.1 + tile.1) as usize] = true;
            self.start_x = self.start_x.max((position.0 + tile.0) as usize + 5);
        }
        // println!("y {:?}", self.start_y);
        // self.print();
    }

    fn valid_block_position(&self, shape: Shape, position: (isize, isize)) -> bool {
        for tile in shape.get_tiles() {
            if !self.free_tile((position.0 + tile.0, position.1 + tile.1)) {
                return false;
            }
        }

        true
    }

    fn free_tile(&self, position: (isize, isize)) -> bool {
        if position.0 < 0
            || position.0 >= self.cave.len() as isize
            || position.1 < 0
            || position.1 >= self.cave[position.0 as usize].len() as isize
        {
            false
        } else {
            !self.cave[position.0 as usize][position.1 as usize]
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for line in self.cave.iter().rev().take(100) {
            for tile in line {
                if *tile {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn find_pattern_start(data: &[usize], size: usize) -> (usize, usize) {
    for start in 0..data.len() - size {
        for s in start + 1..data.len() - size {
            if data[s..s + size] == data[start..start + size] {
                return (start, s - start);
            }
        }
    }

    panic!("Pattern not found");
}

fn part1(input: &str) -> String {
    let jets = input.chars().collect::<Vec<_>>();

    let rocks = vec![
        Shape::Horizontal,
        Shape::Cross,
        Shape::Corner,
        Shape::Vertical,
        Shape::Square,
    ];

    let mut cave = Cave::new(jets.clone());

    for index in 0..2022 {
        cave.add_block(rocks[index % rocks.len()]);
    }
    let height1 = cave.start_x - 4;

    height1.to_string()
}

fn part2(input: &str) -> String {
    let jets = input.chars().collect::<Vec<_>>();

    let rocks = vec![
        Shape::Horizontal,
        Shape::Cross,
        Shape::Corner,
        Shape::Vertical,
        Shape::Square,
    ];

    let n = 100;
    let mut cave = Cave::new(jets);

    // Drop a few rocks
    let mut increase = Vec::new();
    let mut last = 0;
    for index in 0..40000 {
        cave.add_block(rocks[index % rocks.len()]);
        if index != 0 && index % n == 0 {
            increase.push((cave.start_x - 4) - last);
            last = cave.start_x - 4;
        }
    }

    let size = 5;
    let (start, length) = find_pattern_start(&increase, size);

    let pattern0 = &increase[..start];
    let pattern = &increase[start..start + length];

    // let pattern0 = [153, 159, 145, 159, 154, 174, 152, 153, 150];
    // let pattern = [145, 144, 169, 150, 169, 155, 152, 161, 155, 164, 165, 171, 171, 172, 143, 153, 157, 141, 149, 168, 153, 167, 155, 146, 169, 152, 159, 173, 176, 158, 173, 148, 150, 162, 143, 149, 166, 154, 160, 156, 152, 165, 159, 156, 174, 167, 164, 174, 149, 148, 163, 144, 147, 165, 155, 158, 155, 159, 157, 159, 162, 167, 168, 165, 179, 154, 145, 157, 152, 146, 160, 157, 155, 160, 159, 153, 158, 163, 157, 175, 162, 180, 161, 143, 150, 164];

    let total = 1000000000000usize / n;
    let n0 = pattern0.len();
    let m = (total - n0) / pattern.len();
    let left = (total - n0) - m * pattern.len();
    let height2: usize = m * pattern.iter().sum::<usize>()
        + pattern0.iter().sum::<usize>()
        + pattern[..left].iter().sum::<usize>()
        - 1;

    height2.to_string()
}

crate::run!();

crate::test_example_aoc!(3068, 1514285714288usize);

crate::test_aoc!(3171, 1586627906921usize);
