//! Headless AI self-play benchmark: strength and speed sanity check.
//! Run with: cargo run --release --example selfplay

use blokus::ai::{Rng, choose_move, choose_move_random};
use blokus::game::{GameState, PLAYER_NAMES};
use std::time::Instant;

fn play(strong: [bool; 4], seed: u64) -> ([i32; 4], u32) {
    let mut gs = GameState::new();
    let mut rng = Rng(seed | 1);
    let mut moves = 0;
    loop {
        let p = gs.current;
        let mv = if strong[p] { choose_move(&gs, p, &mut rng) } else { choose_move_random(&gs, p, &mut rng) };
        match mv {
            Some(mv) => {
                gs.apply(p, &mv);
                moves += 1;
            }
            None => gs.active[p] = false,
        }
        gs.advance_turn();
        if gs.is_over() {
            return ([gs.score(0), gs.score(1), gs.score(2), gs.score(3)], moves);
        }
    }
}

fn main() {
    let games = 20;

    // Strong self-play: timing + score quality.
    let t0 = Instant::now();
    let mut totals = [0i32; 4];
    let mut total_moves = 0u32;
    for seed in 0..games {
        let (scores, moves) = play([true; 4], seed * 31 + 7);
        for p in 0..4 {
            totals[p] += scores[p];
        }
        total_moves += moves;
    }
    let dt = t0.elapsed();
    println!("== {games} strong-AI self-play games in {dt:.2?} ==");
    println!(
        "   {:.2} ms/move average, {} moves total",
        dt.as_secs_f64() * 1000.0 / total_moves as f64,
        total_moves
    );
    for p in 0..4 {
        println!("   {:<7} avg score {:+.1}", PLAYER_NAMES[p], totals[p] as f64 / games as f64);
    }

    // Strong (Blue, Red) vs random baseline (Yellow, Green).
    let mut strong_wins = 0;
    let mut strong_sum = 0i32;
    let mut random_sum = 0i32;
    for seed in 0..games {
        let (s, _) = play([true, false, true, false], seed * 17 + 3);
        if s[0].max(s[2]) > s[1].max(s[3]) {
            strong_wins += 1;
        }
        strong_sum += s[0] + s[2];
        random_sum += s[1] + s[3];
    }
    println!("== strong vs random baseline: {strong_wins}/{games} wins ==");
    println!(
        "   avg strong score {:+.1}, avg random score {:+.1}",
        strong_sum as f64 / (2 * games) as f64,
        random_sum as f64 / (2 * games) as f64
    );
}
