#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Void,
    Open,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Tile::Void,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(i32),
    Turn(Direction),
}

struct Map {
    map: Vec<Vec<Tile>>,
    cube_size: i32,
}

impl Map {
    fn new(map: Vec<Vec<Tile>>, cube_size: i32) -> Self {
        Self { map, cube_size }
    }
}

type Position = (i32, i32);
type FaceIndex = (i32, i32);

impl Map {
    fn move_position(
        &self,
        mut pos: Position,
        mut dir: Direction,
        count: i32,
        wrap_cube: bool,
    ) -> (Position, Direction) {
        for _ in 0..count as usize {
            let (mut next, mut next_dir) = if wrap_cube {
                self.move_wrap_cube(pos, dir)
            } else {
                self.move_wrap(pos, dir)
            };
            // println!("  {:?} {:?} {:?}", next, next_dir, self.map[next.0 as usize][next.1 as usize]);
            while self.map[next.0 as usize][next.1 as usize] == Tile::Void {
                (next, next_dir) = if wrap_cube {
                    self.move_wrap_cube(next, next_dir)
                } else {
                    self.move_wrap(next, next_dir)
                };
            }

            if self.map[next.0 as usize][next.1 as usize] == Tile::Wall {
                return (pos, dir);
            }

            pos = next;
            dir = next_dir;
        }

        (pos, dir)
    }

    fn move_wrap(&self, mut pos: Position, dir: Direction) -> (Position, Direction) {
        // println!("{:?} {:?}", pos, dir);
        match dir {
            Direction::Right => pos.1 += 1,
            Direction::Down => pos.0 += 1,
            Direction::Left => pos.1 -= 1,
            Direction::Up => pos.0 -= 1,
        }

        if pos.0 == -1 {
            pos.0 = self.map.len() as i32 - 1;
        }

        if pos.0 == self.map.len() as i32 {
            pos.0 = 0;
        }

        if pos.1 == -1 {
            pos.1 = self.map[0].len() as i32 - 1;
        }

        if pos.1 == self.map[0].len() as i32 {
            pos.1 = 0;
        }

        // println!("  > {:?} {:?}", pos, dir);

        (pos, dir)
    }

    fn move_wrap_cube(&self, mut pos: Position, mut dir: Direction) -> (Position, Direction) {
        let face = (pos.0 / self.cube_size, pos.1 / self.cube_size);
        let relative_pos = (pos.0 % self.cube_size, pos.1 % self.cube_size);

        match dir {
            Direction::Right => pos.1 += 1,
            Direction::Down => pos.0 += 1,
            Direction::Left => pos.1 -= 1,
            Direction::Up => pos.0 -= 1,
        }
        // pos = (pos.0.rem_euclid(4 * self.cube_size), pos.1.rem_euclid(4 * self.cube_size));

        if pos.0 < 0
            || pos.0 >= self.map.len() as i32
            || pos.1 < 0
            || pos.1 >= self.map.len() as i32
            || self.map[pos.0 as usize][pos.1 as usize] == Tile::Void
        {
            let (next_face, next_dir, next_relative_pos) =
                self.wrap_position_cube(face, dir, relative_pos);
            pos = (
                next_face.0 * self.cube_size + next_relative_pos.0,
                next_face.1 * self.cube_size + next_relative_pos.1,
            );
            dir = next_dir;
        }
        // println!("  > {:?} {:?}", pos, dir);

        (pos, dir)
    }

    fn wrap_position_cube(
        &self,
        face: FaceIndex,
        dir: Direction,
        pos: Position,
    ) -> (FaceIndex, Direction, Position) {
        let c = self.cube_size - 1;
        match (face, dir) {
            ((0, 1), Direction::Left) => ((2, 0), Direction::Right, (c - pos.0, 0)),
            ((0, 1), Direction::Up) => ((3, 0), Direction::Right, (pos.1, 0)),
            ((0, 2), Direction::Down) => ((1, 1), Direction::Left, (pos.1, c)),
            ((0, 2), Direction::Right) => ((2, 1), Direction::Left, (c - pos.0, c)),
            ((0, 2), Direction::Up) => ((3, 0), Direction::Up, (c, pos.1)),
            ((1, 1), Direction::Left) => ((2, 0), Direction::Down, (0, pos.0)),
            ((1, 1), Direction::Right) => ((0, 2), Direction::Up, (c, pos.0)),
            ((2, 0), Direction::Left) => ((0, 1), Direction::Right, (c - pos.0, 0)),
            ((2, 0), Direction::Up) => ((1, 1), Direction::Right, (pos.1, 0)),
            ((2, 1), Direction::Down) => ((3, 0), Direction::Left, (pos.1, c)),
            ((2, 1), Direction::Right) => ((0, 2), Direction::Left, (c - pos.0, c)),
            ((3, 0), Direction::Down) => ((0, 2), Direction::Down, (0, pos.1)),
            ((3, 0), Direction::Left) => ((0, 1), Direction::Down, (0, pos.0)),
            ((3, 0), Direction::Right) => ((2, 1), Direction::Up, (c, pos.0)),
            // _ => (face, dir, pos),
            _ => panic!(),
        }
    }
}

