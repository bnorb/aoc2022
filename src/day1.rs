#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| -> u32 { l.trim().parse().unwrap() })
                .sum()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn most_calories(input: &Vec<u32>) -> u32 {
    *input
        .iter()
        .reduce(|max, e| if max < e { e } else { max })
        .unwrap()
}

#[aoc(day1, part2)]
pub fn total_calories(input: &Vec<u32>) -> u32 {
    let top_three = input.iter().fold((&0_u32, &0_u32, &0_u32), |mut maxes, e| {
        if e > maxes.0 {
            maxes.2 = maxes.1;
            maxes.1 = maxes.0;
            maxes.0 = e;
        } else if e > maxes.1 {
            maxes.2 = maxes.1;
            maxes.1 = e;
        } else if e > maxes.2 {
            maxes.2 = e;
        }

        maxes
    });

    top_three.0 + top_three.1 + top_three.2
}
