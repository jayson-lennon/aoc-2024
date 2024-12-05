use crate::AocSolver;
use regex::Regex;

type Operand = u32;

pub struct Day03Solver;
impl AocSolver for Day03Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let operands = find_valid_mul_operands(input);
        operands.into_iter().map(|(lhs, rhs)| lhs * rhs).sum()
    }

    fn part_2(input: &str) -> Self::Output {
        let mut stack = find_valid_mul_operands_with_indices(input)
            .into_iter()
            .map(|(index, (a, b))| StackItem::Mul(index, (a, b)))
            .chain(find_do_indices(input).into_iter().map(StackItem::Do))
            .chain(find_dont_indices(input).into_iter().map(StackItem::Dont))
            .collect::<Vec<_>>();
        stack.sort();

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum State {
            Do,
            Dont,
        }

        let mut current_state = State::Do;
        let mut total = 0;
        for item in stack {
            match (current_state, item) {
                (State::Do, StackItem::Mul(_, (a, b))) => total += a * b,
                (State::Do, StackItem::Dont(_)) => current_state = State::Dont,
                (_, StackItem::Do(_)) => current_state = State::Do,
                _ => (),
            }
        }

        total
    }
}

fn find_valid_mul_operands(input: &str) -> Vec<(Operand, Operand)> {
    let re = Regex::new(r"mul\((?<first>\d*),(?<second>\d*)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let (_, [first, second]) = cap.extract();
            (first, second)
        })
        .map(|(first, second)| {
            (
                first.parse::<Operand>().unwrap(),
                second.parse::<Operand>().unwrap(),
            )
        })
        .collect()
}

type Index = usize;

fn find_valid_mul_operands_with_indices(input: &str) -> Vec<(Index, (Operand, Operand))> {
    let re = Regex::new(r"mul\((?<first>\d*),(?<second>\d*)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let start = cap.get(0).unwrap().start();
            let (_, [first, second]) = cap.extract();
            (start, (first, second))
        })
        .map(|(start, (first, second))| {
            (
                start,
                (
                    first.parse::<Operand>().unwrap(),
                    second.parse::<Operand>().unwrap(),
                ),
            )
        })
        .collect()
}

fn find_do_indices(input: &str) -> Vec<Index> {
    let re = Regex::new(r"do\(\)").unwrap();
    re.find_iter(input).map(|mat| mat.start()).collect()
}

fn find_dont_indices(input: &str) -> Vec<Index> {
    let re = Regex::new(r"don't\(\)").unwrap();
    re.find_iter(input).map(|mat| mat.start()).collect()
}

#[derive(Debug, Clone, Copy)]
enum StackItem {
    Do(Index),
    Dont(Index),
    Mul(Index, (u32, u32)),
}

impl StackItem {
    fn index(&self) -> &Index {
        match self {
            StackItem::Do(i) => i,
            StackItem::Dont(i) => i,
            StackItem::Mul(i, _) => i,
        }
    }
}

impl Eq for StackItem {}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Do(l0), Self::Do(r0)) => l0 == r0,
            (Self::Dont(l0), Self::Dont(r0)) => l0 == r0,
            (Self::Mul(l0, _), Self::Mul(r0, _)) => l0 == r0,
            _ => false,
        }
    }
}

impl Ord for StackItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            StackItem::Do(i) => i.cmp(other.index()),
            StackItem::Dont(i) => i.cmp(other.index()),
            StackItem::Mul(i, _) => i.cmp(other.index()),
        }
    }
}

impl PartialOrd for StackItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// match

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PART_1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const SAMPLE_PART_2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn finds_valid_mul_operands() {
        let operands = super::find_valid_mul_operands(SAMPLE_PART_1);

        assert_eq!(operands, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn solves_part_1() {
        let solution = Day03Solver::part_1(SAMPLE_PART_1);

        assert_eq!(solution, 161)
    }

    #[test]
    fn finds_mul_operands_with_indices() {
        let operands = super::find_valid_mul_operands_with_indices(SAMPLE_PART_2);

        assert_eq!(
            operands,
            vec![(1, (2, 4)), (28, (5, 5)), (48, (11, 8)), (64, (8, 5))]
        );
    }

    #[test]
    fn finds_do_indices() {
        let do_it_indices = super::find_do_indices(SAMPLE_PART_2);

        assert_eq!(do_it_indices, vec![59]);
    }

    #[test]
    fn finds_dont_indices() {
        let dont_do_it_indices = super::find_dont_indices(SAMPLE_PART_2);

        assert_eq!(dont_do_it_indices, vec![20]);
    }

    #[test]
    fn solves_part_2() {
        let solution = Day03Solver::part_2(SAMPLE_PART_2);
        assert_eq!(solution, 48)
    }
}
