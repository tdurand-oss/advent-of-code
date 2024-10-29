use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dist(&self, p: Pos) -> i64 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

#[derive(Debug, Clone)]
struct Reading {
    sensor: Pos,
    beacon: Pos,
    radius: i64,
}

impl Reading {
    fn from_string(input: &str) -> Self {
        let regex =
            regex::Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")
                .unwrap();
        let cap = regex.captures(input).unwrap();
        let sensor = Pos::new(
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let beacon = Pos::new(
            cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        );
        Self {
            sensor,
            beacon,
            radius: sensor.dist(beacon),
        }
    }

    fn covers(&self, pos: Pos) -> bool {
        // println!("{} {} ", self.sensor.dist(pos), self.sensor.dist(self.beacon));
        self.sensor.dist(pos) <= self.radius
    }
}

fn find_overlap_sensors_line(readings: &[Reading], y: i64) -> Vec<(i64, i64)> {
    let mut intervals = Vec::new();

    for reading in readings {
        if reading.sensor.y - reading.radius <= y && reading.sensor.y + reading.radius >= y {
            let mut x0 = reading.sensor.x - reading.radius;
            while !reading.covers(Pos::new(x0, y)) {
                x0 += 1;
            }

            let mut x1 = reading.sensor.x + reading.radius;
            while !reading.covers(Pos::new(x1, y)) {
                x1 -= 1;
            }

            intervals.push((x0, x1));
        }
    }

    intervals.sort();

    let mut merged = Vec::new();
    merged.push(intervals[0]);

    for interval in &intervals[1..] {
        if interval.0 <= merged.last().unwrap().1 {
            merged.last_mut().unwrap().1 = interval.1.max(merged.last().unwrap().1);
        } else {
            merged.push(*interval);
        }
    }

    merged
}

fn find_occupied_tiles_line(readings: &[Reading], y: i64) -> HashSet<Pos> {
    let mut occupied = HashSet::new();
    for reading in readings {
        if reading.sensor.y == y {
            occupied.insert(reading.sensor);
        }

        if reading.beacon.y == y {
            occupied.insert(reading.beacon);
        }
    }

    occupied
}

fn find_uncovered(
    readings: &[Reading],
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
) -> Option<Pos> {
    // If a sensor covers the entire are we can eliminate it
    for reading in readings {
        let area_in_sensor_coverage = reading.covers(Pos::new(x_min, y_min))
            && reading.covers(Pos::new(x_min, y_max))
            && reading.covers(Pos::new(x_max, y_max))
            && reading.covers(Pos::new(x_max, y_min));

        if area_in_sensor_coverage {
            return None;
        }
    }

    // Only one location, we found our beacon
    if x_min == x_max && y_min == y_max {
        return Some(Pos::new(x_min, y_min));
    }

    let x_mid = (x_max + x_min) / 2;
    let y_mid = (y_max + y_min) / 2;

    // Divide in 4 subareas and try again
    if let Some(pos) = find_uncovered(readings, x_min, x_mid, y_min, y_mid) {
        return Some(pos);
    }

    if let Some(pos) = find_uncovered(readings, x_mid + 1, x_max, y_min, y_mid) {
        return Some(pos);
    }

    if let Some(pos) = find_uncovered(readings, x_mid + 1, x_max, y_mid + 1, y_max) {
        return Some(pos);
    }

    if let Some(pos) = find_uncovered(readings, x_min, x_mid, y_mid + 1, y_max) {
        return Some(pos);
    }

    None
}

fn part1(input: &str) -> String {
    // Input
    let readings = input.lines().map(Reading::from_string).collect::<Vec<_>>();

    // Star 1
    let y = 2000000;
    let intervals = find_overlap_sensors_line(&readings, y);

    let count: i64 = intervals
        .iter()
        .map(|interval| interval.1 - interval.0 + 1)
        .sum();

    let occupied = find_occupied_tiles_line(&readings, y);
    let count = count - occupied.len() as i64;

    count.to_string()
}

fn part2(input: &str) -> String {
    // Input
    let readings = input.lines().map(Reading::from_string).collect::<Vec<_>>();

    // Star 2
    let beacon = find_uncovered(&readings, 0, 4000000, 0, 4000000).unwrap();
    let freq = 4000000 * beacon.x + beacon.y;

    freq.to_string()
}

crate::run!();

// crate::test_example_aoc!(26, 56000011);

crate::test_aoc!(5716881, 10852583132904i64);
