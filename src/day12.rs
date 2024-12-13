use fxhash::FxHashSet;
use rayon::prelude::*;

use crate::{
    grid::{Grid2D, Pos2, Query},
    mask::Mask2D,
    AocSolver,
};

pub struct Day12Solver;

impl AocSolver for Day12Solver {
    type Output = u64;

    // Strategy:
    // 1. get all plant types (letters)
    // 2. for each plant type, find all the plots (same types that are adjacent). this is done 1
    //    plant type per thread
    //    - finding the plots uses a recursive nightmare that walks the plants until no more
    //      adjacent ones are found
    //    - once a plot is found, a loop removes it from a set of all plants. this is so we know
    //      what other plots need to be walked
    // 3. (in thread): calculate the area and perimeter
    //   - area is the total number of plants in the plot
    //   - perimeter is calculated using a grid query that checks if there is an adjacent plant in
    //     the plot. if none, then thats fence+1
    // 4. sum all the results from above
    fn part_1(input: &str) -> Self::Output {
        let garden = Garden::from(input);
        garden
            .plants()
            .par_iter()
            .map(|plant| {
                garden
                    .find_plots(*plant)
                    .iter()
                    .map(|plot| {
                        let area = plot.area();
                        let perimeter = plot.perimeter(&garden);
                        area * perimeter
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    // Strategy:
    // 1. do part one up until the perimeter calculation
    // 2. create a mask from each plot
    // 3. compare each plant in the plot mask with a known pattern that identifies a corner
    // 4. for each matched corner, increment the "unique sides" by 1
    // 5. sum the results
    fn part_2(input: &str) -> Self::Output {
        let garden = Garden::from(input);
        garden
            .plants()
            .iter()
            .map(|plant| {
                garden
                    .find_plots(*plant)
                    .iter()
                    .map(|plot| {
                        let area = plot.area();
                        let mask = Mask2D::from_positions(&plot.plants, 1);
                        let sides = find_sides(plot, mask);
                        area * sides
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

fn find_sides(plot: &Plot, mask: Mask2D) -> u64 {
    let mut sides = 0;
    for plant in plot {
        let mut this_sides = 0;
        this_sides += Corner1.n_corners(*plant, &mask);
        this_sides += Corner2.n_corners(*plant, &mask);
        this_sides += Corner3.n_corners(*plant, &mask);
        this_sides += Corner4.n_corners(*plant, &mask);
        sides += this_sides;
    }
    sides
}

trait CornerFinder {
    fn n_corners(&self, plant: Pos2, mask: &Mask2D) -> u64;
}

struct Corner1;
struct Corner2;
struct Corner3;
struct Corner4;

impl CornerFinder for Corner1 {
    #[rustfmt::skip]
    fn n_corners(&self, plant: Pos2, mask: &Mask2D) -> u64 {
        // x x
        // . x
        let coords = [(-1, 0), (-1, 1), (0, 1)];

        let (this, a, b, c) = extract_submask(mask, plant, coords);

        let pat1 = match (a, b, this, c) {
            (0, 0,
             _, 0) => 1,

            _ => 0
        };

        let pat2 = match (a, b, this, c) {
            (1, 0,
             _, 1) => 1,

            _ => 0
        };

        pat1 + pat2
    }
}

impl CornerFinder for Corner2 {
    #[rustfmt::skip]
    fn n_corners(&self, plant: Pos2, mask: &Mask2D) -> u64 {
        // x x
        // x .
        let coords = [(0, -1), (-1, -1), (-1, 0)];

        let (this, a, b, c) = extract_submask(mask, plant, coords);

        let pat1 = match (b, c, a, this) {
            (0, 0,
             0, _) => 1,

            _ => 0
        };
        let pat2 = match (b, c, a, this) {
            (0, 1,
             1, _) => 1,

            _ => 0
        };

        pat1 + pat2
    }
}

impl CornerFinder for Corner3 {
    #[rustfmt::skip]
    fn n_corners(&self, plant: Pos2, mask: &Mask2D) -> u64 {
        // x .
        // x x
        let coords = [(1, 0), (1, -1), (0, -1)];

        let (this, a, b, c) = extract_submask(mask, plant, coords);

        let pat1 = match (c, this, b, a) {
            (0, _,
             0, 0) => 1,

            _ => 0
        };

        let pat2 = match (c, this, b, a) {
            (1, _,
             0, 1) => 1,

            _ => 0
        };

        pat1 + pat2
    }
}

impl CornerFinder for Corner4 {
    #[rustfmt::skip]
    fn n_corners(&self, plant: Pos2, mask: &Mask2D) -> u64 {
        // . x
        // x x
        let coords = [(0, 1), (1, 1), (1, 0)];

        let (this, a, b, c) = extract_submask(mask, plant, coords);

        let pat1 = match (this, a, c, b) {
            (_, 0,
             0, 0) => 1,

            _ => 0
        };

        let pat2 = match (this, a, c, b) {
            (_, 1,
             1, 0) => 1,

            _ => 0
        };

        pat1 + pat2
    }
}

fn extract_submask(mask: &Mask2D, plant: Pos2, coords: [(isize, isize); 3]) -> (u8, u8, u8, u8) {
    let this = mask[plant];
    let a = mask[plant + Pos2::from(coords[0])];
    let b = mask[plant + Pos2::from(coords[1])];
    let c = mask[plant + Pos2::from(coords[2])];
    (this, a, b, c)
}

type Plant = char;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Plot {
    plants: Vec<Pos2>,
    kind: Plant,
}

impl Plot {
    pub fn area(&self) -> u64 {
        self.plants.len() as u64
    }

    pub fn perimeter(&self, garden: &Garden) -> u64 {
        use grid_query::Fencing;

        self.plants
            .iter()
            .map(|pos| garden.query(Fencing { plant: self.kind }, *pos))
            .sum()
    }
}

impl<'a> IntoIterator for &'a Plot {
    type Item = &'a Pos2;
    type IntoIter = std::slice::Iter<'a, Pos2>;

    fn into_iter(self) -> Self::IntoIter {
        self.plants.iter()
    }
}

impl IntoIterator for Plot {
    type Item = Pos2;
    type IntoIter = std::vec::IntoIter<Pos2>;

    fn into_iter(self) -> Self::IntoIter {
        self.plants.into_iter()
    }
}

struct Garden {
    garden: Grid2D,
}

impl Garden {
    // Get all plots for the specified plant
    fn find_plots(&self, plant: Plant) -> Vec<Plot> {
        use grid_query::{find_plot_impl, PlantKind};

        // get coordinates of all of the target plant
        let mut all_plants: FxHashSet<Pos2> = self
            .garden
            .find_all_iter(PlantKind(plant))
            .map(|(pos, _)| pos)
            .collect::<FxHashSet<_>>();

        // contains all plots for this plant
        let mut plots = Vec::default();

        loop {
            // get an available plant coordinate
            let next = all_plants.iter().next().cloned();

            if let Some(next) = next {
                // the plot containing the `next` coordinate is located and saved into `plot`
                let mut plot: FxHashSet<Pos2> = FxHashSet::default();
                find_plot_impl(&self.garden, &mut plot, plant, next);

                {
                    // when the length of the all_plants set doesn't change, then that means we are
                    // done.
                    let current_len = all_plants.len();

                    // we remove the found plot from all known plant coordinates
                    all_plants = all_plants.symmetric_difference(&plot).copied().collect();

                    // bail if we the set size didn't change (all plants already visited)
                    if all_plants.len() == current_len {
                        break;
                    }
                }

                plots.push(Plot {
                    plants: plot.into_iter().collect(),
                    kind: plant,
                });
            } else {
                // all plots founds
                break;
            }
        }

        plots
    }

    pub fn plants(&self) -> FxHashSet<Plant> {
        self.garden.unique()
    }

    pub fn query<Q>(&self, query: Q, pos: Pos2) -> Q::Output
    where
        Q: Query,
    {
        self.garden.query(query, pos)
    }
}

mod grid_query {

    use super::Plant;
    use crate::grid::{Direction, Finder, Grid2D, Pos2, Query};
    use fxhash::FxHashSet;
    use smallvec::SmallVec;

    pub struct AdjacentPlant(pub Plant);

    impl Query for AdjacentPlant {
        type Output = SmallVec<[Pos2; 4]>;

        fn query(&mut self, grid: &Grid2D, pos: Pos2) -> Self::Output {
            let mut adjacent = SmallVec::default();

            // up, down, left, right
            for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let check_pos = pos + Direction::from(direction);
                if let Some(next) = grid.get(check_pos) {
                    if next == self.0 {
                        adjacent.push(check_pos);
                    }
                }
            }

            adjacent
        }
    }

    /// Recursive plot finder.
    pub fn find_plot_impl(
        grid: &Grid2D,
        visited: &mut FxHashSet<Pos2>,
        plant: char,
        current: Pos2,
    ) {
        let adjacent = grid.query(AdjacentPlant(plant), current);

        if adjacent.is_empty() {
            visited.insert(current);
            return;
        }

        for pos in adjacent {
            if visited.insert(pos) {
                find_plot_impl(grid, visited, plant, pos);
            }
        }
    }

    /// Grid query to calculate the fencing of a specific position.
    pub struct Fencing {
        pub plant: char,
    }

    impl Query for Fencing {
        type Output = u64;

        #[allow(clippy::collapsible_else_if)]
        fn query(&mut self, grid: &Grid2D, pos: Pos2) -> Self::Output {
            let mut total = 0;

            // up, down, left, right
            for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let check_pos = pos + Direction::from(direction);
                if !grid.on_grid(check_pos) {
                    total += 1;
                } else {
                    if grid[check_pos] != self.plant {
                        total += 1;
                    }
                }
            }

            total
        }
    }

    #[derive(Clone)]
    pub struct PlantKind(pub char);

    impl Finder for PlantKind {
        fn check(&self, ch: char) -> bool {
            ch == self.0
        }
    }
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        Self {
            garden: Grid2D::from(value),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const SAMPLE_2: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn parses() {
        let garden = Garden::from(SAMPLE_1);
        assert_eq!(garden.garden.dim().rows(), 4);
        assert_eq!(garden.garden.dim().cols(), 4);
    }

    #[test]
    fn finds_perimeter_with_1_plot() {
        let garden = Garden::from(SAMPLE_1);
        let plots = garden.find_plots('A');
        assert_eq!(plots[0].perimeter(&garden), 10);

        let plots = garden.find_plots('C');
        assert_eq!(plots[0].perimeter(&garden), 10);
    }

    #[test]
    fn finds_area_with_1_plot() {
        let garden = Garden::from(SAMPLE_1);
        let plots = garden.find_plots('A');
        assert_eq!(plots[0].area(), 4);

        let garden = Garden::from(SAMPLE_2);
        let plots = garden.find_plots('I');

        let mut areas = plots.iter().map(|plot| plot.area()).collect::<Vec<_>>();
        areas.sort();

        assert_eq!(areas[0], 4);
        assert_eq!(areas[1], 14);
    }

    #[test]
    fn finds_area_with_2_plots_of_the_same_plant_type() {
        let garden = Garden::from(SAMPLE_2);
        let plots = garden.find_plots('I');

        let mut areas = plots.iter().map(|plot| plot.area()).collect::<Vec<_>>();
        areas.sort();

        assert_eq!(areas[0], 4);
        assert_eq!(areas[1], 14);
    }

    #[test]
    fn finds_plot_when_plot_size_is_1() {
        let garden = Garden::from(SAMPLE_1);

        let plot = garden.find_plots('D');

        assert_eq!(
            plot[0],
            Plot {
                plants: vec![Pos2::from((1, 3))],
                kind: 'D'
            }
        );
    }

    #[test]
    fn finds_plot() {
        let garden = Garden::from(SAMPLE_1);

        let mut plot = garden.find_plots('B');
        let expected = [(1, 0), (1, 1), (2, 0), (2, 1)]
            .iter()
            .map(|pos| Pos2::from(*pos))
            .collect::<Vec<_>>();

        let mut actual = plot.remove(0);
        actual.plants.sort();

        assert_eq!(
            actual,
            Plot {
                plants: expected,
                kind: 'B'
            }
        );
    }

    #[test]
    fn solves_part_1() {
        let answer = Day12Solver::part_1(SAMPLE_1);
        assert_eq!(answer, 140);

        let answer = Day12Solver::part_1(SAMPLE_2);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn solves_part_2() {
        let answer = Day12Solver::part_2(SAMPLE_1);
        assert_eq!(answer, 80);

        let answer = Day12Solver::part_2(SAMPLE_2);
        assert_eq!(answer, 1206);
    }
}
