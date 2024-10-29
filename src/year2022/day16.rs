use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Room {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Room {
    fn from_str(string: &str) -> Self {
        let r = regex::Regex::new("Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)")
            .unwrap();
        let cap = r.captures(string).unwrap();
        let valve = cap.get(1).unwrap().as_str().to_owned();
        let flow_rate = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let tunnels = cap
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Self {
            name: valve,
            flow_rate,
            tunnels,
        }
    }
}

fn compute_distances(rooms: &[Room]) -> Vec<Vec<u32>> {
    let mut distances = vec![vec![0; rooms.len()]; rooms.len()];

    let room_to_index = rooms
        .iter()
        .enumerate()
        .map(|(index, room)| (room.name.clone(), index))
        .collect::<HashMap<_, _>>();

    for room in rooms {
        let start = room_to_index[&room.name];
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        to_visit.push((start, 0));
        while !to_visit.is_empty() {
            let (current, distance) = to_visit.remove(0);

            if visited.contains(&current) {
                continue;
            }

            distances[start][current] = distance;
            distances[current][start] = distance;
            visited.insert(current);

            for tunnel in &rooms[current].tunnels {
                to_visit.push((room_to_index[tunnel], distance + 1));
            }
        }
    }

    distances
}

#[derive(Debug, Clone, Copy)]
struct State1 {
    released_pressure: u32,
    available_flow_rate: u32,
    room_index: usize,
    time_left: u32,
}

fn find_pressure1(
    rooms: &[Room],
    distances: &[Vec<u32>],
    state: State1,
    visited: &mut [bool],
    max_pressure: &mut u32,
) {
    // If all the available leftover pressure is not enough give up
    if state.released_pressure + state.available_flow_rate * state.time_left <= *max_pressure {
        return;
    }

    // Open the valve
    let released_pressure =
        state.released_pressure + state.time_left * rooms[state.room_index].flow_rate;
    *max_pressure = (*max_pressure).max(released_pressure);

    // Try to go in other tooms
    for tunnel in 0..distances.len() {
        if rooms[tunnel].flow_rate == 0
            || visited[tunnel]
            || state.time_left < distances[state.room_index][tunnel] + 1
        {
            continue;
        }

        visited[tunnel] = true;

        let mut new_state = state;
        new_state.released_pressure = released_pressure;
        new_state.available_flow_rate -= rooms[state.room_index].flow_rate;
        new_state.room_index = tunnel;
        new_state.time_left -= distances[state.room_index][tunnel] + 1;
        find_pressure1(rooms, distances, new_state, visited, max_pressure);

        visited[tunnel] = false;
    }
}

#[derive(Debug, Clone, Copy)]
struct State2 {
    released_pressure: u32,
    available_flow_rate: u32,
    room_index: (usize, usize),
    time_left: (u32, u32),
}

fn find_pressure2(
    rooms: &[Room],
    distances: &[Vec<u32>],
    state: State2,
    visited: &mut [bool],
    max_pressure: &mut u32,
) {
    // If all the available leftover pressure is not enough give up
    if state.released_pressure + state.available_flow_rate * state.time_left.0 <= *max_pressure {
        return;
    }

    if state.released_pressure + state.available_flow_rate * state.time_left.1 <= *max_pressure {
        return;
    }

    // Open the valve
    let released_pressure = state.released_pressure
        + state.time_left.0 * rooms[state.room_index.0].flow_rate
        + state.time_left.1 * rooms[state.room_index.1].flow_rate;
    *max_pressure = (*max_pressure).max(released_pressure);

    // Try to go in other rooms
    for neighbor0 in 0..distances.len() {
        if rooms[neighbor0].flow_rate == 0
            || visited[neighbor0]
            || state.time_left.0 < distances[state.room_index.0][neighbor0] + 1
        {
            continue;
        }

        visited[neighbor0] = true;

        for neighbor1 in 0..distances.len() {
            if rooms[neighbor1].flow_rate == 0
                || visited[neighbor1]
                || state.time_left.1 < distances[state.room_index.1][neighbor1] + 1
            {
                continue;
            }

            visited[neighbor1] = true;

            let mut new_state = state;
            new_state.released_pressure = released_pressure;
            new_state.available_flow_rate -=
                rooms[state.room_index.0].flow_rate + rooms[state.room_index.1].flow_rate;
            new_state.room_index.0 = neighbor0;
            new_state.time_left.0 -= distances[state.room_index.0][neighbor0] + 1;
            new_state.room_index.1 = neighbor1;
            new_state.time_left.1 -= distances[state.room_index.1][neighbor1] + 1;
            find_pressure2(rooms, distances, new_state, visited, max_pressure);

            visited[neighbor1] = false;
        }

        visited[neighbor0] = false;
    }

    for neighbor1 in 0..distances.len() {
        if rooms[neighbor1].flow_rate == 0
            || visited[neighbor1]
            || state.time_left.1 < distances[state.room_index.1][neighbor1] + 1
        {
            continue;
        }

        visited[neighbor1] = true;

        let mut new_state = state;
        new_state.released_pressure =
            state.released_pressure + state.time_left.1 * rooms[state.room_index.1].flow_rate;
        new_state.available_flow_rate -= rooms[state.room_index.1].flow_rate;
        new_state.room_index.1 = neighbor1;
        new_state.time_left.1 -= distances[state.room_index.1][neighbor1] + 1;
        find_pressure2(rooms, distances, new_state, visited, max_pressure);

        visited[neighbor1] = false;
    }
}

fn part1(input: &str) -> String {
    // Input
    let rooms = input.lines().map(Room::from_str).collect::<Vec<_>>();

    let distances = compute_distances(&rooms);
    let start = rooms
        .iter()
        .find_position(|room| room.name == "AA")
        .unwrap()
        .0;

    // Star 1
    let available_flow_rate = rooms.iter().map(|room| room.flow_rate).sum::<u32>();

    let mut visited = vec![false; rooms.len()];
    visited[start] = true;
    let mut pressure1 = 0;
    let state = State1 {
        released_pressure: 0,
        available_flow_rate,
        room_index: start,
        time_left: 30,
    };
    find_pressure1(&rooms, &distances, state, &mut visited, &mut pressure1);

    pressure1.to_string()
}

fn part2(input: &str) -> String {
    // Input
    let rooms = input.lines().map(Room::from_str).collect::<Vec<_>>();

    let distances = compute_distances(&rooms);
    let start = rooms
        .iter()
        .find_position(|room| room.name == "AA")
        .unwrap()
        .0;

    // Star 2
    let available_flow_rate = rooms.iter().map(|room| room.flow_rate).sum::<u32>();

    let mut visited = vec![false; rooms.len()];
    visited[start] = true;
    let mut pressure2 = 0;
    let state = State2 {
        released_pressure: 0,
        available_flow_rate,
        room_index: (start, start),
        time_left: (26, 26),
    };
    find_pressure2(&rooms, &distances, state, &mut visited, &mut pressure2);

    pressure2.to_string()
}

crate::run!();

// crate::test_example_aoc!(1651, 1707); //TODO

crate::test_aoc!(1896, 2576);
