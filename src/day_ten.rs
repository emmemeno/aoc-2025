#![allow(dead_code, unused)]

use super::InputMode;

struct Machine {
    lights: Vec<u32>,
    joltage: Vec<i32>,
    buttons: Vec<Vec<u32>>,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let (light_str, rest) = input.split_once(']').unwrap();
        let (buttons_str, joltage_str) = rest.split_once('{').unwrap();

        let mut lights = vec![];
        let light_str = &light_str[1..];
        for light in light_str.trim().chars() {
            match light {
                '#' => lights.push(1),
                '.' => lights.push(0),
                _ => unreachable!(),
            }
        }
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

pub fn part_one() {
    let mut output: Vec<usize> = vec![];
    let max_deep_search = 11;
    let machines = parse(InputMode::Example);

    for (n, m) in machines.iter().enumerate() {
        //..
    }
}

