//! The 21 Blokus pieces (all free polyominoes of 1..=5 squares) and their
//! precomputed distinct orientations (rotations + reflections, deduplicated).

use std::collections::HashSet;
use std::sync::OnceLock;

pub const NUM_PIECES: usize = 21;

/// Base shapes as (row, col) cells. Order: 1, 2, two triominoes,
/// five tetrominoes, twelve pentominoes.
const SHAPES: [&[(i8, i8)]; NUM_PIECES] = [
    // 1: monomino
    &[(0, 0)],
    // 2: domino
    &[(0, 0), (0, 1)],
    // I3
    &[(0, 0), (0, 1), (0, 2)],
    // V3
    &[(0, 0), (0, 1), (1, 0)],
    // I4
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // O4
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
    // T4
    &[(0, 0), (0, 1), (0, 2), (1, 1)],
    // L4
    &[(0, 0), (1, 0), (2, 0), (2, 1)],
    // S4
    &[(0, 1), (0, 2), (1, 0), (1, 1)],
    // F5
    &[(0, 1), (0, 2), (1, 0), (1, 1), (2, 1)],
    // I5
    &[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
    // L5
    &[(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)],
    // N5
    &[(0, 1), (1, 1), (2, 0), (2, 1), (3, 0)],
    // P5
    &[(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)],
    // T5
    &[(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)],
    // U5
    &[(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)],
    // V5
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    // W5
    &[(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)],
    // X5
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    // Y5
    &[(0, 1), (1, 0), (1, 1), (2, 1), (3, 1)],
    // Z5
    &[(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)],
];

pub const PIECE_NAMES: [&str; NUM_PIECES] = [
    "1", "2", "I3", "V3", "I4", "O4", "T4", "L4", "S4", "F", "I5", "L5", "N", "P", "T5", "U", "V5",
    "W", "X", "Y", "Z",
];

#[derive(Debug)]
pub struct Piece {
    pub size: usize,
    /// Each orientation is normalized (min row/col = 0) and sorted.
    pub orientations: Vec<Vec<(i8, i8)>>,
}

pub fn normalize(cells: &[(i8, i8)]) -> Vec<(i8, i8)> {
    let min_r = cells.iter().map(|c| c.0).min().unwrap();
    let min_c = cells.iter().map(|c| c.1).min().unwrap();
    let mut v: Vec<(i8, i8)> = cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    v.sort_unstable();
    v
}

pub fn rotate_cw(cells: &[(i8, i8)]) -> Vec<(i8, i8)> {
    normalize(&cells.iter().map(|&(r, c)| (c, -r)).collect::<Vec<_>>())
}

pub fn flip_horizontal(cells: &[(i8, i8)]) -> Vec<(i8, i8)> {
    normalize(&cells.iter().map(|&(r, c)| (r, -c)).collect::<Vec<_>>())
}

fn build_pieces() -> Vec<Piece> {
    SHAPES
        .iter()
        .map(|shape| {
            let mut seen: HashSet<Vec<(i8, i8)>> = HashSet::new();
            let mut orientations = Vec::new();
            let mut cur = normalize(shape);
            for _ in 0..2 {
                for _ in 0..4 {
                    if seen.insert(cur.clone()) {
                        orientations.push(cur.clone());
                    }
                    cur = rotate_cw(&cur);
                }
                cur = flip_horizontal(&cur);
            }
            Piece { size: shape.len(), orientations }
        })
        .collect()
}

pub fn pieces() -> &'static [Piece] {
    static PIECES: OnceLock<Vec<Piece>> = OnceLock::new();
    PIECES.get_or_init(build_pieces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_set_matches_official_blokus() {
        let ps = pieces();
        assert_eq!(ps.len(), 21);
        let total_squares: usize = ps.iter().map(|p| p.size).sum();
        assert_eq!(total_squares, 89);
        let count_of = |n| ps.iter().filter(|p| p.size == n).count();
        assert_eq!(count_of(1), 1);
        assert_eq!(count_of(2), 1);
        assert_eq!(count_of(3), 2);
        assert_eq!(count_of(4), 5);
        assert_eq!(count_of(5), 12);
    }

    #[test]
    fn orientation_counts_are_correct() {
        let ps = pieces();
        // Known distinct-orientation counts for a few pieces.
        assert_eq!(ps[0].orientations.len(), 1); // monomino
        assert_eq!(ps[1].orientations.len(), 2); // domino
        assert_eq!(ps[5].orientations.len(), 1); // O4 square
        assert_eq!(ps[18].orientations.len(), 1); // X pentomino
        assert_eq!(ps[9].orientations.len(), 8); // F pentomino
        // Every orientation has the piece's size and is normalized.
        for p in ps {
            for o in &p.orientations {
                assert_eq!(o.len(), p.size);
                assert_eq!(o.iter().map(|c| c.0).min().unwrap(), 0);
                assert_eq!(o.iter().map(|c| c.1).min().unwrap(), 0);
            }
        }
    }
}
