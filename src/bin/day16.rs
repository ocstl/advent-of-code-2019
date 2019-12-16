use std::iter::repeat;

const INPUT_PATH: &str = "inputs/day16.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Signal(Vec<i32>);

impl Iterator for Signal {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let signal = (0..self.0.len())
            .map(|idx| {
                self.0
                    .iter()
                    .zip(fft_row(idx + 1))
                    .map(|(digit, factor)| digit * factor)
                    .sum::<i32>()
                    .abs()
                    % 10
            })
            .collect();

        self.0 = signal;
        Some(self.0.clone())
    }
}

fn fft_row(idx: usize) -> impl Iterator<Item = i32> {
    repeat(0)
        .take(idx)
        .chain(repeat(1).take(idx))
        .chain(repeat(0).take(idx))
        .chain(repeat(-1).take(idx))
        .cycle()
        .skip(1)
}

/// The FFT (Flawed Frequency Transmission) algorithm is an upper unitriangular
/// matrix. We could use the `nalgebra` crate, but this solution is fast enough.
fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let digits: Vec<i32> = input.trim().bytes().map(|b| (b - b'0') as i32).collect();
    part1(&digits);
    part2(&digits);

    Ok(())
}

/// After 100 phases of FFT, what are the first eight digits in the final output
/// list?
fn part1(digits: &[i32]) {
    let part1 = Signal(digits.to_vec())
        .nth(99)
        .unwrap()
        .into_iter()
        .take(8)
        .fold(0, |acc, digit| acc * 10 + digit);

    println!("Part 1: {}", part1);
}

/// After repeating your input signal 10000 times and running 100 phases of FFT,
/// what is the eight-digit message embedded in the final output list?
fn part2(digits: &[i32]) {
    let offset = digits
        .iter()
        .take(7)
        .fold(0, |acc, &digit| acc * 10 + digit) as usize;

    // This ensures that we are only dealing with the second half, which is
    // nothing but '1's.
    assert!(2 * offset > digits.len() * 10_000);

    // We can ignore the digits before the offset. Starting at the end to
    // accumulate.
    let mut signal: Vec<i32> = repeat(digits.iter())
        .take(10_000)
        .flatten()
        .skip(offset)
        .copied()
        .collect();
    signal.reverse();

    for _ in 0..100 {
        signal = signal
            .into_iter()
            .scan(0, |acc, digit| {
                *acc = (*acc + digit) % 10;
                Some(*acc)
            })
            .collect();
    }

    let part2 = signal
        .into_iter()
        .rev()
        .take(8)
        .fold(0, |acc, digit| acc * 10 + digit);
    println!("Part 2: {}", part2);
}
