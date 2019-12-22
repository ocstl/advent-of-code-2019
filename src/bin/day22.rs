const INPUT_PATH: &str = "inputs/day22.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy)]
enum Technique {
    NewStack,
    Cut(i128),
    Increment(i128),
}

impl From<&str> for Technique {
    fn from(input: &str) -> Self {
        if input == "deal into new stack" {
            return Technique::NewStack;
        }

        let value = input
            .split_whitespace()
            .last()
            .map(|v| v.parse::<i128>().unwrap())
            .unwrap();
        if input.starts_with("cut") {
            Technique::Cut(value)
        } else {
            Technique::Increment(value)
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct ModLinear {
    a: i128,
    b: i128,
}

impl ModLinear {
    fn new(a: i128, b: i128) -> Self {
        ModLinear { a, b }
    }

    fn compose(self, other: Self, modulus: i128) -> Self {
        ModLinear::new(
            (self.a * other.a) % modulus,
            (self.a * other.b + self.b) % modulus,
        )
    }

    /// Repeat the linear transformation n times.
    fn repeat(mut self, mut n: u64, modulus: i128) -> Self {
        let mut result = ModLinear::new(1, 0);

        while n > 1 {
            if n & 1 == 1 {
                result = self.compose(result, modulus);
            }

            self = self.compose(self, modulus);
            n >>= 1;
        }

        self.compose(result, modulus)
    }

    fn apply(self, position: i128, modulus: i128) -> i128 {
        (self.a * position + self.b).rem_euclid(modulus)
    }
}

impl From<Technique> for ModLinear {
    fn from(technique: Technique) -> Self {
        match technique {
            Technique::NewStack => ModLinear::new(-1, -1),
            Technique::Cut(x) => ModLinear::new(1, -x),
            Technique::Increment(x) => ModLinear::new(x, 0),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let techniques: Vec<Technique> = input.lines().map(Technique::from).collect();
    part1(&techniques);
    part2(&techniques);

    Ok(())
}

/// After shuffling your factory order deck of 10007 cards, what is the position
/// of card 2019?
fn part1(techniques: &[Technique]) {
    const SIZE: i128 = 10_007;
    let total = techniques
        .iter()
        .fold(ModLinear::new(1, 0), |acc, &technique| {
            ModLinear::from(technique).compose(acc, SIZE)
        });

    let part1 = total.apply(2019, SIZE);
    println!("Part 1: {}", part1);
}

/// When you get back, you discover that the 3D printers have combined their
/// power to create for you a single, giant, brand new, factory order deck of
/// 119315717514047 space cards.
///
/// Finally, a deck of cards worthy of shuffling!
///
/// You decide to apply your complete shuffle process (your puzzle input) to the
/// deck 101741582076661 times in a row.
///
/// You'll need to be careful, though - one wrong move with this many cards and
/// you might overflow your entire ship!
///
/// After shuffling your new, giant, factory order deck that many times, what
/// number is on the card that ends up in position 2020?
fn part2(techniques: &[Technique]) {
    const SIZE: i128 = 119_315_717_514_047;
    const SHUFFLES: u64 = 101_741_582_076_661;
    let shuffle = techniques
        .iter()
        .fold(ModLinear::new(1, 0), |acc, &technique| {
            ModLinear::from(technique).compose(acc, SIZE)
        });

    // This is the complete shuffle. We need to invert it to get the required
    // initial position:
    // ax + b = y (mod m)
    // x = a^(-1) (y - b) (mod m)
    let complete = shuffle.repeat(SHUFFLES, SIZE);

    // Since the size of the deck is prime, the multiplicative inverse of a is
    // given by:
    // a^(-1) = a^(m-2) (mod m)
    let inverse_a = modular_inverse(complete.a, SIZE);
    let part2 = (inverse_a * (2020 - complete.b)).rem_euclid(SIZE);

    println!("Part 2: {}", part2);
}

fn modular_inverse(mut a: i128, modulus: i128) -> i128 {
    let mut n = modulus - 2;
    let mut result = 1;

    while n > 1 {
        if n & 1 == 1 {
            result = (result * a) % modulus;
        }

        a = (a * a) % modulus;
        n >>= 1;
    }

    (result * a) % modulus
}
