fn split_lists(data: &str) -> (Vec<u32>, Vec<u32>) {
    let (a, b): (Vec<_>, Vec<_>) = data
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (a.trim(), (b.trim())))
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();
    (a, b)
}

fn total_dist(a: Vec<u32>, b: Vec<u32>) -> u32 {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

fn similarity(a: Vec<u32>, b: Vec<u32>) -> usize {
    let mut similarity: usize = 0;
    for n in a {
        let total_matches = b.iter().filter(|b| b == &&n).count();
        similarity += n as usize * total_matches;
    }
    similarity
}

#[cfg(test)]
mod tests {

    use crate::day01::{similarity, total_dist};

    const SAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn splits_lists() {
        let (a, b) = super::split_lists(SAMPLE);

        assert_eq!(a, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(b, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn calculates_total_distance() {
        let (mut a, mut b) = super::split_lists(SAMPLE);
        a.sort();
        b.sort();

        let answer = total_dist(a, b);

        assert_eq!(answer, 11);
    }

    #[test]
    fn answer_part_1() {
        let data = include_str!("../data/day01.txt");
        let (mut a, mut b) = super::split_lists(data);
        a.sort();
        b.sort();

        let answer = total_dist(a, b);

        dbg!(answer);
        // panic!();
    }

    #[test]
    fn calculate_similarity() {
        let (a, b) = super::split_lists(SAMPLE);

        assert_eq!(similarity(a, b), 31);
    }

    #[test]
    fn answer_part_2() {
        let data = include_str!("../data/day01.txt");
        let (a, b) = super::split_lists(data);

        let answer = similarity(a, b);

        dbg!(answer);
        // panic!();
    }
}
