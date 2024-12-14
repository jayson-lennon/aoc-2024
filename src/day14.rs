use fxhash::FxHashMap;
use regex::Regex;
use thread_manager::ThreadManager;

use crate::{wrap::WrappingI64, AocSolver};

pub struct Day14Solver;

impl AocSolver for Day14Solver {
    type Output = usize;

    fn part_1(input: &str) -> Self::Output {
        #[cfg(debug_assertions)]
        let dimensions = (7, 11);

        #[cfg(not(debug_assertions))]
        let dimensions = (103, 101);

        let mut bots = SecurityTeam::new(input, dimensions);
        bots.timeshift(100);

        bots.quadrants()
            .into_iter()
            .map(|quadrant| bots.in_quadrant(quadrant))
            .product()
    }

    // Strategy:
    // Spawn some threads to check the map. Map is scanned by looking for contiguous entries of
    // bots (the border). Once we find enough in a row, then we have the pattern.
    fn part_2(_: &str) -> Self::Output {
        let input = include_str!("../data/day14.txt");
        const DIMENSIONS: (i64, i64) = (103, 101);
        const MAX_PATTERN_ROW: usize = DIMENSIONS.0 as usize - 33;

        let thread_manager =
            ThreadManager::<()>::new(num_cpus::get().saturating_sub(1).clamp(1, usize::MAX));

        let (tx, rx) = kanal::unbounded();

        for i in 0..(DIMENSIONS.0 * DIMENSIONS.1) {
            let tx = tx.clone();

            thread_manager.execute(move || {
                let mut bots = SecurityTeam::new(input, DIMENSIONS);
                bots.timeshift(i);

                let map = bots.make_lazy_2d_bot_map();
                for (n_row, row) in map.iter().enumerate() {
                    // patern is 33 units high, so if we get near the bottom then it won't fit
                    // anymore. bail
                    if n_row > MAX_PATTERN_ROW {
                        break;
                    }
                    let mut contiguous = 0;
                    for col in row {
                        match col {
                            1 => contiguous += 1,
                            _ => contiguous = 0,
                        }
                        if contiguous > 8 {
                            let _ = tx.send(i);
                        }
                    }
                }
            });
        }
        match rx.recv() {
            Ok(seconds) => seconds as usize,
            Err(_) => panic!("recv error"),
        }
    }
}

type Wi64 = WrappingI64;
type Pos = (i64, i64);
type Total = usize;
type BotPositions = FxHashMap<Pos, Total>;
type BotMap2d = Vec<Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Rect {
    top_left: Pos,
    bottom_right: Pos,
}

impl Rect {
    #[rustfmt::skip]
    fn contains(&self, pos: Pos) -> bool {
        pos.0 >= self.top_left.0 && pos.0 <= self.bottom_right.0  // row
            && pos.1 >= self.top_left.1 && pos.1 <= self.bottom_right.1 // col
    }
}

impl From<(Pos, Pos)> for Rect {
    fn from((top_left, bottom_right): (Pos, Pos)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}
#[derive(Debug)]
struct Bot {
    pos: (Wi64, Wi64),
    velocity: (i64, i64),
}

impl Bot {
    #[inline(always)]
    fn timeshift(&mut self, seconds: i64) {
        let velocity = (self.velocity.0 * seconds, self.velocity.1 * seconds);
        self.pos.0 += velocity.0;
        self.pos.1 += velocity.1;
    }

    #[inline(always)]
    fn pos_as_i64(&self) -> (i64, i64) {
        (self.pos.0.as_i64(), self.pos.1.as_i64())
    }
}

#[derive(Debug)]
pub struct SecurityTeam {
    inner: Vec<Bot>,
    rows: i64,
    cols: i64,
}

impl SecurityTeam {
    #[allow(dead_code)]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn new(input: &str, (rows, cols): (i64, i64)) -> Self {
        let re_bot = Regex::new(r#"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)"#).unwrap();
        SecurityTeam {
            inner: {
                input
                    .lines()
                    .map(|line| {
                        let caps = re_bot.captures(line).unwrap();
                        let (_, [col, row, v_col, v_row]) = caps.extract();
                        Bot {
                            pos: (
                                Wi64::new(row.to_i64(), (0, rows - 1)),
                                Wi64::new(col.to_i64(), (0, cols - 1)),
                            ),
                            velocity: (v_row.to_i64(), v_col.to_i64()),
                        }
                    })
                    .collect()
            },
            rows,
            cols,
        }
    }

    pub fn timeshift(&mut self, seconds: i64) {
        self.inner.iter_mut().for_each(|bot| {
            bot.timeshift(seconds);
        });
    }

    /// Returns a hashmap that contains bot positions and the number per position
    fn calculate_bot_position_counts(&self) -> BotPositions {
        let mut bots: FxHashMap<(i64, i64), usize> = FxHashMap::default();
        for bot in self.inner.iter() {
            let entry = bots.entry(bot.pos_as_i64()).or_default();
            *entry += 1;
        }
        bots
    }

