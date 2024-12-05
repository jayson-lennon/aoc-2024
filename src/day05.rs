use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::AocSolver;

pub struct Day5Solver;

impl AocSolver for Day5Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let manual = SafetyManual::from(input);
        let updates = manual.iter_valid_part1_updates().collect::<Vec<_>>();

        let mut sum: u32 = 0;
        for update in updates {
            let mid_index = update.len() / 2;
            sum += update[mid_index] as u32;
        }
        sum
    }

    fn part_2(input: &str) -> Self::Output {
        0
    }
}

type Rules = HashMap<u8, Vec<u8>>;
type Updates = Vec<Vec<u8>>;

#[derive(Debug, Clone)]
struct Page {
    /// This page number
    number: u8,
    /// Pages that must be printed after this page
    follows: Vec<u8>,
}

impl Page {
    fn empty(page: u8) -> Self {
        Self {
            number: page,
            follows: Vec::default(),
        }
    }
}

impl Eq for Page {}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        // When this page exists in the other `follows` vec, then this means this page is
        // ordered after the other.
        if other.follows.contains(&self.number) {
            Ordering::Greater
        } else if self.follows.contains(&other.number) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
struct SafetyManual {
    rules: Rules,
    updates: Updates,
}

impl SafetyManual {
    fn rules(&self) -> &Rules {
        &self.rules
    }
    fn updates(&self) -> &Updates {
        &self.updates
    }

    /// Returns part1 valid updates
    fn iter_valid_part1_updates(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.updates.iter().filter(|pages| {
            let mut order_buf: Vec<u8> = Vec::default();
            for page in pages.iter() {
                if order_buf.contains(page) {
                    return false;
                }
                order_buf.push(*page);
                if let Some(follows) = self.rules.get(page) {
                    for p in follows {
                        if order_buf.contains(p) {
                            return false;
                        }
                    }
                }
            }
            true
        })
    }

    fn iter_incorrectly_sorted(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.updates.iter().filter(|pages| {
            let mut order_buf: Vec<u8> = Vec::default();
            for page in pages.iter() {
                if order_buf.contains(page) {
                    return false;
                }
                order_buf.push(*page);
                if let Some(follows) = self.rules.get(page) {
                    for p in follows {
                        if order_buf.contains(p) {
                            return true;
                        }
                    }
                }
            }
            false
        })
    }

    fn sort(&self, incorrect_updates: &[u8]) -> Vec<Page> {
        let pages = self
            .rules
            .iter()
            .map(|(page, follows)| {
                (
                    *page,
                    Page {
                        number: *page,
                        follows: follows.to_owned(),
                    },
                )
            })
            .collect::<HashMap<u8, Page>>();

        let mut pages = incorrect_updates
            .iter()
            .map(|n| pages.get(n).cloned().unwrap_or_else(|| Page::empty(*n)))
            .collect::<Vec<_>>();
        pages.sort();
        pages
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseType {
    Rule,
    PrintOrder,
}

impl From<&str> for SafetyManual {
    fn from(value: &str) -> Self {
        use ParseType::*;

        let mut rules = HashMap::default();
        let mut updates = Vec::default();
        let mut parsing = ParseType::Rule;

        for line in value.lines() {
            if line.is_empty() {
                parsing = PrintOrder;
                continue;
            }
            match parsing {
                Rule => {
                    let (page, follows) = {
                        let (page, follows) = line.split_once('|').unwrap();
                        (page.parse::<u8>().unwrap(), follows.parse::<u8>().unwrap())
                    };
                    let entry: &mut Vec<u8> = rules.entry(page).or_default();
                    entry.push(follows);
                }
                PrintOrder => {
                    updates.push(line.split(',').map(|n| n.parse::<u8>().unwrap()).collect());
                }
            }
        }
        SafetyManual { rules, updates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn parses_into_safety_manual() {
        let manual = SafetyManual::from(SAMPLE);

        assert_eq!(manual.rules().len(), 6);
        assert_eq!(manual.updates().len(), 6);

        assert_eq!(manual.rules()[&47], &[53, 13, 61, 29]);
        assert_eq!(manual.updates()[0], &[75, 47, 61, 53, 29]);
    }

    #[test]
    fn get_correctly_ordered_updates() {
        let manual = SafetyManual::from(SAMPLE);

        let updates = manual.iter_valid_part1_updates().collect::<Vec<_>>();

        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0], &[75, 47, 61, 53, 29]);
    }

    #[test]
    fn solves_part_1() {
        let answer = Day5Solver::part_1(SAMPLE);

        assert_eq!(answer, 143);
    }

    #[test]
    fn get_incorrectly_sorted_entries() {
        let manual = SafetyManual::from(SAMPLE);

        let incorrect_updates = manual.iter_incorrectly_sorted().collect::<Vec<_>>();
        assert_eq!(incorrect_updates.len(), 3);
        assert_eq!(incorrect_updates[0], &[75, 97, 47, 61, 53]);
    }

    #[test]
    fn applies_sort() {
        let manual = SafetyManual::from(SAMPLE);

        let incorrect_updates = manual.iter_incorrectly_sorted().collect::<Vec<_>>();

        let sorted = incorrect_updates
            .into_iter()
            .map(|pages| {
                manual
                    .sort(pages)
                    .into_iter()
                    .map(|page| page.number)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(sorted[0], vec![97, 75, 47, 61, 53]);
        assert_eq!(sorted[1], vec![61, 29, 13]);
        assert_eq!(sorted[2], vec![97, 75, 47, 29, 13]);
    }
}

// pages to print: collect ordering rules
