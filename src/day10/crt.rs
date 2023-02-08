use std::fmt::Display;

pub enum Instruction {
    AddX(i32),
    Noop,
}

pub struct Crt {
    screen: [[char; 40]; 6],
    sprite: i32,
    pixel: (usize, usize),
}

impl Crt {
    pub fn new() -> Self {
        Crt {
            screen: [['.'; 40]; 6],
            sprite: 1,
            pixel: (0, 0),
        }
    }

    pub fn tick(&mut self) {
        if self.pixel.1 as i32 >= self.sprite - 1 && self.pixel.1 as i32 <= self.sprite + 1 {
            self.screen[self.pixel.0][self.pixel.1] = '#';
        }

        self.pixel.1 += 1;
        if self.pixel.1 % 40 == 0 {
            self.pixel = (self.pixel.0 + 1, 0);
        }
    }

    pub fn add_x(&mut self, dx: i32) {
        self.sprite += dx
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self
            .screen
            .iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "\n{}", out)
    }
}
