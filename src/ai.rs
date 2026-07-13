//! Blokus AI.
//!
//! Strategy: enumerate every legal move (the old C++ AI only scanned a small
//! window around one edge cell), then score each move on the ideas strong
//! human play is built on:
//!
//! * play big pieces early (official scoring is per square),
//! * maximize your own open corners (future mobility),
//! * take corners away from opponents (blocking),
//! * race toward the center in the opening so you don't get walled in,
//! * keep your pieces spread out rather than clumped.
//!
//! Everything is bitboard-based, so a full evaluation of the thousands of
//! opening moves takes a few milliseconds.

use crate::game::{Bits, GameState, Move, N, ROW_MASK};
use crate::pieces::pieces;

/// Simple xorshift so we don't need an RNG dependency; only used to break
/// ties between equally-scored moves.
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

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

struct Ctx {
    all: Bits,
    /// Opponents' seed cells (their future placement anchors), per opponent.
    opp_seeds: [Bits; 4],
}

fn build_ctx(gs: &GameState, p: usize) -> Ctx {
    let mut opp_seeds = [[0u32; N]; 4];
    for o in 0..4 {
        if o != p && gs.active[o] {
            opp_seeds[o] = gs.seeds(o);
        }
    }
    Ctx { all: gs.union(), opp_seeds }
}

fn evaluate(gs: &GameState, p: usize, mv: &Move, ctx: &Ctx) -> i32 {
    let piece_size = pieces()[mv.piece].size as i32;

    // Occupancy after the move.
    let mut own_after = gs.occ[p];
    let mut all_after = ctx.all;
    let mut center_bonus = 0i32;
    let mut blocked = 0i32;
    for (r, c) in mv.cells() {
        let (r, c) = (r as usize, c as usize);
        own_after[r] |= 1 << c;
        all_after[r] |= 1 << c;
        // Chebyshev distance to the center 2x2; closer is better.
        let dr = (r as i32 * 2 - (N as i32 - 1)).abs() / 2;
        let dc = (c as i32 * 2 - (N as i32 - 1)).abs() / 2;
        center_bonus += 9 - dr.max(dc);
        // Each opponent anchor we sit on is a placement they lose.
        for o in 0..4 {
            if o != p {
                blocked += (ctx.opp_seeds[o][r] >> c & 1) as i32;
            }
        }
    }

    // Our open corners after the move: empty cells diagonally adjacent to us
    // and not edge-adjacent to us. This is our future mobility.
    let own_orth = orth_expand(&own_after);
    let own_diag = diag_expand(&own_after);
    let mut my_corners = 0i32;
    for r in 0..N {
        my_corners += (own_diag[r] & !own_orth[r] & !all_after[r] & ROW_MASK).count_ones() as i32;
    }

    // Opening: the first few rounds are about development, later the piece
    // race and blocking dominate.
    let opening = gs.round < 5;
    let center_w = if opening { 4 } else { 1 };

    piece_size * 90 + my_corners * 14 + blocked * 28 + center_bonus * center_w
}

/// Pick the best move for player `p`, or None if there is no legal move.
pub fn choose_move(gs: &GameState, p: usize, rng: &mut Rng) -> Option<Move> {
    let moves = gs.legal_moves(p);
    if moves.is_empty() {
        return None;
    }
    let ctx = build_ctx(gs, p);
    let mut best: Option<Move> = None;
    let mut best_score = i32::MIN;
    let mut ties = 0u64;
    for mv in moves {
        let s = evaluate(gs, p, &mv, &ctx);
        if s > best_score {
            best_score = s;
            best = Some(mv);
            ties = 1;
        } else if s == best_score {
            // Reservoir sampling over tied moves for variety between games.
            ties += 1;
            if rng.next() % ties == 0 {
                best = Some(mv);
            }
        }
    }
    best
}

/// A deliberately weak baseline: uniformly random legal move. Used in tests
/// to confirm the real AI is stronger, and roughly matches the strength of
/// the old C++ AI's early-game random placement.
pub fn choose_move_random(gs: &GameState, p: usize, rng: &mut Rng) -> Option<Move> {
    let moves = gs.legal_moves(p);
    if moves.is_empty() {
        None
    } else {
        Some(moves[(rng.next() % moves.len() as u64) as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_game(strong: [bool; 4], seed: u64) -> [i32; 4] {
        let mut gs = GameState::new();
        let mut rng = Rng(seed | 1);
        loop {
            let p = gs.current;
            let mv = if strong[p] {
                choose_move(&gs, p, &mut rng)
            } else {
                choose_move_random(&gs, p, &mut rng)
            };
            match mv {
                Some(mv) => gs.apply(p, &mv),
                None => gs.active[p] = false,
            }
            gs.advance_turn();
            if gs.is_over() {
                break;
            }
        }
        [gs.score(0), gs.score(1), gs.score(2), gs.score(3)]
    }

    #[test]
    fn selfplay_completes_and_scores_are_sane() {
        for seed in 1..=5u64 {
            let scores = play_game([true; 4], seed);
            for s in scores {
                assert!((-89..=20).contains(&s), "score out of range: {s}");
            }
            // Strong self-play should place most pieces: every player better
            // than -60 (the old AI frequently stalled far earlier).
            for s in scores {
                assert!(s > -60, "AI stalled with score {s}");
            }
        }
    }

    #[test]
    fn strong_ai_beats_random_ai() {
        let mut strong_wins = 0;
        let games = 8;
        for seed in 0..games {
            // Strong plays Blue and Red, random plays Yellow and Green.
            let scores = play_game([true, false, true, false], seed * 7 + 1);
            let strong_best = scores[0].max(scores[2]);
            let random_best = scores[1].max(scores[3]);
            if strong_best > random_best {
                strong_wins += 1;
            }
        }
        assert!(
            strong_wins >= games * 3 / 4,
            "strong AI only won {strong_wins}/{games} against random"
        );
    }
}
