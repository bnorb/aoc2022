fn calc_move_score(their: char, mine: char) -> u32 {
    let score = mine as u32 - 'W' as u32;
    score
        + match (their, mine) {
            ('B', 'Z') | ('A', 'Y') | ('C', 'X') => 6,
            ('A', 'Z') | ('C', 'Y') | ('B', 'X') => 0,
            _ => 3,
        }
}

fn calc_move_score_correct(their: char, outcome: char) -> u32 {
    let mine;
    let their = their as u32 - 'A' as u32;

    let score = match (their, outcome) {
        (their, 'X') => {
            mine = (their + 2) % 3;
            0
        }
        (their, 'Z') => {
            mine = (their + 1) % 3;
            6
        }
        (their, _) => {
            mine = their;
            3
        }
    };

    score + mine + 1
}

#[aoc(day2, part1)]
pub fn rps(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calc_move_score(l.chars().next().unwrap(), l.chars().nth(2).unwrap()))
        .sum()
}

#[aoc(day2, part2)]
pub fn rps_correct(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calc_move_score_correct(l.chars().next().unwrap(), l.chars().nth(2).unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winning_score_is_6_plus_move() {
        assert_eq!(calc_move_score('C', 'X'), 6 + 1);
        assert_eq!(calc_move_score('A', 'Y'), 6 + 2);
        assert_eq!(calc_move_score('B', 'Z'), 6 + 3);
    }

    #[test]
    fn draw_score_is_3_plus_move() {
        assert_eq!(calc_move_score('A', 'X'), 3 + 1);
        assert_eq!(calc_move_score('B', 'Y'), 3 + 2);
        assert_eq!(calc_move_score('C', 'Z'), 3 + 3);
    }

    #[test]
    fn losing_score_is_only_move() {
        assert_eq!(calc_move_score('B', 'X'), 1);
        assert_eq!(calc_move_score('C', 'Y'), 2);
        assert_eq!(calc_move_score('A', 'Z'), 3);
    }

    #[test]
    fn correct_winning_score_is_6_plus_move() {
        assert_eq!(calc_move_score_correct('A', 'Z'), 6 + 2);
        assert_eq!(calc_move_score_correct('B', 'Z'), 6 + 3);
        assert_eq!(calc_move_score_correct('C', 'Z'), 6 + 1);
    }

    #[test]
    fn correct_draw_score_is_3_plus_move() {
        assert_eq!(calc_move_score_correct('A', 'Y'), 3 + 1);
        assert_eq!(calc_move_score_correct('B', 'Y'), 3 + 2);
        assert_eq!(calc_move_score_correct('C', 'Y'), 3 + 3);
    }

    #[test]
    fn correct_losing_score_is_only_move() {
        assert_eq!(calc_move_score_correct('A', 'X'), 3);
        assert_eq!(calc_move_score_correct('B', 'X'), 1);
        assert_eq!(calc_move_score_correct('C', 'X'), 2);
    }
}
