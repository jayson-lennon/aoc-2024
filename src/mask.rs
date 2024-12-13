use std::ops::{Index, IndexMut};

use crate::grid::Pos2;

#[derive(Debug)]
pub struct Mask2D {
    mask: Vec<Vec<u8>>,
    border: usize,
}

impl Mask2D {
    pub fn from_positions(positions: &[Pos2], border: usize) -> Self {
        let (row_len, col_len) = {
            let row_len = positions.iter().map(|pos| pos.row).max().unwrap() + 1;
            let col_len = positions.iter().map(|pos| pos.col).max().unwrap() + 1;
            (row_len as usize, col_len as usize)
        };

        let mut mask: Vec<Vec<u8>> = Vec::with_capacity(row_len + (border * 2));

        for _ in 0..row_len + (border * 2) {
            mask.push(vec![0; col_len + (border * 2)])
        }

        let mut mask = Mask2D { mask, border };

        for pos in positions {
            mask[pos] = 1;
        }

        mask
    }

    pub fn get<P>(&self, pos: P) -> Option<u8>
    where
        P: Into<Pos2>,
    {
        let pos = pos.into();
        let (row, col) = {
            let Pos2 { row, col } = pos;
            (col as usize + self.border, row as usize + self.border)
        };

        if row >= self.mask.len() || col >= self.mask[0].len() {
            None
        } else {
            Some(self[Pos2::from((row as isize, col as isize))])
        }
    }
}

impl IndexMut<&Pos2> for Mask2D {
    fn index_mut(&mut self, Pos2 { row, col }: &Pos2) -> &mut Self::Output {
        let border = self.border;
        &mut self.mask[*row as usize + border][*col as usize + border]
    }
}

impl IndexMut<Pos2> for Mask2D {
    fn index_mut(&mut self, Pos2 { row, col }: Pos2) -> &mut Self::Output {
        let border = self.border;
        &mut self.mask[row as usize + border][col as usize + border]
    }
}

impl Index<&Pos2> for Mask2D {
    type Output = u8;

    fn index(&self, Pos2 { row, col }: &Pos2) -> &Self::Output {
        let border = self.border;
        &self.mask[*row as usize + border][*col as usize + border]
    }
}

impl Index<Pos2> for Mask2D {
    type Output = u8;

    fn index(&self, Pos2 { row, col }: Pos2) -> &Self::Output {
        let border = self.border;
        let row = row + border as isize;
        let col = col + border as isize;
        &self.mask[row as usize][col as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_mask_from_positions() {
        let positions = pos(&[(1, 2), (3, 4)]);

        let mask = Mask2D::from_positions(&positions, 0);

        assert_eq!(
            mask.mask,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
            ]
        )
    }

    #[test]
    fn creates_mask_with_padding() {
        let positions = pos(&[(0, 0), (2, 2)]);

        let mask = Mask2D::from_positions(&positions, 1);

        assert_eq!(
            mask.mask,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0],
            ]
        )
    }

    fn pos(positions: &[(isize, isize)]) -> Vec<Pos2> {
        positions
            .iter()
            .map(|pos| Pos2::from(*pos))
            .collect::<Vec<_>>()
    }

    #[test]
    fn get_element() {
        // 0 1 0
        // 1 1 0
        // 0 0 1
        let positions = pos(&[(0, 1), (1, 0), (1, 1), (2, 2)]);

        let mask = Mask2D::from_positions(&positions, 0);

        assert_eq!(mask.get((0, 1)), Some(1));
        assert_eq!(mask.get((0, 0)), Some(0));
    }

    #[test]
    fn creates_mask_with_offset_element() {
        let positions = pos(&[(1, 2), (3, 4)]);

        let mask = Mask2D::from_positions(&positions, 0);

        assert_eq!(
            mask.mask,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
            ]
        )
    }
}
