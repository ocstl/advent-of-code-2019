use num::integer::Integer;

const INPUT_PATH: &str = "inputs/day12.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates(i64, i64, i64);

impl Coordinates {
    fn signum(self) -> Self {
        Coordinates(self.0.signum(), self.1.signum(), self.2.signum())
    }

    fn energy(self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::ops::Add<Coordinates> for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coordinates(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub<Coordinates> for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coordinates(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    position: Coordinates,
    velocity: Coordinates,
}

impl Moon {
    fn new(position: Coordinates) -> Self {
        Moon {
            position,
            velocity: Coordinates(0, 0, 0),
        }
    }

    fn apply_gravity(&mut self, moons: &[Moon]) -> &mut Self {
        let mut dv = Coordinates::default();
        for moon in moons {
            dv = dv + (moon.position - self.position).signum();
        }
        self.velocity = self.velocity + dv;
        self
    }

    fn apply_velocity(&mut self) -> &mut Self {
        self.position = self.position + self.velocity;
        self
    }

    fn potential_energy(&self) -> i64 {
        self.position.energy()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.energy()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn dimensionality_reduction(&mut self, dim: usize) {
        match dim {
            0 => {
                self.position.1 = 0;
                self.position.2 = 0
            }
            1 => {
                self.position.0 = 0;
                self.position.2 = 0
            }
            2 => {
                self.position.0 = 0;
                self.position.1 = 0
            }
            _ => unreachable!(),
        }
    }
}

// This should properly be a TryFrom, but we'll assume proper formatting.
impl From<&str> for Moon {
    fn from(input: &str) -> Self {
        let mut coordinates = input.split(',').map(|s| {
            s.trim_matches(|c: char| !(c.is_numeric() || c == '-'))
                .parse::<i64>()
                .unwrap()
        });
        Moon::new(Coordinates(
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Simulator {
    moons: Vec<Moon>,
}

impl Simulator {
    fn new(moons: &[Moon]) -> Self {
        Simulator {
            moons: moons.to_vec(),
        }
    }
}

impl<'a> Iterator for Simulator {
    type Item = Vec<Moon>;

    fn next(&mut self) -> Option<Self::Item> {
        let moons = self.moons.clone();
        for moon in self.moons.iter_mut() {
            moon.apply_gravity(&moons).apply_velocity();
        }

        Some(moons)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let moons: Vec<Moon> = input.lines().map(Moon::from).collect();
    part1(&moons);
    part2(&moons);
    Ok(())
}

/// What is the total energy in the system after simulating the moons given in
/// your scan for 1000 steps?
fn part1(moons: &[Moon]) {
    let part1: i64 = Simulator::new(moons)
        .nth(1000)
        .map(|moons| moons.iter().map(Moon::total_energy).sum())
        .unwrap();
    println!("Part 1: {}", part1);
}

/// How many steps does it take to reach the first state that exactly matches a
/// previous state?
fn part2(moons: &[Moon]) {
    // Find the first recurring state for each dimension. This will always be
    // the initial state, so we don't have to care about any offset.
    let mut part2: usize = 1;

    for dim in 0..3 {
        let mut new_moons = moons.to_vec();
        for moon in new_moons.iter_mut() {
            moon.dimensionality_reduction(dim);
        }

        let mut simulator = Simulator::new(&new_moons);
        let first_position = simulator.next().unwrap();
        let steps = simulator.position(|moons| moons == first_position).unwrap() + 1;
        part2 = part2.lcm(&steps);
    }

    println!("Part 2: {}", part2);
}
