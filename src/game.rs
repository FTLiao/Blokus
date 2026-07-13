//! Blokus rules engine on the official 20x20 board.
//!
//! Board state is kept as per-player bitboards: one u32 per row, bit `c` set
//! means column `c` of that row is occupied by that player.

use crate::pieces::{NUM_PIECES, pieces};
use std::collections::HashSet;

pub const N: usize = 20;
pub const ROW_MASK: u32 = (1 << N) - 1;
pub const TOTAL_SQUARES: i32 = 89;

pub type Bits = [u32; N];

/// Starting corners in play order: Blue, Yellow, Red, Green.
pub const START_CORNERS: [(usize, usize); 4] = [(0, 0), (0, N - 1), (N - 1, N - 1), (N - 1, 0)];
pub const PLAYER_NAMES: [&str; 4] = ["Blue", "Yellow", "Red", "Green"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub piece: usize,
    pub orient: usize,
    pub row: i32,
    pub col: i32,
}

impl Move {
    pub fn cells(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        pieces()[self.piece].orientations[self.orient]
            .iter()
            .map(move |&(r, c)| (self.row + r as i32, self.col + c as i32))
    }
}

/// Cells orthogonally adjacent to any set bit (includes the bits themselves).
fn orth_expand(b: &Bits) -> Bits {
    let mut out = [0u32; N];
    for r in 0..N {
        let row = b[r];
        out[r] |= row | (row << 1) | (row >> 1);
        if r > 0 {
            out[r - 1] |= row;
        }
        if r + 1 < N {
            out[r + 1] |= row;
        }
    }
    for row in &mut out {
        *row &= ROW_MASK;
    }
    out
}

/// Cells diagonally adjacent to any set bit.
fn diag_expand(b: &Bits) -> Bits {
    let mut out = [0u32; N];
    for r in 0..N {
        let spread = ((b[r] << 1) | (b[r] >> 1)) & ROW_MASK;
        if r > 0 {
            out[r - 1] |= spread;
        }
        if r + 1 < N {
            out[r + 1] |= spread;
        }
    }
    out
}

fn count_bits(b: &Bits) -> u32 {
    b.iter().map(|r| r.count_ones()).sum()
}

