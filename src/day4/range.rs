pub struct Range {
    start: u8,
    end: u8,
}

impl Range {
    pub fn parse(range: &str) -> Self {
        let mut iter = range.split('-').map(|d| d.trim().parse().unwrap());

        Range {
            start: iter.next().unwrap(),
            end: iter.next().unwrap(),
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (other.start <= self.start && other.end >= self.start)
    }
}
