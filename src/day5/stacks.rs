#[derive(Debug, Clone, PartialEq)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn parse(stacks: &str) -> Self {
        let mut iter = stacks.lines().rev();
        let indices = iter.next().unwrap();
        let count: usize = indices.trim().split("   ").last().unwrap().parse().unwrap();

        let mut v = vec![vec![]; count];

        while let Some(line) = iter.next() {
            let mut chars = line.trim_end().chars();
            chars.next();
            let mut stack_idx = 0;

            while let Some(char) = chars.next() {
                if char != ' ' {
                    v[stack_idx].push(char);
                }
                chars.nth(2);
                stack_idx += 1;
            }
        }

        Stacks(v)
    }

    pub fn make_move(&mut self, mv: &Move) {
        let mut i = 0;
        while i < mv.count {
            if let Some(c) = self.0[mv.from].pop() {
                self.0[mv.to].push(c);
            }

            i += 1;
        }
    }

    pub fn make_move_at_once(&mut self, mv: &Move) {
        let mut tmp = Vec::new();
        let mut i = 0;

        while i < mv.count {
            if let Some(c) = self.0[mv.from].pop() {
                tmp.push(c);
            }

            i += 1;
        }

        while let Some(c) = tmp.pop() {
            self.0[mv.to].push(c);
        }
    }

    pub fn top(&self) -> String {
        self.0
            .iter()
            .filter(|stack| stack.len() > 0)
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn parse(mv: &str) -> Self {
        let mut parts = mv.trim().split(' ');
        let count = parts.nth(1).unwrap().parse().unwrap();
        let mut from = parts.nth(1).unwrap().parse().unwrap();
        let mut to = parts.nth(1).unwrap().parse().unwrap();

        assert!(from > 0);
        assert!(to > 0);

        from -= 1;
        to -= 1;

        Move { count, from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_stacks() {
        let stacks = "    [D]
[N] [C]    
[Z] [M] [P]
 1   2   3";

        assert_eq!(
            Stacks::parse(stacks),
            Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
        );
    }

    #[test]
    fn can_parse_move() {
        let mv = "move 1 from 2 to 1";
        assert_eq!(
            Move::parse(mv),
            Move {
                count: 1,
                from: 1,
                to: 0
            }
        )
    }
}
