use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program};

const INPUT_PATH: &str = "inputs/day23.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;

    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// Boot up all 50 computers and attach them to your network. What is the Y
/// value of the first packet sent to address 255?
fn part1(program: Program) -> Result<()> {
    const NBR_COMPUTERS: usize = 50;

    let mut senders = Vec::new();
    let mut receivers = Vec::new();

    for address in 0..NBR_COMPUTERS {
        let (mut computer, tx, rx) = Computer::new();
        tx.send(address as isize)?;

        senders.push(tx);
        receivers.push(rx);

        computer.load_program(program.clone());

        std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
            let mut computer = computer;
            computer.execute()?;
            Ok(())
        });
    }

    let mut idx = 0;

    loop {
        let receiver = &receivers[idx];
        match receiver.try_recv() {
            Ok(address) => {
                let x = receiver.recv()?;
                let y = receiver.recv()?;
                if address == 255 {
                    println!("Part 1: {}", y);
                    break;
                } else {
                    let sender = &senders[address as usize];
                    sender.send(x)?;
                    sender.send(y)?;
                }
            }
            _ => {
                senders[idx].send(-1)?;
            }
        }

        idx = (idx + 1) % NBR_COMPUTERS;
    }

    Ok(())
}

/// Monitor packets released to the computer at address 0 by the NAT. What is
/// the first Y value delivered by the NAT to the computer at address 0 twice in
/// a row?
fn part2(program: Program) -> Result<()> {
    const NBR_COMPUTERS: usize = 50;

    let mut senders = Vec::new();
    let mut receivers = Vec::new();

    for address in 0..NBR_COMPUTERS {
        let (mut computer, tx, rx) = Computer::new();
        tx.send(address as isize)?;

        senders.push(tx);
        receivers.push(rx);

        computer.load_program(program.clone());

        std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
            let mut computer = computer;
            computer.execute()?;
            Ok(())
        });
    }

    let mut nat = None;
    let mut last_y = -1;

    loop {
        for sender in &senders {
            sender.send(-1)?;
        }

        // Give the threads some time to catch up.
        std::thread::sleep(std::time::Duration::from_millis(1));

        let packet = receivers.iter().find_map(|rx| {
            if let Ok(address) = rx.try_recv() {
                let x = rx.recv().unwrap();
                let y = rx.recv().unwrap();
                Some((address, x, y))
            } else {
                None
            }
        });

        match packet {
            Some((255, x, y)) => {
                nat = Some((x, y));
            }
            Some((address, x, y)) => {
                senders[address as usize].send(x)?;
                senders[address as usize].send(y)?;
            }
            None => {
                if let Some((x, y)) = nat {
                    if y == last_y {
                        println!("Part 2: {}", y);
                        break;
                    }

                    last_y = y;
                    senders[0].send(x)?;
                    senders[0].send(y)?;
                }
            }
        }
    }

    Ok(())
}
