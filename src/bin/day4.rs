use radixal::IntoDigits;
use std::collections::HashMap;

const INPUT: (u32, u32) = (240_298, 784_956);

fn main() {
    part1(INPUT);
    part2(INPUT);
}

/// However, they do remember a few key facts about the password:
///
/// - It is a six-digit number.
/// - The value is within the range given in your puzzle input.
/// - Two adjacent digits are the same (like 22 in 122345).
/// - Going from left to right, the digits never decrease; they only ever
/// increase or stay the same (like 111123 or 135679).
fn part1(input: (u32, u32)) {
    let part1 = (input.0..=input.1)
        .filter(|&password| {
            let digits: Vec<u32> = password.into_decimal_digits().collect();
            ascending_digits(&digits) && digits_count(&digits).values().any(|&count| count >= 2)
        })
        .count();

    println!("Part 1: {}", part1);
}

/// An Elf just remembered one more important detail: the two adjacent matching
/// digits are not part of a larger group of matching digits.
fn part2(input: (u32, u32)) {
    let part2 = (input.0..=input.1)
        .filter(|&password| {
            let digits: Vec<u32> = password.into_decimal_digits().collect();
            ascending_digits(&digits) && digits_count(&digits).values().any(|&count| count == 2)
        })
        .count();

    println!("Part 2: {}", part2);
}

fn ascending_digits(digits: &[u32]) -> bool {
    digits.windows(2).all(|pair| pair[0] <= pair[1])
}

fn digits_count(digits: &[u32]) -> HashMap<u32, u32> {
    digits.iter().fold(HashMap::new(), |mut acc, &digit| {
        *acc.entry(digit).or_default() += 1;
        acc
    })
}
