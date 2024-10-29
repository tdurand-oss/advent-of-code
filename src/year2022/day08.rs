use std::collections::HashSet;

enum ViewDirection {
    North,
    East,
    South,
    West,
}

struct ForestView<'m, T: Copy> {
    data: &'m Vec<Vec<T>>,
    direction: ViewDirection,
}

impl<'m, T: Copy> ForestView<'m, T> {
    fn new(data: &'m Vec<Vec<T>>, direction: ViewDirection) -> Self {
        Self { data, direction }
    }

    fn get(&self, i: usize, j: usize) -> T {
        match self.direction {
            ViewDirection::North => self.data[i][j],
            ViewDirection::East => self.data[j][i],
            ViewDirection::South => self.data[i][self.size() - 1 - j],
            ViewDirection::West => self.data[self.size() - 1 - j][i],
        }
    }

    fn get_original_coordinate(&self, i: usize, j: usize) -> (usize, usize) {
        match self.direction {
            ViewDirection::North => (i, j),
            ViewDirection::East => (j, i),
            ViewDirection::South => (i, self.size() - 1 - j),
            ViewDirection::West => (self.size() - 1 - j, i),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn find_visible_from_side(forest: ForestView<i32>) -> HashSet<(usize, usize)> {
    let mut visible = HashSet::new();

    for row in 0..forest.size() {
        let mut visible_height = -1;
        for col in 0..forest.size() {
            if forest.get(row, col) > visible_height {
                visible.insert(forest.get_original_coordinate(row, col));
                visible_height = forest.get(row, col);
            }
        }
    }

    visible
}

fn compute_scenic(scenic: &mut Vec<Vec<i32>>, forest: ForestView<i32>) {
    for row in 1..forest.size() - 1 {
        let mut last_tree_of_size = vec![-1; 10];

        for col in 1..scenic.len() - 1 {
            let current_height = forest.get(row, col);
            let mut visible_distance = col;

            // Find visible distance by checking all previous tree taller than the current one
            for height in current_height..10 {
                if last_tree_of_size[height as usize] < 0 {
                    continue;
                }
                let distance_to_next_tree_of_height =
                    col - last_tree_of_size[height as usize] as usize;
                visible_distance = visible_distance.min(distance_to_next_tree_of_height);
            }

            last_tree_of_size[current_height as usize] = col as i32;

            let coordinate = forest.get_original_coordinate(row, col);
            scenic[coordinate.0][coordinate.1] *= visible_distance as i32;
        }
    }
}

fn preprocess(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> String {
    let forest = preprocess(input);

    let mut visible = HashSet::new();
    visible.extend(find_visible_from_side(ForestView::new(
        &forest,
        ViewDirection::North,
    )));
    visible.extend(find_visible_from_side(ForestView::new(
        &forest,
        ViewDirection::East,
    )));
    visible.extend(find_visible_from_side(ForestView::new(
        &forest,
        ViewDirection::South,
    )));
    visible.extend(find_visible_from_side(ForestView::new(
        &forest,
        ViewDirection::West,
    )));

    visible.len().to_string()
}

fn part2(input: &str) -> String {
    let forest = preprocess(input);

    let mut scenic = vec![vec![1i32; forest.len()]; forest.len()];
    compute_scenic(&mut scenic, ForestView::new(&forest, ViewDirection::North));
    compute_scenic(&mut scenic, ForestView::new(&forest, ViewDirection::East));
    compute_scenic(&mut scenic, ForestView::new(&forest, ViewDirection::South));
    compute_scenic(&mut scenic, ForestView::new(&forest, ViewDirection::West));
    let max_scenic = scenic.iter().flatten().max().unwrap();

    max_scenic.to_string()
}

crate::run!();

crate::test_example_aoc!(21, 8);

crate::test_aoc!(1736, 268800);
