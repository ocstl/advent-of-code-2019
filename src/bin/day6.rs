use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const COM: &str = "COM";
const INPUT_PATH: &str = "inputs/day6.txt";
const SANTA: &str = "SAN";
const YOU: &str = "YOU";

#[derive(Default)]
struct CenterOfMass<'a> {
    com: Option<&'a str>,
    satellites: Vec<&'a str>,
}

impl<'a> CenterOfMass<'a> {
    fn add_com(&mut self, com: &'a str) {
        self.com = Some(com);
    }

    fn add_satellite(&mut self, satellite: &'a str) {
        self.satellites.push(satellite);
    }
}

struct OrbitalMap<'a>(HashMap<&'a str, CenterOfMass<'a>>);

impl<'a> OrbitalMap<'a> {
    fn new() -> Self {
        OrbitalMap(HashMap::new())
    }

    fn count_satellites(&self, name: &str) -> usize {
        if let Some(com) = self.0.get(name) {
            com.satellites
                .iter()
                .map(|sat| 1 + self.count_satellites(sat))
                .sum()
        } else {
            0
        }
    }

    fn com_distance(&self, name: &str) -> HashMap<&str, usize> {
        std::iter::successors(self.0.get(name).and_then(|com| com.com), |com| {
            self.0.get(com).and_then(|com| com.com)
        })
        .enumerate()
        .map(|(distance, name)| (name, distance))
        .collect()
    }
}

impl<'a> From<&'a str> for OrbitalMap<'a> {
    fn from(input: &'a str) -> Self {
        input.lines().fold(OrbitalMap::new(), |mut acc, orbit| {
            let mut it = orbit.split(')');
            let com = it.next().expect("Missing parent.");
            let satellite = it.next().expect("Missing child.");

            acc.0.entry(com).or_default().add_satellite(satellite);
            acc.0.entry(satellite).or_default().add_com(com);
            acc
        })
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let orbits = OrbitalMap::from(input.as_str());

    part1(&orbits)?;
    part2(&orbits)?;
    Ok(())
}

/// What is the total number of direct and indirect orbits in your map data?
fn part1(orbits: &OrbitalMap) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(COM);

    let mut part1 = 0;
    while let Some(name) = stack.pop() {
        part1 += orbits.count_satellites(name);
        if let Some(sats) = orbits.0.get(name) {
            stack.extend(&sats.satellites);
        }
    }

    println!("Part 1: {}", part1);
    Ok(())
}

/// What is the minimum number of orbital transfers required to move from the
/// object YOU are orbiting to the object SAN is orbiting? (Between the objects
/// they are orbiting - not between YOU and SAN.)
fn part2(orbits: &OrbitalMap) -> Result<()> {
    let you = orbits.com_distance(YOU);
    let santa = orbits.com_distance(SANTA);

    let part2 = santa
        .into_iter()
        .filter_map(|(name, distance)| you.get(name).map(|d| d + distance))
        .min()
        .unwrap();

    println!("Part 2: {}", part2);
    Ok(())
}