fn apply_instructions(map: &Map, instructions: &[Instruction], wrap_cube: bool) -> i32 {
    let mut dir = Direction::Right;
    let mut pos = (0, 50);
    for instruction in instructions {
        match instruction {
            Instruction::Move(count) => {
                (pos, dir) = map.move_position(pos, dir, *count, wrap_cube);
            }
            Instruction::Turn(turn) => {
                dir = match turn {
                    Direction::Right => match dir {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Right,
                    },
                    Direction::Left => match dir {
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Left,
                    },
                    _ => unreachable!(),
                }
            }
        }

        // println!("{:?} {:?} {:?}", instruction, pos, dir);
    }

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir as i32
}

fn preprocess(input: &str) -> (Map, Vec<Instruction>) {
    let size = 50;
    let map = input
        .lines()
        .take(4 * size)
        .map(|line| {
            let mut v = line.chars().map(Tile::from_char).collect::<Vec<_>>();
            v.resize(4 * size, Tile::Void);
            v
        })
        .collect::<Vec<_>>();

    let map = Map::new(map, size as i32);

    let line = input.lines().last().unwrap();
    let mut instructions = Vec::new();
    let mut start = 0;
    while let Some(end) = line[start..].find(['R', 'L']) {
        let direction = line[start..].chars().nth(end).unwrap();
        instructions.push(Instruction::Move(
            line[start..start + end].parse::<i32>().unwrap(),
        ));
        instructions.push(Instruction::Turn(if direction == 'R' {
            Direction::Right
        } else {
            Direction::Left
        }));
        start += end + 1;
    }
    instructions.push(Instruction::Move(line[start..].parse::<i32>().unwrap()));

    (map, instructions)
}

fn part1(input: &str) -> String {
    let (map, instructions) = preprocess(input);
    let result = apply_instructions(&map, &instructions, false);
    result.to_string()
}

fn part2(input: &str) -> String {
    let (map, instructions) = preprocess(input);

    // Tests
    assert_eq!(
        map.move_wrap_cube((0, 50), Direction::Up),
        ((150, 0), Direction::Right)
    );
    assert_eq!(
        map.move_wrap_cube((0, 50), Direction::Left),
        ((149, 0), Direction::Right)
    );
    assert_eq!(
        map.move_wrap_cube((49, 149), Direction::Down),
        ((99, 99), Direction::Left)
    );
    assert_eq!(
        map.move_wrap_cube((49, 149), Direction::Right),
        ((100, 99), Direction::Left)
    );
    assert_eq!(
        map.move_wrap_cube((0, 100), Direction::Up),
        ((199, 0), Direction::Up)
    );
    assert_eq!(
        map.move_wrap_cube((50, 50), Direction::Left),
        ((100, 0), Direction::Down)
    );
    assert_eq!(
        map.move_wrap_cube((50, 99), Direction::Right),
        ((49, 100), Direction::Up)
    );
    assert_eq!(
        map.move_wrap_cube((100, 0), Direction::Left),
        ((49, 50), Direction::Right)
    );
    assert_eq!(
        map.move_wrap_cube((100, 0), Direction::Up),
        ((50, 50), Direction::Right)
    );
    assert_eq!(
        map.move_wrap_cube((149, 99), Direction::Down),
        ((199, 49), Direction::Left)
    );
    assert_eq!(
        map.move_wrap_cube((149, 99), Direction::Right),
        ((0, 149), Direction::Left)
    );
    assert_eq!(
        map.move_wrap_cube((199, 0), Direction::Down),
        ((0, 100), Direction::Down)
    );
    assert_eq!(
        map.move_wrap_cube((199, 0), Direction::Left),
        ((0, 99), Direction::Down)
    );
    assert_eq!(
        map.move_wrap_cube((199, 49), Direction::Right),
        ((149, 99), Direction::Up)
    );

    let result = apply_instructions(&map, &instructions, true);
    result.to_string()
}

crate::run!();

crate::test_aoc!(146092, 110342);
