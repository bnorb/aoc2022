mod crt;

use self::crt::{Instruction, Crt};

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Instruction::Noop,
            addx => Instruction::AddX(addx[5..].parse().unwrap()),
        })
        .collect()
}

#[aoc(day10, part1)]
fn get_signal_strength(instructions: &Vec<Instruction>) -> i32 {
    let mut clock = 1;
    let mut x = 1;
    let mut next_point = 20;
    let mut sum = 0;

    for inst in instructions {
        if clock == next_point || clock == next_point - 1 {
            sum += x * next_point;
            if next_point == 220 {
                break;
            }

            next_point += 40;
        }

        match inst {
            Instruction::Noop => clock += 1,
            Instruction::AddX(dx) => {
                x += dx;
                clock += 2;
            }
        }
    }

    sum
}

#[aoc(day10, part2)]
fn display(instructions: &Vec<Instruction>) -> Crt {
    let mut crt = Crt::new();

    instructions.iter().for_each(|inst| {
        crt.tick();
        if let Instruction::AddX(dx) = inst {
            crt.tick();
            crt.add_x(*dx);
        }
    });

    crt
}
