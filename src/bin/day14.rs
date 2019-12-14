use std::collections::HashMap;

const INPUT_PATH: &str = "inputs/day14.txt";
const FUEL: &str = "FUEL";
const ORE: &str = "ORE";
const ONE_TRILLION: u64 = 1_000_000_000_000;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    qty: u64,
    name: String,
}

impl std::ops::Mul<u64> for &Input {
    type Output = Input;

    fn mul(self, factor: u64) -> Self::Output {
        Input {
            qty: self.qty * factor,
            name: self.name.clone(),
        }
    }
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let mut iter = input.trim().split(' ');
        let qty = iter.next().unwrap().parse::<u64>().unwrap();
        let name = iter.next().unwrap().to_string();
        Input { qty, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reaction {
    output: Input,
    inputs: Vec<Input>,
}

impl From<&str> for Reaction {
    fn from(input: &str) -> Self {
        let mut iter = input.split(" => ");
        let inputs = iter.next().unwrap().split(',').map(Input::from).collect();
        let output = Input::from(iter.next().unwrap());
        Reaction { output, inputs }
    }
}

#[derive(Debug, Clone)]
struct ReactionList(HashMap<String, Reaction>);

impl ReactionList {
    fn steps_to_ore(&self, name: &str) -> u64 {
        if name == ORE {
            0
        } else {
            self.0
                .get(name)
                .unwrap()
                .inputs
                .iter()
                .map(|input| 1 + self.steps_to_ore(&input.name))
                .max()
                .unwrap()
        }
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    fn produce(&self, name: &str, amount: u64) -> impl Iterator<Item = Input> + '_ {
        let Reaction { output, inputs } = self.0.get(name).unwrap();
        let ratio = amount / output.qty + if amount % output.qty != 0 { 1 } else { 0 };

        inputs.iter().map(move |input| input * ratio)
    }
}

impl From<&str> for ReactionList {
    fn from(input: &str) -> Self {
        ReactionList(
            input
                .lines()
                .map(Reaction::from)
                .map(|reaction| (reaction.output.name.clone(), reaction))
                .collect(),
        )
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let list = ReactionList::from(input.as_str());
    part1(&list);
    part2(&list);
    Ok(())
}

/// By sorting in order of distance from "ORE", we can accumulate intermediate
/// products, thus accounting for "scraps".
fn produce(name: &str, amount: u64, list: &ReactionList) -> u64 {
    let mut accumulator = HashMap::new();
    accumulator.insert(name.to_string(), amount);

    let mut reactants: Vec<&String> = list.keys().collect();
    reactants.sort_by_key(|reactant| list.steps_to_ore(reactant));

    while let Some(output) = reactants.pop() {
        if let Some(&amount) = accumulator.get(output.as_str()) {
            for input in list.produce(output, amount) {
                *accumulator.entry(input.name).or_default() += input.qty;
            }
        }
    }

    *accumulator.get(ORE).unwrap_or(&0)
}

/// Given the list of reactions in your puzzle input, what is the minimum amount
/// of ORE required to produce exactly 1 FUEL?
fn part1(list: &ReactionList) {
    let part1 = produce(FUEL, 1, list);
    println!("Part 1: {}", part1);
}

/// Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
fn part2(list: &ReactionList) {
    let ore_amount = |fuel: u64| produce(FUEL, fuel, list);

    // Binary search.
    let mut min_fuel = ONE_TRILLION / ore_amount(1);
    let mut max_fuel = min_fuel;
    while ore_amount(max_fuel) < ONE_TRILLION {
        max_fuel *= 2;
    }

    while max_fuel > min_fuel + 1 {
        let mid = (min_fuel + max_fuel) / 2;
        if ore_amount(mid) > ONE_TRILLION {
            max_fuel = mid;
        } else {
            min_fuel = mid;
        }
    }

    println!("Part 2: {}", min_fuel);
}
