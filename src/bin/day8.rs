const INPUT_PATH: &str = "inputs/day8.txt";
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let pixels: Vec<Pixel> = input.trim_end().chars().map(Pixel::from).collect();
    part1(&pixels);
    part2(&pixels);
    Ok(())
}

/// To make sure the image wasn't corrupted during transmission, the Elves would
/// like you to find the layer that contains the fewest 0 digits. On that layer,
/// what is the number of 1 digits multiplied by the number of 2 digits?
fn part1(input: &[Pixel]) {
    let layer_size = HEIGHT * WIDTH;

    let part1 = input
        .chunks_exact(layer_size)
        .min_by_key(|layer| count_pixel(layer, Pixel::Black))
        .map(|layer| count_pixel(layer, Pixel::White) * count_pixel(layer, Pixel::Transparent))
        .unwrap_or_default();

    println!("Part 1: {}", part1);
}

/// Then, the full image can be found by determining the top visible pixel in
/// each position. What message is produced after decoding your image?
fn part2(input: &[Pixel]) {
    let layer_size = HEIGHT * WIDTH;

    let part2: Vec<Pixel> = (0..layer_size)
        .filter_map(|idx| {
            input
                .iter()
                .skip(idx)
                .step_by(layer_size)
                .find(|&&pixel| pixel != Pixel::Transparent)
                .copied()
        })
        .collect();

    for line in part2.chunks_exact(WIDTH) {
        println!(
            "{}",
            line.iter()
                .map(|&pixel| (pixel as u8 + b'0') as char)
                .collect::<String>()
        );
    }
}

#[allow(clippy::naive_bytecount)]
fn count_pixel(layer: &[Pixel], target: Pixel) -> usize {
    layer.iter().filter(|&&pixel| pixel == target).count()
}
