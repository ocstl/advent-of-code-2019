type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day1.txt";

/// Fuel required to launch a given module is based on its mass. Specifically,
/// to find the fuel required for a module, take its mass, divide by three,
/// round down, and subtract 2.
fn fuel_requirement(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

/// What is the sum of the fuel requirements for all of the modules on your
/// spacecraft?
fn part1(input: &str) -> Result<()> {
    let mut requirement = 0;
    for line in input.lines() {
        requirement += fuel_requirement(line.parse::<i32>()?);
    }

    println!("Part 1: {}", requirement);
    Ok(())
}

/// What is the sum of the fuel requirements for all of the modules on your
/// spacecraft when also taking into account the mass of the added fuel?
/// (Calculate the fuel requirements for each module separately, then add them
/// all up at the end.)
fn part2(input: &str) -> Result<()> {
    let mut requirement = 0;
    for line in input.lines() {
        let mut temp = fuel_requirement(line.parse::<i32>()?).max(0);
        while temp != 0 {
            requirement += temp;
            temp = fuel_requirement(temp).max(0);
        }
    }

    println!("Part 2: {}", requirement);
    Ok(())
}