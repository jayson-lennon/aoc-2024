use std::ops::{Add, AddAssign, Deref};

#[derive(Debug, Clone, Copy)]
pub struct WrappingI64 {
    value: i64,
    min: i64,
    max: i64,
}

impl WrappingI64 {
    pub fn new(initial: i64, (min, max): (i64, i64)) -> Self {
        Self {
            value: initial,
            min,
            max,
        }
    }

    fn wrap(self, rhs: i64) -> i64 {
        let n = self.max - self.min + 1;
        ((self.value + rhs - self.min) % n + n) % n + self.min
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }
}

impl PartialEq<i64> for WrappingI64 {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other
    }
}

impl PartialEq<WrappingI64> for i64 {
    fn eq(&self, other: &WrappingI64) -> bool {
        *self == other.value
    }
}

impl PartialEq<WrappingI64> for WrappingI64 {
    fn eq(&self, other: &WrappingI64) -> bool {
        self.value == other.value
    }
}

impl Eq for WrappingI64 {}

impl Add<i64> for WrappingI64 {
    type Output = WrappingI64;

    fn add(self, rhs: i64) -> Self::Output {
        WrappingI64 {
            value: self.wrap(rhs),
            min: self.min,
            max: self.max,
        }
    }
}

impl Add<WrappingI64> for WrappingI64 {
    type Output = WrappingI64;

    fn add(self, rhs: WrappingI64) -> Self::Output {
        rhs + self.value
    }
}

impl AddAssign<i64> for WrappingI64 {
    fn add_assign(&mut self, rhs: i64) {
        self.value = self.wrap(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps() {
        let cases = [
            ((0, (0, 11)), (6, 6)),
            ((5, (0, 5)), (2, 1)),
            ((5, (0, 7)), (5, 2)),
            ((5, (-1, 7)), (3, -1)),
            ((5, (-1, 7)), (-3, 2)),
            ((2, (-3, 7)), (7, -2)),
            ((-2, (-3, 7)), (10, -3)),
            ((-2, (-4, -1)), (3, -3)),
            ((-2, (-4, -1)), (-3, -1)),
        ];
        for (i, ((n, (min, max)), (add, expected))) in cases.into_iter().enumerate() {
            let mut n = WrappingI64::new(n, (min, max));
            n += add;
            assert_eq!(n, expected, "failed on index {i}");
        }
    }
}
