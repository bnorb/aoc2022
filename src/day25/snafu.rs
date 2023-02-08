use std::{collections::LinkedList, fmt::Display, ops::Add};

#[derive(Clone, Copy, PartialEq)]
enum Digit {
    N2,
    N1,
    Z,
    P1,
    P2,
}

impl Digit {
    fn new(c: char) -> Self {
        match c {
            '=' => Digit::N2,
            '-' => Digit::N1,
            '0' => Digit::Z,
            '1' => Digit::P1,
            '2' => Digit::P2,
            _ => unimplemented!(),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::N2 => write!(f, "="),
            Digit::N1 => write!(f, "-"),
            Digit::Z => write!(f, "0"),
            Digit::P1 => write!(f, "1"),
            Digit::P2 => write!(f, "2"),
        }
    }
}

impl Add for Digit {
    type Output = (Digit, Digit);

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Digit::Z, rhs) => (Digit::Z, rhs),
            (lhs, Digit::Z) => (Digit::Z, lhs),
            (Digit::P1, Digit::P1) => (Digit::Z, Digit::P2),
            (Digit::P1, Digit::P2) | (Digit::P2, Digit::P1) => (Digit::P1, Digit::N2),
            (Digit::P1, Digit::N1) | (Digit::N1, Digit::P1) => (Digit::Z, Digit::Z),
            (Digit::P1, Digit::N2) | (Digit::N2, Digit::P1) => (Digit::Z, Digit::N1),
            (Digit::P2, Digit::P2) => (Digit::P1, Digit::N1),
            (Digit::P2, Digit::N1) | (Digit::N1, Digit::P2) => (Digit::Z, Digit::P1),
            (Digit::P2, Digit::N2) | (Digit::N2, Digit::P2) => (Digit::Z, Digit::Z),
            (Digit::N1, Digit::N1) => (Digit::Z, Digit::N2),
            (Digit::N1, Digit::N2) | (Digit::N2, Digit::N1) => (Digit::N1, Digit::P2),
            (Digit::N2, Digit::N2) => (Digit::N1, Digit::P1),
        }
    }
}

pub struct Snafu(LinkedList<Digit>);

impl Snafu {
    pub fn parse(input: &str) -> Self {
        Self(input.chars().map(|c| Digit::new(c)).collect())
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.0.iter() {
            write!(f, "{digit}")?
        }
        Ok(())
    }
}

impl Add for Snafu {
    type Output = Snafu;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut sum = LinkedList::new();
        let mut carry = LinkedList::new();
        let mut s;

        let len = self.0.len().min(rhs.0.len());

        let add = |a: Digit, b: Digit, mut carry: LinkedList<Digit>| {
            let c_count = carry.len();
            let (mut c, mut s) = a + b;

            if c != Digit::Z {
                carry.push_back(c);
            }

            for _ in 0..c_count {
                (c, s) = s + carry.pop_front().unwrap();
                if c != Digit::Z {
                    carry.push_back(c);
                }
            }

            (s, carry)
        };

        for _ in 0..len {
            let a = self.0.pop_back().unwrap();
            let b = rhs.0.pop_back().unwrap();
            (s, carry) = add(a, b, carry);

            sum.push_front(s);
        }

        while let Some(a) = carry.pop_front() {
            let b = if self.0.len() > 0 {
                self.0.pop_back().unwrap()
            } else if rhs.0.len() > 0 {
                rhs.0.pop_back().unwrap()
            } else {
                Digit::Z
            };

            (s, carry) = add(a, b, carry);
            sum.push_front(s);
        }

        let mut remaining = if self.0.len() > 0 {
            self.0
        } else if rhs.0.len() > 0 {
            rhs.0
        } else {
            LinkedList::new()
        }
        .into_iter();

        while let Some(rem) = remaining.next_back() {
            sum.push_front(rem)
        }

        Self(sum)
    }
}
