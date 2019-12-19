use advent_of_code_2019::intcode::{read_program, Computer, Program};

const INPUT_PATH: &str = "inputs/day19.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;

    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// How many points are affected by the tractor beam in the 50x50 area closest
/// to the emitter?
fn part1(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();

    for x in 0..50 {
        for y in 0..50 {
            tx.send(x)?;
            tx.send(y)?;
            computer.load_program(program.clone()).execute()?;
        }
    }

    let part1: isize = rx.try_iter().sum();
    println!("Part 1: {}", part1);

    Ok(())
}

/// Find the 100x100 square closest to the emitter that fits entirely within the
/// tractor beam; within that square, find the point closest to the emitter.
/// What value do you get if you take that point's X coordinate, multiply it by
/// 10000, then add the point's Y coordinate?
fn part2(program: Program) -> Result<()> {
    // Some eyeballing shows that x <= y, possible strictly greater past 0, but
    // the difference is minimal. Now, to fit a 100x100 ship, if the upper-right
    // point is (x, y), the lower-left point is (x - 99, y + 99). So, if both
    // are within the tractor beam, the top-left point will be (x - 99, y).
    for x in 100.. {
        let (mut computer, tx, rx) = Computer::new();
        let first_y = (0..)
            .find(|&y| {
                tx.send(x).unwrap();
                tx.send(y).unwrap();
                computer.load_program(program.clone()).execute().unwrap();
                rx.recv().unwrap() == 1
            })
            .unwrap();

        tx.send(x - 99)?;
        tx.send(first_y + 99)?;
        computer.load_program(program.clone()).execute()?;
        if rx.recv()? == 1 {
            let part2 = (x - 99) * 10_000 + first_y;
            println!("Part 2: {}", part2);
            break;
        }
    }

    Ok(())
}
