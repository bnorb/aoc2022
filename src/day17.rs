mod tetris;

use self::tetris::{Chamber, InfiniteStorm, Rock, Wind};
use std::collections::HashMap;

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<Wind> {
    input.chars().map(|c| Wind::new(c)).collect()
}

#[aoc(day17, part1)]
fn tetris(winds: &Vec<Wind>) -> usize {
    let storm = InfiniteStorm::new(winds);
    let mut chamber = Chamber::new(storm);
    let mut rock = Rock::Horizontal;

    for _ in 0..2022 {
        chamber.drop_rock(&rock);
        rock = rock.next().unwrap();
    }

    chamber.get_height()
}

type ChamberState = ([u8; 7], usize, Rock);
type Repetition = ((usize, usize), (usize, usize));

fn calculate_rep_height((start, end): Repetition) -> (usize, usize) {
    let gain_per_rep = end.1 - start.1;
    let idx_diff = end.0 - start.0;
    let remaining = 1000000000000 - end.0;
    let rep_height = (remaining / idx_diff) * gain_per_rep;
    let rocks_left = remaining % idx_diff;

    (rep_height, rocks_left)
}

#[aoc(day17, part2)]
fn bigass_tetris(winds: &Vec<Wind>) -> usize {
    let storm = InfiniteStorm::new(winds);
    let mut chamber = Chamber::new(storm);
    let mut rock = Rock::Horizontal;
    let mut states: HashMap<ChamberState, (usize, usize)> = HashMap::new();

    let mut fallen: usize = 0;
    let repetition;
    loop {
        chamber.drop_rock(&rock);
        fallen += 1;
        let state = (
            chamber.get_top(),
            chamber.get_wind_index(),
            rock.next().unwrap(),
        );

        rock = rock.next().unwrap();
        if states.contains_key(&state) {
            repetition = (
                states.get(&state).unwrap().clone(),
                (fallen, chamber.get_height()),
            );
            break;
        }
        states.insert(state, (fallen, chamber.get_height()));
    }

    let (rep_height, rocks_left) = calculate_rep_height(repetition);
    for _ in 0..rocks_left {
        chamber.drop_rock(&rock);
        rock = rock.next().unwrap();
    }

    rep_height + chamber.get_height()
}
