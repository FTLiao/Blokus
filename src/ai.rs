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

use crate::game::{Bits, GameState, Move, N, ROW_MASK, TOTAL_SQUARES};
use crate::pieces::pieces;
use std::time::Instant;

/// Simple xorshift so we don't need an RNG dependency.
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// Uniform f64 in (0, 1).
    fn uniform(&mut self) -> f64 {
        ((self.next() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }

    /// Standard Gumbel noise: adding it to scores and taking the argmax
    /// samples from the softmax of the scores (perturb-and-max trick).
    fn gumbel(&mut self) -> f64 {
        let u = self.uniform();
        -(-u.ln()).ln()
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

/// Moves scoring within this many eval points of the best are treated as
/// interchangeable by the greedy policy; picking uniformly among them keeps
/// games (especially openings) from always starting identically.
const GREEDY_TOLERANCE: i32 = 25;

/// Pick a near-best move for player `p`, or None if there is no legal move.
/// Instead of a strict argmax, this samples uniformly among all moves within
/// `GREEDY_TOLERANCE` eval points of the best, so play varies between games.
pub fn choose_move(gs: &GameState, p: usize, rng: &mut Rng) -> Option<Move> {
    let moves = gs.legal_moves(p);
    if moves.is_empty() {
        return None;
    }
    let ctx = build_ctx(gs, p);
    let scores: Vec<i32> = moves.iter().map(|mv| evaluate(gs, p, mv, &ctx)).collect();
    let best = *scores.iter().max().unwrap();
    // Reservoir sampling over the near-best moves.
    let mut pick: Option<Move> = None;
    let mut count = 0u64;
    for (i, &s) in scores.iter().enumerate() {
        if s + GREEDY_TOLERANCE >= best {
            count += 1;
            if rng.next() % count == 0 {
                pick = Some(moves[i]);
            }
        }
    }
    pick
}

/// Search effort for the strong AI.
#[derive(Clone, Copy, Debug)]
pub enum Budget {
    /// Wall-clock thinking time.
    Millis(u64),
    /// Fixed number of bandit iterations (deterministic-ish; used in tests).
    Iterations(u32),
}

// ---- Strong AI: time-budgeted UCB1 bandit over candidate moves ----------

/// Number of top statically-scored moves kept as bandit arms.
const TOP_K: usize = 40;
/// UCB1 exploration constant (values live in [0,1]).
const UCB_C: f64 = 0.45;
/// Virtual visits seeding each arm from its normalized static score, so the
/// first pulls follow the heuristic ordering.
const PRIOR_N: f64 = 2.0;
/// Rollout horizon in plies after the arm move (~2 full rounds).
const ROLLOUT_PLIES: u32 = 8;
/// Gumbel noise scale (in eval units) for the noisy-greedy rollout policy.
const GUMBEL_SCALE: f64 = 25.0;
/// In rollouts, evaluate at most this many (randomly sampled) legal moves.
const ROLLOUT_MOVE_CAP: usize = 96;

/// Pick a move for the current rollout player: greedy over the static eval
/// plus Gumbel noise, so repeated rollouts explore different lines.
fn rollout_move(gs: &GameState, p: usize, rng: &mut Rng) -> Option<Move> {
    let moves = gs.legal_moves(p);
    if moves.is_empty() {
        return None;
    }
    let ctx = build_ctx(gs, p);
    let mut best: Option<Move> = None;
    let mut best_s = f64::NEG_INFINITY;
    let n = moves.len();
    let consider = |mv: &Move, rng: &mut Rng, best: &mut Option<Move>, best_s: &mut f64| {
        let s = evaluate(gs, p, mv, &ctx) as f64 + GUMBEL_SCALE * rng.gumbel();
        if s > *best_s {
            *best_s = s;
            *best = Some(*mv);
        }
    };
    if n <= ROLLOUT_MOVE_CAP {
        for mv in &moves {
            consider(mv, rng, &mut best, &mut best_s);
        }
    } else {
        for _ in 0..ROLLOUT_MOVE_CAP {
            let mv = moves[(rng.next() % n as u64) as usize];
            consider(&mv, rng, &mut best, &mut best_s);
        }
    }
    best
}

/// Leaf value from `p`'s perspective, in [0, 1].
fn leaf_value(gs: &GameState, p: usize) -> f64 {
    if gs.is_over() {
        let my = gs.score(p) as f64;
        let best_opp = (0..4).filter(|&o| o != p).map(|o| gs.score(o)).max().unwrap() as f64;
        return 0.5 + 0.5 * ((my - best_opp) / 12.0).tanh();
    }
    // Mid-game proxy: squares already placed plus mobility (open seeds),
    // measured against the strongest opponent still in the game.
    let potential = |q: usize| -> f64 {
        (TOTAL_SQUARES - gs.squares_remaining(q)) as f64 + 0.4 * gs.seed_count(q) as f64
    };
    let opp_best = (0..4)
        .filter(|&o| o != p && gs.active[o])
        .map(potential)
        .fold(f64::NEG_INFINITY, f64::max);
    let opp_best = if opp_best.is_finite() {
        opp_best
    } else {
        (0..4).filter(|&o| o != p).map(potential).fold(f64::NEG_INFINITY, f64::max)
    };
    0.5 + 0.5 * ((potential(p) - opp_best) / 12.0).tanh()
}

/// One Monte-Carlo rollout: play `mv`, then ~`ROLLOUT_PLIES` plies where every
/// player follows the noisy-greedy policy; return the leaf value for `p`.
fn rollout_value(root: &GameState, p: usize, mv: &Move, rng: &mut Rng) -> f64 {
    let mut gs = root.clone();
    gs.apply(p, mv);
    gs.advance_turn();
    for _ in 0..ROLLOUT_PLIES {
        if gs.is_over() {
            break;
        }
        let cur = gs.current;
        match rollout_move(&gs, cur, rng) {
            Some(m) => gs.apply(cur, &m),
            // advance_turn() only stops on players with a move, so this is
            // unreachable in practice; deactivate defensively.
            None => gs.active[cur] = false,
        }
        gs.advance_turn();
    }
    leaf_value(&gs, p)
}

/// Strong AI entry point: UCB1 bandit search over candidate moves within the
/// given budget. `Millis(0)` falls back to the fast greedy policy.
pub fn choose_move_strong(gs: &GameState, p: usize, budget: Budget, rng: &mut Rng) -> Option<Move> {
    if let Budget::Millis(0) = budget {
        return choose_move(gs, p, rng);
    }
    let moves = gs.legal_moves(p);
    match moves.len() {
        0 => return None,
        1 => return Some(moves[0]),
        _ => {}
    }

    // Candidate arms: top K moves by static eval.
    let ctx = build_ctx(gs, p);
    let mut scored: Vec<(i32, Move)> =
        moves.iter().map(|mv| (evaluate(gs, p, mv, &ctx), *mv)).collect();
    scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
    scored.truncate(TOP_K);

    struct Arm {
        mv: Move,
        n: f64,
        sum: f64,
    }
    // Seed each arm with a virtual prior from its normalized static score
    // (plus tiny jitter so equal-scored openings don't tie deterministically).
    let s_max = scored[0].0 as f64;
    let s_min = scored.last().unwrap().0 as f64;
    let span = (s_max - s_min).max(1.0);
    let mut arms: Vec<Arm> = scored
        .iter()
        .map(|&(s, mv)| {
            let prior =
                0.35 + 0.3 * (s as f64 - s_min) / span + 0.02 * (rng.uniform() - 0.5);
            Arm { mv, n: PRIOR_N, sum: PRIOR_N * prior }
        })
        .collect();

    let start = Instant::now();
    let mut total = 0f64;
    let mut iters = 0u32;
    loop {
        match budget {
            Budget::Millis(ms) => {
                if start.elapsed().as_millis() as u64 >= ms {
                    break;
                }
            }
            Budget::Iterations(n) => {
                if iters >= n {
                    break;
                }
            }
        }
        // UCB1 arm selection.
        let ln_t = (total + 1.0).ln();
        let mut pick = 0;
        let mut pick_u = f64::NEG_INFINITY;
        for (i, a) in arms.iter().enumerate() {
            let u = a.sum / a.n + UCB_C * (ln_t / (a.n + 1.0)).sqrt();
            if u > pick_u {
                pick_u = u;
                pick = i;
            }
        }
        let v = rollout_value(gs, p, &arms[pick].mv, rng);
        arms[pick].n += 1.0;
        arms[pick].sum += v;
        total += 1.0;
        iters += 1;
    }

    // Final selection: most-visited arm. Arms within 90% of the top visit
    // count are statistically indistinguishable after this few rollouts, so
    // pick uniformly among them (this keeps openings varied between games);
    // break exact ties by mean value.
    let n_max = arms.iter().map(|a| a.n).fold(0.0, f64::max);
    let near: Vec<&Arm> = arms.iter().filter(|a| a.n >= 0.9 * n_max).collect();
    if near.len() > 1 {
        return Some(near[(rng.next() % near.len() as u64) as usize].mv);
    }
    arms.iter()
        .max_by(|a, b| {
            (a.n, a.sum / a.n)
                .partial_cmp(&(b.n, b.sum / b.n))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|a| a.mv)
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

    /// Play one game where the flagged seats use the bandit search and the
    /// rest use plain greedy; returns final scores.
    fn play_game_bandit(bandit: [bool; 4], iters: u32, seed: u64) -> [i32; 4] {
        let mut gs = GameState::new();
        let mut rng = Rng(seed | 1);
        loop {
            let p = gs.current;
            let mv = if bandit[p] {
                choose_move_strong(&gs, p, Budget::Iterations(iters), &mut rng)
            } else {
                choose_move(&gs, p, &mut rng)
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
    fn bandit_search_beats_greedy() {
        let games = 4;
        let mut wins = 0;
        let mut margin = 0i32;
        for seed in 0..games {
            // Bandit plays Blue and Red, greedy plays Yellow and Green.
            let s = play_game_bandit([true, false, true, false], 200, seed * 13 + 5);
            if s[0].max(s[2]) > s[1].max(s[3]) {
                wins += 1;
            }
            margin += s[0] + s[2] - s[1] - s[3];
        }
        // The bandit should win a clear majority and be ahead on total score;
        // thresholds are loose enough to be stable across seeds.
        assert!(wins >= games / 2, "bandit only won {wins}/{games} vs greedy");
        assert!(margin > 0, "bandit behind greedy on total score ({margin})");
    }

    #[test]
    fn millis_budget_is_respected() {
        let gs = GameState::new();
        let mut rng = Rng(42);
        let t0 = std::time::Instant::now();
        let mv = choose_move_strong(&gs, 0, Budget::Millis(150), &mut rng);
        let dt = t0.elapsed();
        assert!(mv.is_some());
        // One rollout past the deadline is possible, but nothing more.
        assert!(dt.as_millis() < 600, "Millis(150) took {dt:?}");
        // Millis(0) must fall straight back to greedy.
        let t0 = std::time::Instant::now();
        assert!(choose_move_strong(&gs, 0, Budget::Millis(0), &mut rng).is_some());
        assert!(t0.elapsed().as_millis() < 250);
    }
}
