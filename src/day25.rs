mod snafu;

use self::snafu::Snafu;

#[aoc(day25, part1)]
fn calc_snafu(input: &str) -> Snafu {
    let snafus: Vec<Snafu> = input.lines().map(|line| Snafu::parse(line)).collect();
    snafus.into_iter().reduce(|sum, snafu| sum + snafu).unwrap()
}
