use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day1.txt";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT_PATH)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

/// Fuel required to launch a given module is based on its mass. Specifically,
/// to find the fuel required for a module, take its mass, divide by three,
/// round down, and subtract 2.
///
/// What is the sum of the fuel requirements for all of the modules on your
/// spacecraft?
fn part1(input: &str) -> Result<()> {
    let mut requirement = 0;
    for line in input.lines() {
        requirement += line.parse::<i32>()? / 3 - 2;
    }

    println!("Part 1: {}", requirement);
    Ok(())
}

/// Fuel itself requires fuel just like a module - take its mass, divide by
/// three, round down, and subtract 2. However, that fuel also requires fuel,
/// and that fuel requires fuel, and so on. Any mass that would require negative
/// fuel should instead be treated as if it requires zero fuel; the remaining
/// mass, if any, is instead handled by wishing really hard, which has no mass
/// and is outside the scope of this calculation.
fn part2(input: &str) -> Result<()> {
    let mut requirement = 0;
    for line in input.lines() {
        let mut temp = (line.parse::<i32>()? / 3 - 2).max(0);
        while temp != 0 {
            requirement += temp;
            temp = (temp / 3 - 2).max(0);
        }
    }

    println!("Part 2: {}", requirement);
    Ok(())
}