type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day1.txt";

fn fuel_requirement(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}



fn part1(input: &str) -> Result<()> {
    let mut requirement = 0;
    for line in input.lines() {
        requirement += fuel_requirement(line.parse::<i32>()?);
    }

    println!("Part 1: {}", requirement);
    Ok(())
}

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