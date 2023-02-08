use std::collections::HashSet;

type TreeData = (usize, usize, u8);

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Vec<TreeData>> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| (row, col, char.to_digit(10).unwrap() as u8))
                .collect()
        })
        .collect()
}

fn get_column<T: Copy>(col_index: usize, input: &Vec<Vec<T>>) -> Vec<T> {
    input.iter().fold(Vec::new(), |mut col, row| {
        col.push(*row.get(col_index).unwrap());
        col
    })
}

#[aoc(day8, part1)]
fn visible_trees(input: &Vec<Vec<TreeData>>) -> usize {
    let mut visible = HashSet::new();

    let mut fold = |highest: Option<u8>, (r, c, tree): &TreeData| match highest {
        None => {
            visible.insert((*r, *c));
            Some(*tree)
        }
        Some(highest) => {
            if highest < *tree {
                visible.insert((*r, *c));
                Some(*tree)
            } else {
                Some(highest)
            }
        }
    };

    input.iter().for_each(|row| {
        row.iter().fold(None, &mut fold);
        row.iter().rev().fold(None, &mut fold);
    });

    let mut i = 0;
    while i < input.first().unwrap().len() {
        let col: Vec<TreeData> = get_column(i, input);

        col.iter().fold(None, &mut fold);
        col.iter().rev().fold(None, &mut fold);
        i += 1;
    }

    visible.len()
}

fn find_f(tree_height: u8, from: usize, vec: &Vec<TreeData>) -> Option<&TreeData> {
    let mut iter = vec.iter();
    iter.nth(from);
    iter.find(|t| t.2 >= tree_height)
}

fn find_b(tree_height: u8, from: usize, vec: &Vec<TreeData>) -> Option<&TreeData> {
    vec.iter().take(from).rev().find(|t| t.2 >= tree_height)
}

#[aoc(day8, part2)]
fn calc_scores(input: &Vec<Vec<TreeData>>) -> usize {
    input.iter().fold(0, |score, row| {
        let row_best = row.iter().fold(0, |score, t| {
            let left = t.1 - find_b(t.2, t.1, row).unwrap_or(&(0, 0, 0)).1;
            let right = find_f(t.2, t.1, row).unwrap_or(&(0, row.len() - 1, 0)).1 - t.1;

            let col: Vec<TreeData> = get_column(t.1, input);
            let top = t.0 - find_b(t.2, t.0, &col).unwrap_or(&(0, 0, 0)).0;
            let bottom = find_f(t.2, t.0, &col).unwrap_or(&(col.len() - 1, 0, 0)).0 - t.0;

            let s = left * right * top * bottom;

            if s > score {
                s
            } else {
                score
            }
        });

        if row_best > score {
            row_best
        } else {
            score
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_column() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(get_column(0, &data), vec![1, 5, 9]);
        assert_eq!(get_column(1, &data), vec![2, 6, 10]);
        assert_eq!(get_column(2, &data), vec![3, 7, 11]);
        assert_eq!(get_column(3, &data), vec![4, 8, 12]);
    }

    #[test]
    fn can_find_forwards() {
        let vec = vec![(0, 0, 1), (0, 1, 2), (0, 2, 3)];
        assert_eq!(find_f(1, 0, &vec), Some(&(0, 1, 2)));
        assert_eq!(find_f(2, 0, &vec), Some(&(0, 1, 2)));
        assert_eq!(find_f(4, 0, &vec), None);
        assert_eq!(find_f(3, 0, &vec), Some(&(0, 2, 3)));
        assert_eq!(find_f(3, 1, &vec), Some(&(0, 2, 3)));
        assert_eq!(find_f(3, 2, &vec), None);
    }

    #[test]
    fn can_find_backwards() {
        let vec = vec![(0, 0, 1), (0, 1, 2), (0, 2, 3)];
        assert_eq!(find_b(1, 2, &vec), Some(&(0, 1, 2)));
        assert_eq!(find_b(2, 2, &vec), Some(&(0, 1, 2)));
        assert_eq!(find_b(3, 2, &vec), None);
        assert_eq!(find_b(0, 0, &vec), None);
        assert_eq!(find_b(1, 1, &vec), Some(&(0, 0, 1)));
    }
}