#[derive(Clone)]
pub struct GameState {
    pub occ: [Bits; 4],
    /// Bitmask over the 21 pieces still in hand.
    pub pieces_left: [u32; 4],
    pub current: usize,
    /// False once a player can no longer place anything (permanent in Blokus).
    pub active: [bool; 4],
    pub last_piece: [Option<usize>; 4],
    pub last_move: [Option<Move>; 4],
    pub round: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            occ: [[0; N]; 4],
            pieces_left: [(1 << NUM_PIECES) - 1; 4],
            current: 0,
            active: [true; 4],
            last_piece: [None; 4],
            last_move: [None; 4],
            round: 0,
        }
    }

    pub fn union(&self) -> Bits {
        let mut out = [0u32; N];
        for p in 0..4 {
            for r in 0..N {
                out[r] |= self.occ[p][r];
            }
        }
        out
    }

    pub fn occupant(&self, r: usize, c: usize) -> Option<usize> {
        (0..4).find(|&p| self.occ[p][r] >> c & 1 == 1)
    }

    pub fn is_first_move(&self, p: usize) -> bool {
        self.occ[p].iter().all(|&row| row == 0)
    }

    pub fn has_piece(&self, p: usize, piece: usize) -> bool {
        self.pieces_left[p] >> piece & 1 == 1
    }

    /// Cells a piece of player `p` may never cover: occupied cells plus cells
    /// orthogonally adjacent to p's own color.
    pub fn forbidden(&self, p: usize) -> Bits {
        let mut out = orth_expand(&self.occ[p]);
        let all = self.union();
        for r in 0..N {
            out[r] |= all[r];
        }
        out
    }

    /// Empty cells where player `p` could establish the required diagonal
    /// contact (the "seeds" every legal placement must cover at least one of).
    /// On the first move this is the player's starting corner.
    pub fn seeds(&self, p: usize) -> Bits {
        let mut out = [0u32; N];
        if self.is_first_move(p) {
            let (r, c) = START_CORNERS[p];
            if self.occupant(r, c).is_none() {
                out[r] = 1 << c;
            }
            return out;
        }
        let diag = diag_expand(&self.occ[p]);
        let forbidden = self.forbidden(p);
        for r in 0..N {
            out[r] = diag[r] & !forbidden[r] & ROW_MASK;
        }
        out
    }

    pub fn seed_count(&self, p: usize) -> u32 {
        count_bits(&self.seeds(p))
    }

    pub fn move_is_legal(&self, p: usize, mv: &Move) -> bool {
        if !self.has_piece(p, mv.piece) {
            return false;
        }
        let forbidden = self.forbidden(p);
        let seeds = self.seeds(p);
        let mut touches = false;
        for (r, c) in mv.cells() {
            if r < 0 || c < 0 || r >= N as i32 || c >= N as i32 {
                return false;
            }
            let (r, c) = (r as usize, c as usize);
            if forbidden[r] >> c & 1 == 1 {
                return false;
            }
            if seeds[r] >> c & 1 == 1 {
                touches = true;
            }
        }
        touches
    }

    /// All legal moves for player `p`.
    pub fn legal_moves(&self, p: usize) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut seen: HashSet<(usize, usize, i32, i32)> = HashSet::new();
        let forbidden = self.forbidden(p);
        let seeds = self.seeds(p);
        let seed_list: Vec<(i32, i32)> = (0..N)
            .flat_map(|r| (0..N).filter(move |&c| seeds[r] >> c & 1 == 1).map(move |c| (r as i32, c as i32)))
            .collect();
        if seed_list.is_empty() {
            return moves;
        }
        let ps = pieces();
        for piece in 0..NUM_PIECES {
            if !self.has_piece(p, piece) {
                continue;
            }
            for (oi, orient) in ps[piece].orientations.iter().enumerate() {
                for &(sr, sc) in &seed_list {
                    // Try aligning each cell of the piece onto the seed.
                    'anchor: for &(ar, ac) in orient {
                        let (row, col) = (sr - ar as i32, sc - ac as i32);
                        if !seen.insert((piece, oi, row, col)) {
                            continue;
                        }
                        for &(r, c) in orient {
                            let (r, c) = (row + r as i32, col + c as i32);
                            if r < 0 || c < 0 || r >= N as i32 || c >= N as i32 {
                                continue 'anchor;
                            }
                            if forbidden[r as usize] >> c & 1 == 1 {
                                continue 'anchor;
                            }
                        }
                        moves.push(Move { piece, orient: oi, row, col });
                    }
                }
            }
        }
        moves
    }

    pub fn has_any_move(&self, p: usize) -> bool {
        if self.pieces_left[p] == 0 {
            return false;
        }
        !self.legal_moves(p).is_empty()
    }

    pub fn apply(&mut self, p: usize, mv: &Move) {
        debug_assert!(self.move_is_legal(p, mv));
        for (r, c) in mv.cells() {
            self.occ[p][r as usize] |= 1 << c;
        }
        self.pieces_left[p] &= !(1 << mv.piece);
        self.last_piece[p] = Some(mv.piece);
        self.last_move[p] = Some(*mv);
    }

    /// Advance to the next player that can move, permanently deactivating any
    /// player found stuck along the way. Returns the players that got knocked
    /// out this step; `self.is_over()` tells whether the game ended.
    pub fn advance_turn(&mut self) -> Vec<usize> {
        let mut knocked_out = Vec::new();
        for _ in 0..4 {
            self.current = (self.current + 1) % 4;
            if self.current == 0 {
                self.round += 1;
            }
            if !self.active[self.current] {
                continue;
            }
            if self.has_any_move(self.current) {
                return knocked_out;
            }
            self.active[self.current] = false;
            knocked_out.push(self.current);
        }
        knocked_out
    }

    pub fn is_over(&self) -> bool {
        self.active.iter().all(|a| !a)
    }

    pub fn squares_remaining(&self, p: usize) -> i32 {
        let ps = pieces();
        (0..NUM_PIECES)
            .filter(|&i| self.has_piece(p, i))
            .map(|i| ps[i].size as i32)
            .sum()
    }

    /// Official Blokus score: -1 per unplaced square; +15 bonus for placing
    /// all 21 pieces, +5 more if the monomino was placed last.
    pub fn score(&self, p: usize) -> i32 {
        let remaining = self.squares_remaining(p);
        if remaining == 0 {
            if self.last_piece[p] == Some(0) { 20 } else { 15 }
        } else {
            -remaining
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_is_20_by_20() {
        assert_eq!(N, 20);
        let gs = GameState::new();
        // All four starting corners are inside the board.
        for &(r, c) in &START_CORNERS {
            assert!(r < N && c < N);
        }
        assert_eq!(gs.squares_remaining(0), TOTAL_SQUARES);
    }

    #[test]
    fn first_move_must_cover_corner() {
        let gs = GameState::new();
        // Monomino on Blue's corner (0,0) is legal.
        assert!(gs.move_is_legal(0, &Move { piece: 0, orient: 0, row: 0, col: 0 }));
        // Anywhere else is not.
        assert!(!gs.move_is_legal(0, &Move { piece: 0, orient: 0, row: 5, col: 5 }));
        // Every legal first move covers the corner.
        for mv in gs.legal_moves(0) {
            assert!(mv.cells().any(|(r, c)| (r, c) == (0, 0)));
        }
    }

    #[test]
    fn same_color_diagonal_only() {
        let mut gs = GameState::new();
        // Blue plays domino at corner: covers (0,0),(0,1).
        let first = Move { piece: 1, orient: 0, row: 0, col: 0 };
        assert!(gs.move_is_legal(0, &first));
        gs.apply(0, &first);
        // Edge-adjacent placement of monomino at (0,2) is illegal...
        assert!(!gs.move_is_legal(0, &Move { piece: 0, orient: 0, row: 0, col: 2 }));
        // ...but diagonal at (1,2) is legal.
        assert!(gs.move_is_legal(0, &Move { piece: 0, orient: 0, row: 1, col: 2 }));
        // Overlap is illegal.
        assert!(!gs.move_is_legal(0, &Move { piece: 0, orient: 0, row: 0, col: 0 }));
        // Reusing a spent piece is illegal.
        assert!(!gs.move_is_legal(0, &Move { piece: 1, orient: 0, row: 1, col: 2 }));
    }

    #[test]
    fn other_color_edge_contact_allowed() {
        let mut gs = GameState::new();
        gs.apply(0, &Move { piece: 1, orient: 0, row: 0, col: 0 });
        // Yellow starts at (0,19); play pieces toward Blue.
        gs.apply(1, &Move { piece: 4, orient: 0, row: 0, col: 16 });
        // Yellow's next piece may touch Blue edge-to-edge (different colors),
        // as long as it only touches its own color diagonally.
        let mv = Move { piece: 2, orient: 0, row: 1, col: 13 };
        assert!(gs.move_is_legal(1, &mv));
    }

    #[test]
    fn official_scoring() {
        let mut gs = GameState::new();
        assert_eq!(gs.score(0), -89);
        gs.apply(0, &Move { piece: 10, orient: 0, row: 0, col: 0 }); // I5
        assert_eq!(gs.score(0), -84);
        // Simulate having placed everything, monomino last.
        gs.pieces_left[0] = 0;
        gs.last_piece[0] = Some(0);
        assert_eq!(gs.score(0), 20);
        gs.last_piece[0] = Some(5);
        assert_eq!(gs.score(0), 15);
    }

    #[test]
    fn legal_move_generation_matches_validator() {
        let mut gs = GameState::new();
        for p in 0..4 {
            let moves = gs.legal_moves(p);
            assert!(!moves.is_empty());
            for mv in &moves {
                assert!(gs.move_is_legal(p, mv), "generated move must be legal: {mv:?}");
            }
            gs.apply(p, &moves[moves.len() / 2]);
        }
        // Second round: still consistent.
        for p in 0..4 {
            for mv in gs.legal_moves(p) {
                assert!(gs.move_is_legal(p, &mv));
            }
        }
    }
}
