use fxhash::FxHashMap;
use smallvec::{smallvec, SmallVec};

use crate::AocSolver;

pub struct Day11Solver;

impl AocSolver for Day11Solver {
    type Output = u64;

    fn part_1(input: &str) -> Self::Output {
        let stones = Stones::from(input).blink(25);
        stones.len() as u64
    }

    fn part_2(input: &str) -> Self::Output {
        let stones = Stones::from(input).blink(75);
        stones.len() as u64
    }
}

fn split(n: u64, digits: u32) -> (u64, u64) {
    let len = 10_u64.pow(digits / 2);
    let a = n / len;
    let b = n % len;
    (a, b)
}

fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

type Stone = u64;

struct Stones {
    inner: Vec<Stone>,
}

impl Stones {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    fn blink(self, total_blinks: usize) -> Self {
        let mut blinked = 0;

        let mut next = self.inner.clone();
        let mut result: Vec<Stone> = Vec::with_capacity(self.len() * 2);
        let mut memo: FxHashMap<Stone, SmallVec<[Stone; 2]>> = FxHashMap::default();

        while blinked < total_blinks {
            for stone in &next {
                if let Some(cache) = memo.get(stone) {
                    for stone in cache {
                        result.push(*stone);
                    }
                } else {
                    if let Some(new_stone) = apply_rule(ZeroToOne, *stone) {
                        memo.insert(*stone, new_stone.clone());
                        result.push(new_stone[0]);
                        continue;
                    }
                    if let Some(new_stones) = apply_rule(EvenDigits, *stone) {
                        memo.insert(*stone, new_stones.clone());
                        for stone in new_stones {
                            result.push(stone);
                        }
                        continue;
                    }
                    if let Some(new_stone) = apply_rule(Mul2024, *stone) {
                        memo.insert(*stone, new_stone.clone());
                        result.push(new_stone[0]);
                    }
                }
            }
            blinked += 1;
            next.clear();
            next.append(&mut result);
        }

        Self { inner: next }
    }
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        Self {
            inner: value
                .split(' ')
                .map(|n| {
                    if n.contains('\n') {
                        n[0..n.len() - 1].parse::<Stone>().unwrap()
                    } else {
                        n.parse::<Stone>().unwrap()
                    }
                })
                .collect(),
        }
    }
}

struct ZeroToOne;

impl Rule for ZeroToOne {
    fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>> {
        match stone {
            0 => Some(smallvec![1]),
            _ => None,
        }
    }
}

struct EvenDigits;

impl Rule for EvenDigits {
    fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>> {
        let digits = num_digits(stone);

        // if even
        if digits % 2 == 0 {
            let (a, b) = split(stone, digits);
            Some(smallvec![a, b])
        } else {
            None
        }
    }
}

struct Mul2024;

impl Rule for Mul2024 {
    fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>> {
        Some(smallvec![stone * 2024])
    }
}

trait Rule {
    fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>>;
}

fn apply_rule<R>(rule: R, stone: Stone) -> Option<SmallVec<[Stone; 2]>>
where
    R: Rule,
{
    rule.execute(stone)
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r#"125 17"#;

    #[test]
    fn split_num() {
        let (a, b) = split(123_456, 6);
        assert_eq!(a, 123);
        assert_eq!(b, 456);
    }

    #[test]
    fn parses() {
        let stones = Stones::from(SAMPLE);
        assert_eq!(stones.len(), 2);
    }

    #[test]
    fn blinks() {
        let stones = Stones::from(SAMPLE);
        let next_blink = stones.blink(1);
        assert_eq!(next_blink.inner, vec![253000, 1, 7]);
    }

    #[test]
    fn blinks_twice() {
        let stones = Stones::from(SAMPLE);
        let next_blink = stones.blink(2);
        assert_eq!(next_blink.inner, vec![253, 0, 2024, 14168]);
    }

    #[test]
    fn rule_zero_to_one() {
        let stone = apply_rule(ZeroToOne, 0);
        assert_eq!(stone.unwrap()[0], 1);
    }

    #[test]
    fn rule_even_digits() {
        let stone = apply_rule(EvenDigits, 1234);
        let expected: SmallVec<[u64; 2]> = smallvec![12, 34];
        assert_eq!(stone.unwrap(), expected);
    }

    #[test]
    fn rule_mul_2024() {
        let stone = apply_rule(Mul2024, 5);
        assert_eq!(stone.unwrap()[0], 10120);
    }

    #[test]
    fn solves_part_1() {
        let answer = Day11Solver::part_1(SAMPLE);
        assert_eq!(answer, 55312);
    }
}
