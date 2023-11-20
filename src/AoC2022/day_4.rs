use std::str::FromStr;

use anyhow::bail;

#[derive(Debug)]
struct InclusiveRange {
    start: usize,
    end: usize,
}
impl FromStr for InclusiveRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((start, end)) => Ok(Self {
                start: start.parse()?,
                end: end.parse()?,
            }),
            None => bail!("Invalid range: {}", s),
        }
    }
}
impl InclusiveRange {
    fn contain(&self, other: &InclusiveRange) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    fn is_overlapping(&self, other: &InclusiveRange) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
    }
}
pub fn count_inclusion_pair(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| {
            let a = InclusiveRange::from_str(a).unwrap();
            let b = InclusiveRange::from_str(b).unwrap();
            a.contain(&b) || b.contain(&a)
        })
        .filter(|&x| x)
        .count()
}
pub fn count_overlapping_pair(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| {
            let a = InclusiveRange::from_str(a).unwrap();
            let b = InclusiveRange::from_str(b).unwrap();
            if a.is_overlapping(&b) {
                println!("{:?} {:?}", a, b);
            }
            a.is_overlapping(&b)
        })
        .filter(|&x| x)
        .count()
}
