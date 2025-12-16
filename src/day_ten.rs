#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use super::InputMode;

type ButtonId = usize;
type Button = Vec<Trigger>;
type Buttons = Vec<Button>;
type Lights = Vec<bool>;
type Joltage = Vec<u16>;
type Score = i32;
type Trigger = u8;

struct Machine {
    lights: Lights,
    joltage: Joltage,
    buttons: Buttons,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let (light_str, rest) = input.split_once(']').unwrap();
        let (buttons_str, joltage_str) = rest.split_once('{').unwrap();

        let mut lights: Lights = vec![];
        let light_str = &light_str[1..];
        for light in light_str.trim().chars() {
            match light {
                '#' => lights.push(true),
                '.' => lights.push(false),
                _ => unreachable!(),
            }
        }
        let mut buttons: Vec<Button> = vec![];
        let mut joltage_triggers: HashMap<Trigger, u8> = HashMap::new();
        for button_str in buttons_str.trim().split(" ") {
            let mut button: Button = vec![];
            let button_triggers = &button_str[1..button_str.len() - 1];
            for trigger in button_triggers.split(",") {
                let trigger = trigger.parse::<Trigger>().unwrap();
                button.push(trigger);
                if let Some(t) = joltage_triggers.get_mut(&trigger) {
                    *t = *t + 1;
                } else {
                    joltage_triggers.insert(trigger, 1);
                }
            }
            buttons.push(button);
        }
        let mut joltage: Vec<u16> = vec![];
        let joltage_str = &joltage_str[..joltage_str.len() - 1];
        for j in joltage_str.split(",") {
            joltage.push(j.parse::<u16>().unwrap());
        }
        Self {
            lights,
            joltage,
            buttons,
        }
    }

    // ##############################
    // BUTTONS TO LIGHTS
    // ##############################

    fn most_beneficial_buttons_for_light(&self, target: &[bool], exclude: &[ButtonId]) -> Vec<(ButtonId, Score)> {
        // println!("Most beneficial buttons for target: {target:?}");
        let light_to_switch: HashSet<u8> = target
            .iter()
            .enumerate()
            .filter(|(_, l)| **l)
            .map(|(n, _)| n as u8)
            .collect();
        let mut button_ladder: Vec<(ButtonId, Score)> = vec![];
        for (n, button) in self.buttons.iter().enumerate().filter(|(n, _)| !exclude.contains(n)) {
            // if button trigger off the light, its positive effect (add to score)
            let score = button
                .iter()
                .filter(|&trigger| light_to_switch.contains(trigger))
                .count() as i32;
            button_ladder.push((n, score));
        }
        button_ladder.sort_by_key(|&(_, count)| -(count as i16));
        button_ladder
    }

    //return a new modified light target configuration
    fn apply_button_to_lights(&self, button: ButtonId, target: &[bool]) -> Lights {
        let mut new_target = target.to_vec();
        for trigger in &self.buttons[button] {
            let current_status = target[*trigger as usize];
            new_target[*trigger as usize] = !current_status;
        }
        new_target
    }

    fn buttons_to_light_off(
        &self,
        tmp_target: &[bool],
        memo: &mut HashMap<Lights, Vec<ButtonId>>,
    ) {
        // get the actual sequence lenght
        let mut buttons_pressed = match memo.get(tmp_target) {
            Some(bb) => bb.clone(),
            None => vec![],
        };
        let mbb = self.most_beneficial_buttons_for_light(tmp_target, &buttons_pressed);
        println!("Buttons Pressed: {buttons_pressed:?}");
        for (button_id, _) in mbb.into_iter() {

            let new_target = self.apply_button_to_lights(button_id, tmp_target);
            buttons_pressed.push(button_id);
            // check if cache contains a smaller path
            if let Some(cache) =  memo.get(&new_target) {
                if cache.len() <= buttons_pressed.len() {
                    continue;
                }
            }
            memo.insert(new_target.clone(), buttons_pressed.clone());
            // all lights are off, breaking the branch
            if new_target.iter().all(|l| !l) {
                break;
            // deep down the rabitt hole
            } else {
                self.buttons_to_light_off(&new_target, memo);
            }
        }
    }

    // fn button_light_combo_only_length(
    //     &self,
    //     actual_combo_length: usize,
    //     actual_target: &[bool],
    //     ladder: &mut Vec<Vec<Button>>,
    //     shortest_sequence: &mut usize,
    // ) -> usize {
    //     let mbb = self.most_beneficial_buttons_for_light(&actual_target);
    //     for (_n, (button_id, _)) in mbb.into_iter().enumerate() {
    //         let new_target = self.apply_button_to_lights(button_id, &actual_target);
    //         // println!("New target: {new_target:?}");
    //         if all_light_are_off(&new_target) {
    //             if actual_combo_length + 1 < *shortest_sequence {
    //                 *shortest_sequence = actual_combo_length + 1;
    //             }
    //             // println!("Found a new top score ({}) combination for {:?}", shortest_sequence, self.target_lights);
    //             // println!("Combination: {new_combo:?}");
    //             break;
    //         } else if actual_combo_length + 2 < *shortest_sequence && *shortest_sequence != 1 {
    //             self.button_light_combo_only_length(
    //                 actual_combo_length + 1,
    //                 &new_target,
    //                 ladder,
    //                 shortest_sequence,
    //             );
    //         }
    //     }
    //     *shortest_sequence
    // }
    //
    // fn button_light_combo_verbose(
    //     &self,
    //     actual_combo: &Vec<Button>,
    //     actual_target: &[bool],
    //     ladder: &mut Vec<Vec<Button>>,
    //     shortest_sequence: &mut usize,
    // ) -> Vec<Vec<Button>> {
    //     let mbb = self.most_beneficial_buttons_for_light(&actual_target);
    //     for (_n, (button_id, _)) in mbb.into_iter().enumerate() {
    //         let mut new_combo = actual_combo.clone();
    //         new_combo.push(self.buttons[button_id].clone());
    //         let new_target = self.apply_button_to_lights(button_id, &actual_target);
    //         // println!("New target: {new_target:?}");
    //         if all_light_are_off(&new_target) {
    //             if new_combo.len() < *shortest_sequence {
    //                 *shortest_sequence = new_combo.len();
    //             }
    //             ladder.push(new_combo.clone());
    //             println!(
    //                 "Found a new top score ({}) combination for {:?}",
    //                 new_combo.len(),
    //                 self.target_lights
    //             );
    //             // println!("Combination: {new_combo:?}");
    //             break;
    //         } else if new_combo.len() + 1 < *shortest_sequence {
    //             self.button_light_combo_verbose(&new_combo, &new_target, ladder, shortest_sequence);
    //         }
    //     }
    //     ladder.clone()
    // }

    // ##############################
    // BUTTONS TO JOLTAGE
    // ##############################

    // fn most_beneficial_buttons_for_joltage(&self, target: &[u16]) -> Vec<(ButtonId, Score)> {
    //     let min_in_target = target.iter().enumerate().min().unwrap();
    //     let max_in_target = target.iter().enumerate().max().unwrap();
    //     let mut button_ladder: Vec<(ButtonId, Score)> = vec![];
    //     // if target joltage is zero, exclude the button
    //     for (n, button) in self.buttons.iter().enumerate() {
    //         if button.iter().any(|pos| target[*pos as usize] == 0) {
    //             // println!("Filtering {button:?} for {target:?}");
    //             continue;
    //         }
    //         let mut score = 0i32;
    //         score += button.len() as i32;
    //         if button.contains(&(min_in_target.0 as u8)) {
    //             score -= self.joltage_triggers[&(min_in_target.0 as u8)] as i32;
    //         }
    //         if button.contains(&(max_in_target.0 as u8)) {
    //             score += 1;
    //         }
    //         button_ladder.push((n, score));
    //     }
    //     button_ladder.sort_by_key(|&(_, score)| -(score as i16));
    //     button_ladder
    // }
    //
    // //return a new modified light target configuration
    // fn apply_button_to_joltage(&self, button: ButtonId, target: &[u16]) -> Vec<u16> {
    //     let mut new_target = target.to_vec();
    //     for trigger in &self.buttons[button] {
    //         if new_target[*trigger as usize] == 0 {
    //             println!("trying to modify target: {target:?}");
    //             unreachable!();
    //         }
    //         new_target[*trigger as usize] -= 1;
    //     }
    //     new_target
    // }
    // fn button_joltage_combo_only_length(
    //     &self,
    //     actual_combo_length: usize,
    //     actual_target: &[u16],
    //     shortest_sequence: &mut usize,
    //     max_deep_search: usize,
    // ) -> usize {
    //     let mbb = self.most_beneficial_buttons_for_joltage(&actual_target);
    //     for (_n, (button_id, _)) in mbb.into_iter().enumerate() {
    //         let new_target = self.apply_button_to_joltage(button_id, &actual_target);
    //         // println!("New target: {new_target:?}");
    //         if all_joltage_are_zero(&new_target) {
    //             if actual_combo_length + 1 < *shortest_sequence {
    //                 *shortest_sequence = actual_combo_length + 1;
    //             }
    //             println!(
    //                 "Found a new top score ({}) combination for {:?}",
    //                 actual_combo_length, self.joltage
    //             );
    //             break;
    //         } else if actual_combo_length + 2 < *shortest_sequence {
    //             self.button_joltage_combo_only_length(
    //                 actual_combo_length + 1,
    //                 &new_target,
    //                 shortest_sequence,
    //                 max_deep_search,
    //             );
    //         }
    //     }
    //     *shortest_sequence
    // }

    // fn button_joltage_combo_verbose(&self, actual_combo: &Vec<Button>, actual_target: &[u16], ladder: &mut Vec<Vec<Button>>, shortest_sequence: &mut usize) -> Vec<Vec<Button>> {
    //     let mbb = self.most_beneficial_buttons_for_joltage(&actual_target);
    //     for (_n, (button_id, _)) in mbb.into_iter().enumerate() {
    //         let mut new_combo = actual_combo.clone();
    //         new_combo.push(self.buttons[button_id].clone());
    //         let new_target = self.apply_button_to_joltage(button_id, &actual_target);
    //         // println!("New target: {new_target:?}");
    //         if all_joltage_are_zero(&new_target) {
    //             if new_combo.len() < *shortest_sequence {
    //                 *shortest_sequence = new_combo.len();
    //             }
    //             ladder.push(new_combo.clone());
    //             println!("Found a new top score ({}) combination for {:?}", new_combo.len(), self.target_lights);
    //             println!("Combination: {new_combo:?}");
    //             break;
    //         } else if new_combo.len() + 1< *shortest_sequence {
    //             self.button_joltage_combo_verbose(&new_combo, &new_target, ladder, shortest_sequence);
    //         }
    //     }
    //     ladder.clone()
    // }
}

