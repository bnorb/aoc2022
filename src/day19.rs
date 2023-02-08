mod simulation;

use self::simulation::{Robot, Simulation};

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<[Robot; 4]> {
    input
        .lines()
        .map(|line| Robot::parse_blueprint(line))
        .collect()
}

#[aoc(day19, part1)]
fn calc_quality_level(blueprints: &Vec<[Robot; 4]>) -> usize {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| {
            let mut simulation = Simulation::new(*bp, 24);
            let max = simulation.run();

            (i + 1) * max as usize
        })
        .fold(0, |sum, max| sum + max)
}

#[aoc(day19, part2)]
fn check_top_3(blueprints: &Vec<[Robot; 4]>) -> u16 {
    blueprints
        .iter()
        .take(3)
        .map(|bp| {
            let mut simulation = Simulation::new(*bp, 32);
            simulation.run()
        })
        .reduce(|prod, max| prod * max)
        .unwrap()
}
