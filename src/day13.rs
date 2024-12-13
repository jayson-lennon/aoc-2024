use crate::AocSolver;
use regex::Regex;

pub struct Day13Solver;

impl AocSolver for Day13Solver {
    type Output = i64;

    fn part_1(input: &str) -> Self::Output {
        let machine = ClawMachine::from(input);
        machine
            .combos
            .iter()
            .filter_map(|combo| combo.solve().map(|[a, b]| (a * 3) + b))
            .sum()
    }

    fn part_2(input: &str) -> Self::Output {
        let machine = ClawMachine::from(input);
        machine
            .combos
            .iter()
            .filter_map(|combo| combo.solve_part2().map(|[a, b]| (a * 3) + b))
            .sum()
    }
}

struct ButtonCombo {
    a: [i64; 2],
    b: [i64; 2],
    p: [i64; 2],
}

impl ButtonCombo {
    #[inline(always)]
    fn solve(&self) -> Option<[i64; 2]> {
        solve_system(self.a, self.b, self.p)
    }

    #[inline(always)]
    fn solve_part2(&self) -> Option<[i64; 2]> {
        solve_system(
            self.a,
            self.b,
            [self.p[0] + 10000000000000, self.p[1] + 10000000000000],
        )
    }
}

struct ClawMachine {
    combos: Vec<ButtonCombo>,
}

#[inline(always)]
fn solve_system([ax, ay]: [i64; 2], [bx, by]: [i64; 2], [px, py]: [i64; 2]) -> Option<[i64; 2]> {
    let b = ((ax * py) - (ay * px)) / ((ax * by) - (ay * bx));
    let a = (px - (bx * b)) / ax;

    if (a * ax) + (b * bx) != px || (a * ay) + (b * by) != py {
        None
    } else {
        Some([a, b])
    }
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let re_equation = Regex::new(r#"X\+(\d*), Y\+(\d*)"#).unwrap();
        let re_prize = Regex::new(r#"X=(\d*), Y=(\d*)"#).unwrap();

        let mut combos = Vec::default();
        let mut lines = value.lines();
        loop {
            let a = lines.next().unwrap();
            let b = lines.next().unwrap();
            let prize = lines.next().unwrap();

            let a = {
                let caps = re_equation.captures(a).unwrap();
                parse_captures(caps)
            };
            let b = {
                let caps = re_equation.captures(b).unwrap();
                parse_captures(caps)
            };
            let p = {
                let caps = re_prize.captures(prize).unwrap();
                parse_captures(caps)
            };

            combos.push(ButtonCombo { a, b, p });

            if lines.next().is_none() {
                break;
            }
        }
        Self { combos }
    }
}

#[inline(always)]
fn parse_captures(caps: regex::Captures<'_>) -> [i64; 2] {
    [
        caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    const SAMPLE_2: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279"#;

    #[test]
    fn parses() {
        let machine = ClawMachine::from(SAMPLE);
        assert_eq!(machine.combos.len(), 4);
    }

    #[test]
    fn solve_system_of_equations() {
        let [a, b] = solve_system([94, 34], [22, 67], [8400, 5400]).unwrap();
        assert_eq!(a, 80);
        assert_eq!(b, 40);
    }

    #[test]
    fn returns_none_when_no_solution() {
        let answer = solve_system([26, 66], [67, 21], [12748, 12176]);
        assert!(answer.is_none());
    }

    #[test]
    fn solves_part_1() {
        let answer = Day13Solver::part_1(SAMPLE);
        assert_eq!(answer, 480);
    }

    #[test]
    fn check_int_overflow() {
        // this will panic if we go too high
        Day13Solver::part_2(SAMPLE_2);
    }
}