fn all_light_are_off(lights: &[bool]) -> bool {
    lights.iter().all(|l| !l)
}

fn all_joltage_are_zero(joltage: &[u16]) -> bool {
    joltage.iter().all(|j| *j == 0)
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
        // OLD FUNCTION
        // let single_output = m.button_light_combo_only_length(
        //     0,
        //     &m.lights,
        //     &mut vec![],
        //     &mut max_deep_search.clone(),
        // );
        // NEW FUNCTION
        let mut cache = HashMap::new();
        m.buttons_to_light_off(&m.lights, &mut cache);
        let single_output = cache.get(&m.lights).unwrap();
        println!("Machine #{n}: {}", single_output.len());
        output.push(single_output.len());
    }
    println!("Output: {}", output.iter().sum::<usize>());
}
//
// pub fn part_two() {
//     let output: Vec<usize> = vec![];
//     let max_deep_search = 10;
//     let machines = parse(InputMode::Example);
//
//     let test = 0;
//     let best_button =
//         machines[test].most_beneficial_buttons_for_joltage(&machines[test].target_joltage);
//     println!("Best button for Joltage, machine #{test}:");
//     println!(
//         "{:?} - {:?}",
//         machines[test].buttons, machines[test].target_joltage
//     );
//     for (b_id, score) in best_button {
//         println!(
//             "Button: {:?} - score: {}",
//             machines[test].buttons[b_id], score
//         );
//     }
//     let single_output = machines[test].button_joltage_combo_only_length(
//         0,
//         &machines[test].target_joltage,
//         &mut 1000,
//         max_deep_search,
//     );
// for (n, m) in machines.iter().enumerate() {
//     let single_output = m.button_joltage_combo_only_length(0, &m.target_joltage, &mut 1000, max_deep_search);
//     println!("Machine #{n}: {single_output}");
//     output.push(single_output);
// }
// println!("Output: {}", output.iter().sum::<usize>());
// }