    /// Create a 2d map where each position has a number indicating the total number of bots
    fn make_2d_bot_map(&self) -> BotMap2d {
        let mut buf = (0..self.rows as usize)
            .map(|_| vec![0; self.cols as usize])
            .collect::<Vec<_>>();

        let positions = self.calculate_bot_position_counts();

        for ((row, col), total) in positions {
            buf[row as usize][col as usize] = total as u8;
        }
        buf
    }

    /// Create a 2d map where each filled position is a 1 and any unfilled position is a 0
    fn make_lazy_2d_bot_map(&self) -> BotMap2d {
        let mut buf = (0..self.rows as usize)
            .map(|_| vec![0; self.cols as usize])
            .collect::<Vec<_>>();

        for bot in &self.inner {
            let (row, col) = bot.pos_as_i64();
            buf[row as usize][col as usize] = 1;
        }
        buf
    }

    /// Returns the quadrants of the map
    pub fn quadrants(&self) -> [Rect; 4] {
        let mid_col = self.cols / 2;
        let mid_row = self.rows / 2;
        [
            // top left
            Rect {
                top_left: (0, 0),
                bottom_right: (mid_row - 1, mid_col - 1),
            },
            // top right
            Rect {
                top_left: (0, mid_col + 1),
                bottom_right: (mid_row - 1, self.cols - 1),
            },
            // bottom left
            Rect {
                top_left: (mid_row + 1, 0),
                bottom_right: (self.rows - 1, mid_col - 1),
            },
            // bottom right
            Rect {
                top_left: (mid_row + 1, mid_col + 1),
                bottom_right: (self.rows - 1, self.cols - 1),
            },
        ]
    }

    pub fn in_quadrant(&self, quadrant: Rect) -> usize {
        self.inner
            .iter()
            .filter(|bot| quadrant.contains(bot.pos_as_i64()))
            .count()
    }
}

impl std::fmt::Display for SecurityTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self.make_2d_bot_map();
        for row in map {
            for col in row {
                let ch = char::from_digit(col as u32, 10).unwrap();
                write!(f, "{}", ch).unwrap();
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

trait Toi64 {
    fn to_i64(&self) -> i64;
}

impl Toi64 for &str {
    fn to_i64(&self) -> i64 {
        self.parse::<i64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day14::SecurityTeam, AocSolver};

    use super::Day14Solver;

    const SAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    const SAMPLE_100: &str = r#"00000020010
00000000000
10000000000
01100000000
00000100000
00012000000
01000010000
"#;

    #[test]
    fn parses() {
        let bots = SecurityTeam::new(SAMPLE, (7, 11));
        assert_eq!(bots.len(), 12);
    }

    #[test]
    fn calculates_robot_positions_after_1_timeshift() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(1);

        assert_eq!(bots.inner.first().unwrap().pos_as_i64(), (1, 3));
        assert_eq!(bots.inner.last().unwrap().pos_as_i64(), (2, 6));
    }

    #[test]
    fn calculates_robot_positions_after_2_timeshifts() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(2);

        assert_eq!(bots.inner.first().unwrap().pos_as_i64(), (5, 6));
        assert_eq!(bots.inner.last().unwrap().pos_as_i64(), (6, 3));
    }

    #[test]
    fn calculates_robot_positions_after_100_timeshifts() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(100);

        assert_eq!(bots.to_string(), SAMPLE_100);
    }

    #[test]
    fn gets_quadrants() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(100);

        assert_eq!(
            bots.quadrants(),
            [
                ((0, 0), (2, 4)).into(),
                ((0, 6), (2, 10)).into(),
                ((4, 0), (6, 4)).into(),
                ((4, 6), (6, 10)).into(),
            ]
        );
    }

    #[test]
    fn counts_bots_in_quadrant() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(100);

        assert_eq!(bots.in_quadrant(((0, 0), (2, 4)).into()), 1);
        assert_eq!(bots.in_quadrant(((0, 6), (2, 10)).into()), 3);
        assert_eq!(bots.in_quadrant(((4, 0), (6, 4)).into()), 4);
        assert_eq!(bots.in_quadrant(((4, 6), (6, 10)).into()), 1);
    }

    #[test]
    fn calculates_safety_factor() {
        let mut bots = SecurityTeam::new(SAMPLE, (7, 11));
        bots.timeshift(100);

        let safety_factor = bots
            .quadrants()
            .into_iter()
            .map(|quadrant| bots.in_quadrant(quadrant))
            .product::<usize>();
        assert_eq!(safety_factor, 12);
    }

    #[test]
    fn solves_part_1() {
        let answer = Day14Solver::part_1(SAMPLE);
        assert_eq!(answer, 12);
    }
}
