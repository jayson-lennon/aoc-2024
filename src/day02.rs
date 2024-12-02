use std::cmp::Ordering;

type Reports = Vec<Vec<u32>>;
type RawLevels<'a> = Vec<&'a str>;

type Levels = Vec<u32>;

mod parser {
    use super::{Levels, RawLevels, Reports};

    pub fn split_into_levels(raw: &str) -> RawLevels {
        raw.split(' ').collect()
    }

    pub fn make_numeric(levels: RawLevels) -> Levels {
        levels
            .iter()
            .map(|level| level.parse::<u32>().unwrap())
            .collect()
    }

    pub fn parse_reports(raw: &str) -> Reports {
        raw.lines()
            .map(split_into_levels)
            .map(make_numeric)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ReportLevelTrend {
    Decreasing,
    Increasing,
}

/// Returns `true` when all inputs are either increasing or decreasing in value.
fn trend_rule(levels: &Levels) -> bool {
    // figure out expected trend by looking at the first two entries
    let trend = match levels[0].cmp(&levels[1]) {
        Ordering::Greater => ReportLevelTrend::Decreasing,
        Ordering::Less => ReportLevelTrend::Increasing,
        Ordering::Equal => return false,
    };

    // check the remaining entries
    for window in levels.windows(2).skip(1) {
        let [a, b] = window else { unimplemented!() };
        match trend {
            ReportLevelTrend::Increasing => {
                if a > b {
                    return false;
                }
            }
            ReportLevelTrend::Decreasing => {
                if a < b {
                    return false;
                }
            }
        }
    }

    true
}

/// Returns `true` when two adjacent levels are within the tolerance range of 1 to 3.
fn tolerance_rule(levels: &Levels) -> bool {
    for window in levels.windows(2) {
        let [a, b] = window else { unimplemented!() };
        let diff = a.abs_diff(*b);
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

/// Calculates the total number of safe reports (part1 solution)
fn safe_count_part1(reports: Reports) -> usize {
    let trend_mask = reports.iter().map(trend_rule);
    let tolerance_mask = reports.iter().map(tolerance_rule);

    trend_mask
        .zip(tolerance_mask)
        .filter(|(a, b)| *a && *b)
        .count()
}

/// Generates all possible levels by applying a "dampening" (1 level removed at each index).
fn expand_for_dampening(levels: &Levels) -> Vec<Levels> {
    let dampened = levels.iter().enumerate().map(|(i, _)| {
        let mut dampened = levels.clone();
        dampened.remove(i);
        dampened
    });

    vec![levels.clone()].into_iter().chain(dampened).collect()
}

/// Calculates the total number of safe reports (part2 solution)
fn safe_count_part2(reports: Reports) -> usize {
    reports
        .iter()
        .filter(|levels| {
            let dampened_levels = expand_for_dampening(levels);
            safe_count_part1(dampened_levels) > 0
        })
        .count()
}

#[cfg(test)]
mod tests {

    use crate::day02::{
        parser::parse_reports, safe_count_part1, safe_count_part2, tolerance_rule, trend_rule,
    };

    use super::expand_for_dampening;

    const SAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn parse_raw_reports_into_levels() {
        let reports = parse_reports(SAMPLE);

        assert_eq!(
            reports,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        )
    }

    #[test]
    fn applies_trend_rule() {
        let reports = parse_reports(SAMPLE);

        let actual = reports.iter().map(trend_rule).collect::<Vec<_>>();
        let expected = vec![true, true, true, false, true, true];

        assert_eq!(actual, expected);
    }

    #[test]
    fn applies_tolerance_rule() {
        let reports = parse_reports(SAMPLE);

        let actual = reports.iter().map(tolerance_rule).collect::<Vec<_>>();

        let expected = vec![true, false, false, true, false, true];

        assert_eq!(actual, expected);
    }

    #[test]
    fn solve_part_1_example() {
        let reports = parse_reports(SAMPLE);

        let safe_count = safe_count_part1(reports);

        assert_eq!(safe_count, 2);
    }

    #[test]
    fn solve_part_1() {
        let reports = parse_reports(include_str!("../data/day02.txt"));

        let safe_count = safe_count_part1(reports);

        dbg!(safe_count);
        // panic!();
    }

    #[test]
    fn generates_all_possible_dampened_levels() {
        let reports = parse_reports(SAMPLE);

        let dampened_levels = expand_for_dampening(&reports[0]);

        assert_eq!(
            dampened_levels,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![6, 4, 2, 1],
                vec![7, 4, 2, 1],
                vec![7, 6, 2, 1],
                vec![7, 6, 4, 1],
                vec![7, 6, 4, 2],
            ]
        );
    }

    #[test]
    fn solve_part_2_example() {
        let reports = parse_reports(SAMPLE);

        let safe_count = safe_count_part2(reports);

        assert_eq!(safe_count, 4);
    }

    #[test]
    fn solve_part_2() {
        let reports = parse_reports(include_str!("../data/day02.txt"));

        let safe_count = safe_count_part2(reports);

        dbg!(safe_count);
        // panic!();
    }
}
