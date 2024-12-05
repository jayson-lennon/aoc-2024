use std::{cmp::Ordering, collections::HashMap};

use crate::AocSolver;

pub struct Day5Solver;

// Strategy: Implement PartialEq, Eq, PartialOrd, and Ord using the page ordering rules.
// Then use std sort.

impl AocSolver for Day5Solver {
    type Output = u32;

    fn part_1(input: &str) -> Self::Output {
        let manual = SafetyManual::from(input);
        let updates = manual.iter_updates(Sorted::Correctly);

        median(updates)
    }

    fn part_2(input: &str) -> Self::Output {
        let manual = SafetyManual::from(input);
        let updates = manual.iter_updates(Sorted::Incorrectly).map(|update| {
            let mut update = update.clone();
            update.sort();
            update
        });

        median(updates)
    }
}

fn median(updates: impl Iterator<Item = Vec<Page>>) -> u32 {
    let mut sum: u32 = 0;
    for update in updates {
        let mid_index = update.len() / 2;
        sum += update[mid_index].number as u32;
    }
    sum
}

type Rules = HashMap<u8, Page>;

#[derive(Debug, Clone)]
struct Page {
    /// This page number
    number: u8,
    /// Pages that must be printed after this page
    follows: Vec<u8>,
}

impl Page {
    /// Create a new empty page. Used as a default if there are no rules applied to the page.
    fn empty(page: u8) -> Self {
        Self {
            number: page,
            follows: Vec::default(),
        }
    }
}

// Implement std stuff

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

/// For the `iter_updates` method. Determines if we want to get back only sorted or only unsorted
/// entries
#[derive(Clone, Copy)]
enum Sorted {
    Correctly,
    Incorrectly,
}

/// Contains the rules and updates parsed from the problem input.
#[derive(Debug)]
struct SafetyManual {
    rules: Rules,
    updates: Vec<Vec<u8>>,
}

impl SafetyManual {
    /// Returns an iterator over updates in `Page` format so they can be sorted later.
    fn iter_updates(&self, sorted: Sorted) -> impl Iterator<Item = Vec<Page>> + '_ {
        use Sorted::*;
        self.updates.iter().filter_map(move |pages| {
            let update = pages
                .iter()
                .map(|page| {
                    self.rules
                        .get(page)
                        .cloned()
                        .unwrap_or_else(|| Page::empty(*page))
                })
                .collect::<Vec<_>>();
            match sorted {
                Correctly => {
                    if update.is_sorted() {
                        Some(update)
                    } else {
                        None
                    }
                }
                Incorrectly => {
                    if !update.is_sorted() {
                        Some(update)
                    } else {
                        None
                    }
                }
            }
        })
    }
}

/// Parser helper
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
                    let entry = rules.entry(page).or_insert_with(|| Page::empty(page));
                    entry.follows.push(follows);
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
    fn get_correctly_ordered_updates() {
        let manual = SafetyManual::from(SAMPLE);

        let updates = manual
            .iter_updates(Sorted::Correctly)
            .map(|update| {
                update
                    .into_iter()
                    .map(|page| page.number)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0], &[75, 47, 61, 53, 29]);
        assert_eq!(updates[1], &[97, 61, 53, 29, 13]);
        assert_eq!(updates[2], &[75, 29, 13]);
    }

    #[test]
    fn solves_part_1() {
        let answer = Day5Solver::part_1(SAMPLE);

        assert_eq!(answer, 143);
    }

    #[test]
    fn get_incorrectly_ordered_updates() {
        let manual = SafetyManual::from(SAMPLE);

        let updates = manual
            .iter_updates(Sorted::Incorrectly)
            .map(|update| {
                update
                    .into_iter()
                    .map(|page| page.number)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0], &[75, 97, 47, 61, 53]);
        assert_eq!(updates[1], &[61, 13, 29]);
        assert_eq!(updates[2], &[97, 13, 75, 29, 47]);
    }

    #[test]
    fn solves_part_2() {
        let answer = Day5Solver::part_2(SAMPLE);
        assert_eq!(answer, 123);
    }
}
