#![allow(dead_code, unused)]

use super::InputMode;

pub fn part_one() {
    let mut output: Vec<usize> = vec![];
    let machines = parse(InputMode::Normal);
    let output: usize = machines.iter().map(|m| fewest_buttons_to_light(m)).sum();
    println!("Output: {output}");
}

// I could not solve part two. The first idea come from
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// while this: rust implementation is coded by https://github.com/janek37/advent-of-code/blob/main/2025/day10.rs
pub fn part_two() {
    let mut output: Vec<usize> = vec![];
    let machines = parse(InputMode::Normal);
    let output: usize = machines.iter().map(|m| fewest_buttons_to_joltage(m)).sum();
    println!("Output: {output}");
}

fn fewest_buttons_to_light(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    for subgroup in subgroups(&binary_buttons) {
        if subgroup.iter().fold(0, |a, &b| a ^ b) == machine.lights {
            return subgroup.len()
        }
    }
    unreachable!()
}

fn fewest_buttons_to_joltage(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    let subset_xors: Vec<_> = subgroups(&binary_buttons)
        .iter()
        .map(|subset| (subset.to_owned(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, &machine.joltage).unwrap()
}

fn fewest_joltage_presses_recur(subset_xors: &[(Vec<u32>, u32)], joltages: &[i32]) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = get_binary_joltages(joltages);
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let new_joltages = get_new_joltages(joltages, &subset);
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count = fewest_joltage_presses_recur(
                    subset_xors, &new_joltages
                ).map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

fn get_new_joltages(joltages: &[i32], subset: &[u32]) -> Vec<i32> {
    let mut new_joltages = Vec::new();
    let mut mask = 1;
    for &joltage in joltages {
        new_joltages.push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
        mask <<= 1;
    }
    new_joltages
}

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

fn get_binary_joltages(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}

fn subgroups(group: &[u32]) -> Vec<Vec<u32>> {
    let mut groups: Vec<Vec<u32>> = Vec::new();
    for count in 0..=group.len() {
        groups.extend(get_all_combinations(group, count));
    }
    groups
}

fn get_all_combinations(group: &[u32], count: usize) -> Vec<Vec<u32>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        group[..group.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_all_combinations(&group[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<u32>>>()
            })
            .collect()
    }
}

struct Machine {
    lights: u32,
    joltage: Vec<i32>,
    buttons: Vec<Vec<u32>>,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let (light_str, rest) = input.split_once(']').unwrap();
        let (buttons_str, joltage_str) = rest.split_once('{').unwrap();

        let lights = light_str[1..]
            .trim()
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(n, _)| 1 << n)
            .sum();
        let mut buttons: Vec<Vec<u32>> = vec![];

        for button_str in buttons_str.trim().split(" ") {
            let mut button: Vec<u32> = vec![];
            let button_triggers = &button_str[1..button_str.len() - 1];
            for trigger in button_triggers.split(",") {
                let trigger = trigger.parse::<u32>().unwrap();
                button.push(trigger);
            }
            buttons.push(button);
        }
        let mut joltage: Vec<i32> = vec![];
        let joltage_str = &joltage_str[..joltage_str.len() - 1];
        for j in joltage_str.split(",") {
            joltage.push(j.parse().unwrap());
        }
        Self {
            lights,
            joltage,
            buttons,
        }
    }
}

fn parse(mode: InputMode) -> Vec<Machine> {
    let input: String;
    match mode {
        InputMode::Example => {
            input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
            .to_string();
        }
        InputMode::Normal => {
            input = super::load_input("input/input-day10");
        }
    }
    input.lines().map(|l| Machine::from_str(l)).collect()
}
