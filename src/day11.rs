use cached::proc_macro::cached;
use rules::{EvenDigits, Mul2024, Rule, ZeroToOne};

use crate::AocSolver;

pub struct Day11Solver;

impl AocSolver for Day11Solver {
    type Output = u64;

    fn part_1(input: &str) -> Self::Output {
        Stones::from(input).blink(25)
    }

    fn part_2(input: &str) -> Self::Output {
        Stones::from(input).blink(75)
    }
}

type Stone = u64;

struct Stones {
    inner: Vec<Stone>,
}

impl Stones {
    fn blink(self, blinks: usize) -> u64 {
        self.inner
            .iter()
            .map(|stone| blink_impl(*stone, blinks))
            .sum()
    }
}

#[cached]
#[inline(always)]
fn blink_impl(stone: Stone, blink_n: usize) -> u64 {
    if blink_n == 0 {
        return 1;
    }

    let mut stones = 0;

    for stone in ZeroToOne
        .execute(stone)
        .or_else(|| EvenDigits.execute(stone))
        .or_else(|| Mul2024.execute(stone))
        .unwrap()
    {
        stones += blink_impl(stone, blink_n - 1);
    }
    stones
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

mod rules {
    use smallvec::{smallvec, SmallVec};

    use super::Stone;
    pub trait Rule {
        fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>>;
    }

    pub struct ZeroToOne;

    impl Rule for ZeroToOne {
        #[inline(always)]
        fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>> {
            match stone {
                0 => Some(smallvec![1]),
                _ => None,
            }
        }
    }

    pub struct EvenDigits;

    impl Rule for EvenDigits {
        #[inline(always)]
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

    pub struct Mul2024;

    impl Rule for Mul2024 {
        #[inline(always)]
        fn execute(&self, stone: Stone) -> Option<SmallVec<[Stone; 2]>> {
            Some(smallvec![stone * 2024])
        }
    }

    /// Splits `n` in half (1234 -> 12 34).
    fn split(n: u64, digits: u32) -> (u64, u64) {
        let len = 10_u64.pow(digits / 2);
        let a = n / len;
        let b = n % len;
        (a, b)
    }

    /// Returns the number of digits in a number.
    fn num_digits(n: u64) -> u32 {
        n.ilog10() + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_num() {
            let (a, b) = split(123_456, 6);
            assert_eq!(a, 123);
            assert_eq!(b, 456);
        }

        #[test]
        fn zero_to_one() {
            let stone = ZeroToOne.execute(0);
            assert_eq!(stone.unwrap()[0], 1);
        }

        #[test]
        fn even_digits() {
            let stone = EvenDigits.execute(1234);
            let expected: SmallVec<[u64; 2]> = smallvec![12, 34];
            assert_eq!(stone.unwrap(), expected);
        }

        #[test]
        fn mul_2024() {
            let stone = Mul2024.execute(5);
            assert_eq!(stone.unwrap()[0], 10120);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r#"125 17"#;

    #[test]
    fn parses() {
        let stones = Stones::from(SAMPLE);
        assert_eq!(stones.inner.len(), 2);
    }

    #[test]
    fn blinks() {
        let stones = Stones::from(SAMPLE);
        let next_blink = stones.blink(1);
        assert_eq!(next_blink, 3);
    }

    #[test]
    fn blinks_twice() {
        let stones = Stones::from(SAMPLE);
        let next_blink = stones.blink(2);
        assert_eq!(next_blink, 4);
    }

    #[test]
    fn solves_part_1() {
        let answer = Day11Solver::part_1(SAMPLE);
        assert_eq!(answer, 55312);
    }
}
