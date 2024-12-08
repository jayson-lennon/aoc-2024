use rayon::prelude::*;
use smallvec::SmallVec;
use std::ops::Index;

use crate::AocSolver;

pub struct Day07Solver;

impl AocSolver for Day07Solver {
    type Output = u64;

    fn part_1(input: &str) -> Self::Output {
        let equation_parts = EquationParts::from(input);
        equation_parts
            .parts
            .par_iter()
            .filter_map(|part| {
                if part.can_be_made_true(&[add, mul]) {
                    Some(part.answer)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Output {
        let equation_parts = EquationParts::from(input);

        let (part_1_true, need_concat): (Vec<_>, Vec<_>) = equation_parts
            .parts
            .iter()
            .partition(|part| part.can_be_made_true(&[add, mul]));

        let part_1_sum = part_1_true.into_iter().map(|part| part.answer).sum::<u64>();

        let part_2_sum = need_concat
            .par_iter()
            .filter_map(|part| {
                if part.can_be_made_true(&[add, mul, concat]) {
                    Some(part.answer)
                } else {
                    None
                }
            })
            .sum::<u64>();

        part_1_sum + part_2_sum
    }
}

#[derive(Clone, Debug)]
struct CalibrationEquation {
    answer: u64,
    operands: SmallVec<[u64; 12]>,
}

type MathOp = fn(u64, u64) -> u64;

impl CalibrationEquation {
    fn can_be_made_true(&self, operations: &[MathOp]) -> bool {
        self.can_be_made_true_impl(operations, &self.operands, 0, self.operands[0])
    }

    fn can_be_made_true_impl(
        &self,
        operations: &[MathOp],
        operands: &[u64],
        operand_index: usize,
        current_result: u64,
    ) -> bool {
        // return the result once we've reached the end
        if operand_index == operands.len() - 1 {
            return current_result == self.answer;
        }

        for &op in operations {
            let next_result = op(current_result, operands[operand_index + 1]);

            if self.can_be_made_true_impl(operations, operands, operand_index + 1, next_result) {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Debug)]
struct EquationParts {
    parts: Vec<CalibrationEquation>,
}

impl Index<usize> for EquationParts {
    type Output = CalibrationEquation;

    fn index(&self, index: usize) -> &Self::Output {
        &self.parts[index]
    }
}

impl From<&str> for EquationParts {
    fn from(value: &str) -> Self {
        Self {
            parts: value
                .lines()
                .map(|line| {
                    let (answer, operands) = line.split_once(':').unwrap();
                    let answer = answer.parse::<u64>().unwrap();
                    let operands = operands
                        .split(' ')
                        .filter(|ch| !ch.is_empty())
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<SmallVec<[u64; 12]>>();
                    CalibrationEquation { answer, operands }
                })
                .collect(),
        }
    }
}

#[inline(always)]
fn add(lhs: u64, rhs: u64) -> u64 {
    lhs + rhs
}

#[inline(always)]
fn mul(lhs: u64, rhs: u64) -> u64 {
    lhs * rhs
}

#[inline(always)]
fn concat(a: u64, b: u64) -> u64 {
    let b_digits = if b == 0 {
        1
    } else {
        (b as f64).log10().floor() as u32 + 1
    };

    a * 10u64.pow(b_digits) + b
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn parses() {
        let equation_parts = EquationParts::from(SAMPLE);

        assert_eq!(equation_parts[0].answer, 190);
        assert_eq!(equation_parts[0].operands.to_vec(), vec![10, 19]);
    }

    #[test]
    fn left_to_right_evaluation() {
        let equation_parts = EquationParts::from(SAMPLE);

        let answer = equation_parts[1].can_be_made_true(&[add, mul]);
        assert!(answer);
    }

    #[test]
    fn solves_part1() {
        let answer = Day07Solver::part_1(SAMPLE);

        assert_eq!(answer, 3749);
    }

    #[test]
    fn concat_two_numbers() {
        assert_eq!(concat(8, 4), 84);
        assert_eq!(concat(8, 40), 840);
        assert_eq!(concat(8, 400), 8400);

        assert_eq!(concat(80, 4), 804);
        assert_eq!(concat(80, 40), 8040);
        assert_eq!(concat(80, 400), 80400);

        assert_eq!(concat(800, 4), 8004);
        assert_eq!(concat(800, 40), 80040);
        assert_eq!(concat(800, 400), 800400);
    }

    #[test]
    fn solves_part_2() {
        let answer = Day07Solver::part_2(SAMPLE);
        assert_eq!(answer, 11387);
    }
}
