#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct Cave {
    map: Vec<Vec<Tile>>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
    infinite_floor: bool,
}

impl Cave {
    fn from_rocks(rocks: &[Vec<Position>], infinite_floor: bool) -> Self {
        let mut x_min = usize::MAX;
        let mut x_max = usize::MIN;
        let mut y_max = usize::MIN;
        for rock in rocks {
            for corner in rock {
                x_min = x_min.min(corner.x);
                x_max = x_max.max(corner.x);
                y_max = y_max.max(corner.y);
            }
        }

        if infinite_floor {
            y_max += 2;
        }

        let mut map = vec![vec![Tile::Air; y_max + 1]; x_max - x_min + 1];

        for rock in rocks {
            for index in 0..rock.len() - 1 {
                if rock[index].x == rock[index + 1].x {
                    let min = rock[index].y.min(rock[index + 1].y);
                    let max = rock[index].y.max(rock[index + 1].y);
                    for r in min..max + 1 {
                        map[rock[index].x - x_min][r] = Tile::Rock;
                    }
                } else {
                    let min = rock[index].x.min(rock[index + 1].x);
                    let max = rock[index].x.max(rock[index + 1].x);
                    for r in min..max + 1 {
                        map[r - x_min][rock[index].y] = Tile::Rock;
                    }
                }
            }
        }

        if infinite_floor {
            for col in &mut map {
                col[y_max] = Tile::Rock;
            }
        }

        Self {
            map,
            x_min,
            x_max,
            y_max,
            infinite_floor,
        }
    }

    fn set(&mut self, position: Position, tile: Tile) {
        self.map[position.x - self.x_min][position.y] = tile;
    }

    fn get(&self, position: Position) -> Tile {
        self.map[position.x - self.x_min][position.y]
    }

    fn drop_sand(&mut self, position: Position) -> Option<Position> {
        let mut position = position;
        loop {
            let below = Position::new(position.x, position.y + 1);
            let below_left = Position::new(position.x - 1, position.y + 1);
            let below_right = Position::new(position.x + 1, position.y + 1);

            // Increase map size if needed
            if below_left.x < self.x_min {
                self.x_min -= 1;
                self.map.insert(0, vec![Tile::Air; self.y_max + 1]);
                if self.infinite_floor {
                    self.set(Position::new(self.x_min, self.y_max), Tile::Rock);
                }
            }

            if below_right.x >= self.x_min + self.map.len() {
                self.x_max += 1;
                self.map.push(vec![Tile::Air; self.y_max + 1]);
                if self.infinite_floor {
                    self.set(Position::new(self.x_max, self.y_max), Tile::Rock);
                }
            }

            // Check if falling into the abyss
            if !self.infinite_floor && below.y > self.y_max {
                return None; // Sand falls into the abyss
            }

            // Fall if possible
            if self.get(below) == Tile::Air {
                position = below;
            } else if self.get(below_left) == Tile::Air {
                position = below_left;
            } else if self.get(below_right) == Tile::Air {
                position = below_right;
            } else {
                self.set(position, Tile::Sand);
                return Some(position); // Sand stopped here
            }
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                match self.map[x][y] {
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                    Tile::Air => print!("."),
                }
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|string| {
                    let index = string.find(',').unwrap();
                    Position {
                        x: string[..index].parse::<usize>().unwrap(),
                        y: string[index + 1..].parse::<usize>().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    // Input
    let rocks = parse_input(input);

    let sand_source = Position::new(500, 0);

    // Star 1
    let mut cave = Cave::from_rocks(&rocks, false);

    let mut count1 = 0;
    while cave.drop_sand(sand_source).is_some() {
        count1 += 1;
    }

    count1.to_string()
}

fn part2(input: &str) -> String {
    // Input
    let rocks = parse_input(input);

    let sand_source = Position::new(500, 0);

   // Star 2
   let mut cave = Cave::from_rocks(&rocks, true);

   let mut count2 = 1;
   while cave.drop_sand(sand_source) != Some(sand_source) {
       count2 += 1;
   }

    count2.to_string()
}

crate::run!();

crate::test_example_aoc!(24, 93);

crate::test_aoc!(873, 24813);
