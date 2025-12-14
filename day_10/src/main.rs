use aoctools::{parse_usize, read_file};
use good_lp::{default_solver, Solution, SolverModel, Variable};
use good_lp::{variable, ProblemVariables};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let machines = read_input("input.txt");
    let mut sum_part_1 = 0;
    for m in machines.clone() {
        sum_part_1 += find_the_fewest_button_presses(m);
    }
    println!("Part 1: {}", sum_part_1);
    let elapsed = now.elapsed();
    println!("Elapsed Part 1: {:.2?}", elapsed);

    let now = Instant::now();
    let mut sum_part_2: f64 = 0_f64;
    for m in machines.clone() {
        sum_part_2 += solve_machine_part_2(m);
    }
    println!("Part 2: {}", sum_part_2);
    let elapsed = now.elapsed();
    println!("Elapsed Part 2: {:.2?}", elapsed);
}

fn solve_machine_part_2(machine: Machine) -> f64 {
    let num_of_equations = machine.joltage.requirements.len();
    let num_of_variables = machine.buttons.buttons.len();
    let mut vars = ProblemVariables::new();
    let mut xs: Vec<Variable> = Vec::new();
    for i in 0..num_of_variables {
        xs.push(vars.add(variable().min(0).integer()));
    }
    let mut problem = vars.minimise(xs.iter().sum::<good_lp::Expression>()).using(default_solver);
    for index_of_right_side in 0..num_of_equations {
        let right_side = machine.joltage.requirements[index_of_right_side] as u32;
        let mut indices = Vec::<usize>::new();
        for (b_idx, b) in machine.buttons.buttons.clone().into_iter().enumerate() {
            if b.button.contains(&index_of_right_side) {
                indices.push(b_idx);
            }
        }
        let contraint = indices.iter().map(|i| xs[i.clone()].clone()).sum::<good_lp::Expression>().eq(right_side);
        problem.add_constraint(contraint);
    }
    let solution = problem.solve().unwrap();
    solution.eval(xs.iter().sum::<good_lp::Expression>())
}

fn find_the_fewest_button_presses(machine: Machine) -> usize {
    let mut num_of_presses = 0;
    let mut current_states = Vec::new();
    current_states.push(("".to_string(), initial(&machine.indicator)));
    loop {
        num_of_presses += 1;
        current_states = press_each_button_separately(current_states.clone(), &machine.buttons); //, &mut HashMap::<Vec<bool>, Vec::<(String, Indicator)>>::new());
        let result = check_states(current_states.iter().map(|x| x.clone().1).collect(), &machine.indicator);
        if result {
            break;
        }
    }
    num_of_presses
}

fn check_states(states: Vec<Indicator>, goal: &Indicator) -> bool {
    for state in states {
        if state.lights == goal.lights {
            return true;
        }
    }
    false
}

fn initial(goal: &Indicator) -> Indicator {
    Indicator { lights: vec![false; goal.lights.len()] }
}

fn press_each_button_separately(states: Vec<(String, Indicator)>, buttons: &Buttons) -> Vec<(String, Indicator)> {
    let mut cache_for_one_state = HashMap::<(Indicator, &Buttons), HashMap<Vec<usize>, Indicator>>::new();
    let mut new_states = Vec::<(String, Indicator)>::with_capacity(states.len() * buttons.buttons.len());
    for s in states.iter() {
        let mut next_states: HashMap<Vec<usize>, Indicator> = HashMap::new();
        if cache_for_one_state.contains_key(&(s.1.clone(), buttons)) {
            next_states = cache_for_one_state.get(&(s.1.clone(), buttons)).unwrap().clone();
        } else {
            next_states = generate_next_states(s.1.clone(), buttons);
            cache_for_one_state.insert((s.1.clone(), buttons), next_states.clone());
        }
        for (b, ns) in next_states.into_iter() {
            new_states.push( (format!("{},{:?}", s.0, b), ns));
        }
    }
    new_states
}

fn generate_next_states(state: Indicator, buttons: &Buttons) -> HashMap<Vec<usize>, Indicator> {
    let mut res = HashMap::<Vec<usize>, Indicator>::new();
    for b in buttons.buttons.iter() {
        let ns = apply_button(state.lights.clone(), b.button.clone());
        res.insert(b.button.clone(), Indicator { lights: ns });
    }
    res
}

fn apply_button(state: Vec<bool>, button: Vec<usize>) -> Vec<bool> {
    let mut new_state = state.clone();
    for b in button.iter() {
        new_state = toggle_light(*b, new_state);
    }
    new_state
}

fn toggle_light(idx: usize, lights: Vec<bool>) -> Vec<bool> {
    fn toggle(light: bool) -> bool {
        light ^ true
    }

    let mut output = lights.clone();
    let elem =output.remove(idx);
    output.insert(idx, toggle(elem));
    output
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Indicator {
    lights: Vec<bool>,
}
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Button {
    button: Vec<usize>,
}
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Buttons {
    buttons: Vec<Button>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Joltage {
    requirements: Vec<usize>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Machine {
    indicator: Indicator,
    buttons: Buttons,
    joltage: Joltage,
}

fn read_input(file_name: &str) -> Vec<Machine> {
    let mut machine = Vec::<Machine>::new();
    for line in read_file(file_name) {
        let line = line.unwrap();
        let split1 = line.split("] ").collect::<Vec<&str>>();
        let indicator = Indicator { lights: split1[0].chars().skip(1).map(|c| c == '#').collect::<Vec<bool>>() };

        let split2 = split1[1].split(" {").collect::<Vec<&str>>();
        let buttons = Buttons { buttons: split2[0].split(" ").map(|x| Button { button: x[1..x.len() - 1].split(",").map(|y| parse_usize(y)).collect::<Vec<usize>>() }).collect::<Vec<Button>>() };

        let joltage = Joltage { requirements: split2[1][0..split2[1].len() - 1].split(",").map(|x| parse_usize(x)).collect::<Vec<usize>>() };

        machine.push(Machine {
            indicator,
            buttons,
            joltage
        })
    }
    machine
}
