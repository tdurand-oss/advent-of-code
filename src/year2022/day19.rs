use std::collections::HashSet;

type Robots = [u16; 4];
type Resources = [u16; 4];

#[derive(Debug)]
struct Blueprint {
    robot_costs: [Resources; 4],
}

impl Blueprint {
    fn from_str(string: &str) -> Self {
        let regex = regex::Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let capture = regex.captures(string).unwrap();
        let get = |index| capture.get(index).unwrap().as_str().parse::<u16>().unwrap();

        Self {
            robot_costs: [
                [get(2), 0, 0, 0],
                [get(3), 0, 0, 0],
                [get(4), get(5), 0, 0],
                [get(6), 0, get(7), 0],
            ],
        }
    }
}

fn preprocess(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from_str).collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    robots: Robots,
    resources: Resources,
    time_left: u32,
}

fn evaluate(blueprint: &Blueprint, state: State, cache: &mut HashSet<State>, max_geodes: &mut u16) {
    if state.time_left == 0 || cache.contains(&state) {
        return;
    }
    cache.insert(state);

    // println!("{:?}", state);

    // Mine resources
    let mut new_state = state;
    for index in 0..4 {
        new_state.resources[index] += new_state.robots[index];
    }
    new_state.time_left -= 1;
    *max_geodes = (*max_geodes).max(new_state.resources[3]);

    // If not enough geodes can be collected assuming we build one new robot every turn, give up
    let time_left = state.time_left;

    let ore_no_new_robots = state.resources[1] as u32 + time_left * state.robots[1] as u32;
    let buildable_ore_robots = time_left;
    let additional_collectable_ore = buildable_ore_robots * (buildable_ore_robots - 1) / 2;
    let total_collectable_ore = ore_no_new_robots + additional_collectable_ore;

    let clay_no_new_robots = state.resources[1] as u32 + time_left * state.robots[1] as u32;
    let buildable_clay_robots =
        (total_collectable_ore / blueprint.robot_costs[1][0] as u32 + 1).min(time_left);
    let additional_collectable_clay = buildable_clay_robots * (buildable_clay_robots - 1) / 2;
    let total_collectable_clay = clay_no_new_robots + additional_collectable_clay;

    let obsidian_no_new_robots = state.resources[2] as u32 + time_left * state.robots[2] as u32;
    let buildable_obsidian_robots =
        (total_collectable_clay / blueprint.robot_costs[2][1] as u32 + 1).min(time_left);
    let additional_collectable_obsidian =
        buildable_obsidian_robots * (buildable_obsidian_robots - 1) / 2;
    let total_collectable_obsidian = obsidian_no_new_robots + additional_collectable_obsidian;

    let geodes_no_new_robots = new_state.resources[3] as u32 + time_left * state.robots[3] as u32;
    let buildable_geode_robots =
        (total_collectable_obsidian / blueprint.robot_costs[3][2] as u32 + 1).min(time_left);
    let additional_collectable_geodes = buildable_geode_robots * (buildable_geode_robots - 1) / 2;
    let total_collectable_geodes = geodes_no_new_robots + additional_collectable_geodes;

    if total_collectable_geodes <= *max_geodes as u32 {
        return;
    }

    // Try to build robots
    let mut enough_resources = 0;
    for r in 0..4 {
        let has_resources = state
            .resources
            .iter()
            .zip(blueprint.robot_costs[r].iter())
            .all(|(r1, r2)| r2 <= r1);
        if has_resources {
            enough_resources += 1;
            let mut new_state_robot = new_state;
            for index in 0..4 {
                new_state_robot.resources[index] -= blueprint.robot_costs[r][index];
            }
            new_state_robot.robots[r] += 1;
            evaluate(blueprint, new_state_robot, cache, max_geodes);
        }
    }

    // Don't build robots
    if enough_resources < 4 {
        evaluate(blueprint, new_state, cache, max_geodes);
    }
}

fn evaluate_blueprint(blueprint: &Blueprint, state: State) -> u16 {
    let mut max_geodes = 0;
    let mut cache = HashSet::new();
    evaluate(blueprint, state, &mut cache, &mut max_geodes);
    max_geodes
}

fn part1(input: &str) -> String {
    let blueprints = preprocess(input);

    let score = blueprints
        .iter()
        .enumerate()
        .map(|(index, blueprint)| {
            let state = State {
                robots: [1, 0, 0, 0],
                resources: [0, 0, 0, 0],
                time_left: 24,
            };
            let geodes = evaluate_blueprint(blueprint, state);
            (index + 1) * geodes as usize
        })
        .sum::<usize>();

    score.to_string()
}

fn part2(input: &str) -> String {
    let blueprints = preprocess(input);

    let score = blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let state = State {
                robots: [1, 0, 0, 0],
                resources: [0, 0, 0, 0],
                time_left: 32,
            };
            let geodes = evaluate_blueprint(blueprint, state);
            geodes as usize
        })
        .product::<usize>();

    score.to_string()
}

crate::run!();

crate::test_example_aoc!(33, 3472);

crate::test_aoc!(1565, 10672);
