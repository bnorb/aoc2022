mod simulation;

use self::simulation::Simulation;

#[aoc(day23, part1)]
fn sim_10(input: &str) -> usize {
    let mut sim = Simulation::parse(input);
    for _ in 0..10 {
        sim.sim_round();
    }

    let (r_min, r_max, c_min, c_max) = sim.bounds();

    (r_max - r_min + 1) as usize * (c_max - c_min + 1) as usize - sim.elf_count()
}

#[aoc(day23, part2)]
fn sim_all(input: &str) -> usize {
    let mut sim = Simulation::parse(input);
    let mut i = 1;
    while sim.sim_round() {
        i += 1;
    }

    i
}
