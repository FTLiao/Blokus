//! Headless AI benchmark: strong (UCB1 bandit) vs greedy strength, plus
//! wall-clock budget compliance checks.
//! Run with: cargo run --release --example selfplay

use blokus::ai::{Budget, Rng, choose_move, choose_move_strong};
use blokus::game::{GameState, Move, PLAYER_NAMES};
use std::time::Instant;

/// Play one game; seats flagged in `strong` use the bandit search with the
/// given iteration budget, the rest use plain greedy. Returns final scores,
/// number of strong decisions, and total time spent in strong decisions.
fn play(strong: [bool; 4], iters: u32, seed: u64) -> ([i32; 4], u32, f64) {
    let mut gs = GameState::new();
    let mut rng = Rng(seed | 1);
    let mut strong_calls = 0u32;
    let mut strong_secs = 0f64;
    loop {
        let p = gs.current;
        let mv = if strong[p] {
            let t0 = Instant::now();
            let mv = choose_move_strong(&gs, p, Budget::Iterations(iters), &mut rng);
            strong_secs += t0.elapsed().as_secs_f64();
            strong_calls += 1;
            mv
        } else {
            choose_move(&gs, p, &mut rng)
        };
        match mv {
            Some(mv) => gs.apply(p, &mv),
            None => gs.active[p] = false,
        }
        gs.advance_turn();
        if gs.is_over() {
            return ([gs.score(0), gs.score(1), gs.score(2), gs.score(3)], strong_calls, strong_secs);
        }
    }
}

/// Advance a greedy self-play game to the given round, for timing probes.
fn position_at_round(round: u32, seed: u64) -> GameState {
    let mut gs = GameState::new();
    let mut rng = Rng(seed | 1);
    while gs.round < round && !gs.is_over() {
        let p = gs.current;
        match choose_move(&gs, p, &mut rng) {
            Some(mv) => gs.apply(p, &mv),
            None => gs.active[p] = false,
        }
        gs.advance_turn();
    }
    gs
}

fn time_millis_call(label: &str, gs: &GameState, ms: u64, rng: &mut Rng) {
    let p = gs.current;
    let t0 = Instant::now();
    let mv: Option<Move> = choose_move_strong(gs, p, Budget::Millis(ms), rng);
    let dt = t0.elapsed();
    println!(
        "   {label}: Millis({ms}) returned {} in {:.0} ms {}",
        if mv.is_some() { "a move" } else { "no move" },
        dt.as_secs_f64() * 1000.0,
        if dt.as_millis() as u64 <= ms + ms / 2 { "(within budget)" } else { "(OVER BUDGET)" },
    );
}

fn main() {
    let games = 12u64;
    let iters = 300u32;

    // (a) Strong (Blue, Red) vs greedy (Yellow, Green).
    println!("== strong Iterations({iters}) in seats 0,2 vs greedy in seats 1,3: {games} games ==");
    let mut strong_wins = 0;
    let mut totals = [0i32; 4];
    let mut calls = 0u32;
    let mut secs = 0f64;
    let t0 = Instant::now();
    for seed in 0..games {
        let (s, c, t) = play([true, false, true, false], iters, seed * 17 + 3);
        if s[0].max(s[2]) > s[1].max(s[3]) {
            strong_wins += 1;
        }
        for p in 0..4 {
            totals[p] += s[p];
        }
        calls += c;
        secs += t;
        println!(
            "   game {seed:>2}: scores {:+3} {:+3} {:+3} {:+3}  ({})",
            s[0],
            s[1],
            s[2],
            s[3],
            if s[0].max(s[2]) > s[1].max(s[3]) { "strong wins" } else { "greedy wins" }
        );
    }
    println!("== strong wins {strong_wins}/{games} ({:.0}%) ==", strong_wins as f64 * 100.0 / games as f64);
    for p in 0..4 {
        println!(
            "   {:<7} ({}) avg score {:+.1}",
            PLAYER_NAMES[p],
            if p % 2 == 0 { "strong" } else { "greedy" },
            totals[p] as f64 / games as f64
        );
    }
    println!(
        "   avg strong {:+.1} vs avg greedy {:+.1}; {:.0} ms per strong decision ({} decisions, total {:.1?})",
        (totals[0] + totals[2]) as f64 / (2 * games) as f64,
        (totals[1] + totals[3]) as f64 / (2 * games) as f64,
        secs * 1000.0 / calls as f64,
        calls,
        t0.elapsed(),
    );

    // (b) Wall-clock budget compliance from an opening and a midgame position.
    println!("== Millis budget compliance ==");
    let mut rng = Rng(0xB10C_05);
    let opening = GameState::new();
    time_millis_call("opening (round 0)", &opening, 1000, &mut rng);
    let midgame = position_at_round(6, 99);
    time_millis_call("midgame (round 6)", &midgame, 1000, &mut rng);
}
