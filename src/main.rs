//! Blokus — macroquad UI.
//!
//! Setup screen: toggle each seat between Human and AI, set the AI think-time
//! dial, then start. In game: click a piece in the tray to pick it up,
//! R (or right-click) rotates, F flips, left-click on the board places.
//! AI turns run on a worker thread; Space pauses, Up/Down changes think time.

use blokus::ai::{self, Budget, Rng};
use blokus::game::{GameState, Move, N, PLAYER_NAMES, START_CORNERS, TOTAL_SQUARES};
use blokus::pieces::{NUM_PIECES, flip_horizontal, pieces, rotate_cw};
use macroquad::prelude::*;
use std::thread::JoinHandle;

mod audio;
mod cartoons;
mod memes;

use macroquad::audio::{PlaySoundParams, play_sound, set_sound_volume, stop_sound};

const CELL: f32 = 34.0;
const BOARD_X: f32 = 30.0;
const BOARD_Y: f32 = 78.0;
const TRAY_X: f32 = 748.0;
const TRAY_Y: f32 = 96.0;
const TRAY_COL_W: f32 = 96.0;
const TRAY_ROW_H: f32 = 80.0;
const SIDE_X: f32 = 1044.0;
const SIDE_W: f32 = 226.0;
const CARD_H: f32 = 78.0;
const CARD_GAP: f32 = 10.0;
/// Fixed pacing gap between moves so watch mode reads well at 0 think time.
const PACE_GAP: f32 = 0.25;
const THINK_MAX: f32 = 5.0;
/// Visual craziness dial range (0 = zen, 2 = ludicrous).
const CRAZY_MAX: f32 = 2.0;
/// Lifetime of the block-taunt popup, seconds.
const POPUP_TTL: f32 = 2.8;
/// Base gameplay-music volume and the crossfade length at slot boundaries.
const GAME_VOL: f32 = 0.35;
const FADE_SECS: f64 = 1.5;

fn player_color(p: usize) -> Color {
    match p {
        0 => Color::from_rgba(59, 130, 246, 255),
        1 => Color::from_rgba(250, 204, 21, 255),
        2 => Color::from_rgba(239, 68, 68, 255),
        _ => Color::from_rgba(34, 197, 94, 255),
    }
}

fn dim(c: Color, k: f32) -> Color {
    Color::new(c.r * k, c.g * k, c.b * k, c.a)
}

fn lighten(c: Color, k: f32) -> Color {
    Color::new((c.r + k).min(1.0), (c.g + k).min(1.0), (c.b + k).min(1.0), c.a)
}

/// Cheap deterministic hash noise in [0, 1) from an (index, channel) pair.
fn hnoise(i: u32, k: u32) -> f32 {
    let mut v = i.wrapping_mul(2654435761).wrapping_add(k.wrapping_mul(40503)).wrapping_add(0x9e37);
    v ^= v >> 13;
    v = v.wrapping_mul(1274126177);
    v ^= v >> 16;
    (v & 0xffff) as f32 / 65536.0
}

enum Screen {
    Setup,
    Playing,
    GameOver,
}

struct Sel {
    piece: usize,
    cells: Vec<(i8, i8)>,
}

/// Cartoon + taunt card shown when a move blocks opponent corners.
struct Popup {
    text: String,
    sub: String,
    cartoon: usize,
    color: Color,
    ttl: f32,
}

/// One firework particle. Sparks fly ballistically and fade out; rockets
/// (burst != None) ascend and explode into a spray of sparks when their
/// life runs out.
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    grav: f32,
    color: Color,
    ttl: f32,
    life: f32,
    size: f32,
    burst: Option<(Color, u32)>,
}

struct App {
    screen: Screen,
    is_ai: [bool; 4],
    gs: GameState,
    sel: Option<Sel>,
    rng: Rng,
    /// How long the AI may think per move, in seconds (0 = instant greedy).
    think_time: f32,
    /// Worker thread computing the current AI move (never on render thread).
    ai_handle: Option<JoinHandle<Option<Move>>>,
    /// Wall-clock time the current AI search started (for the progress bar).
    ai_started: f64,
    /// Time since the last move; the next AI move waits for PACE_GAP.
    pace: f32,
    paused: bool,
    /// Which slider is currently being dragged (by widget id), if any.
    drag_slider: Option<u8>,
    /// Spectacle multiplier, 0.0 (zen) to 2.0 (ludicrous).
    craziness: f32,
    toasts: Vec<(String, f32)>,
    particles: Vec<Particle>,
    /// Cartoon taunt card for the latest corner-blocking move.
    popup: Option<Popup>,
    /// Countdown to the next celebration rocket on the game-over screen.
    fw_timer: f32,
    winner: usize,
    meme_text: String,
    meme_cartoon: usize,
    /// Countdown until the next meme rotates in.
    meme_timer: f32,
    /// Decorative meme for the setup screen (rolled on entering it).
    setup_meme: String,
    setup_cartoon: usize,
    /// Chiptune tracks + SFX, synthesized at startup.
    sounds: Option<audio::Sounds>,
    muted: bool,
    /// Which music loop is currently playing.
    track: MusicTrack,
    /// Index into the rotating gameplay playlist.
    game_idx: usize,
    /// Wall-clock time when the current gameplay track started (fade-in).
    game_started: f64,
    /// Wall-clock time when the current gameplay track ends (rotation point).
    game_next_at: f64,
}

#[derive(Clone, Copy, PartialEq)]
enum MusicTrack {
    Silent,
    Menu,
    Game,
    Win,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::Setup,
            is_ai: [false, true, true, true],
            gs: GameState::new(),
            sel: None,
            rng: Rng(macroquad::miniquad::date::now().to_bits() | 1),
            think_time: 1.5,
            ai_handle: None,
            ai_started: 0.0,
            pace: 0.0,
            paused: false,
            drag_slider: None,
            craziness: 1.0,
            toasts: Vec::new(),
            particles: Vec::new(),
            popup: None,
            fw_timer: 0.0,
            winner: 0,
            meme_text: String::new(),
            meme_cartoon: 0,
            meme_timer: 4.0,
            setup_meme: String::new(),
            setup_cartoon: 0,
            sounds: None,
            muted: false,
            track: MusicTrack::Silent,
            game_idx: 0,
            game_started: 0.0,
            game_next_at: 0.0,
        }
    }

    // -- Music + SFX ------------------------------------------------------

    /// Stop whatever music is currently playing.
    fn stop_music(&mut self) {
        let Some(s) = &self.sounds else { return };
        match self.track {
            MusicTrack::Silent => {}
            MusicTrack::Menu => stop_sound(&s.menu),
            MusicTrack::Win => stop_sound(&s.win),
            MusicTrack::Game => {
                if let Some((snd, _)) = s.game.get(self.game_idx) {
                    stop_sound(snd);
                }
            }
        }
    }

    /// Start the music for the current `self.track` (respects mute).
    /// For gameplay this plays the current playlist entry once and schedules
    /// the next rotation; menu/win tracks loop in place.
    fn start_music(&mut self) {
        if self.muted {
            return;
        }
        let Some(s) = &self.sounds else { return };
        match self.track {
            MusicTrack::Silent => {}
            MusicTrack::Menu => play_sound(&s.menu, PlaySoundParams { looped: true, volume: 0.45 }),
            MusicTrack::Win => play_sound(&s.win, PlaySoundParams { looped: true, volume: 0.55 }),
            MusicTrack::Game => {
                if let Some((snd, dur)) = s.game.get(self.game_idx) {
                    // Each playlist slot is at most 20 s: short pieces repeat
                    // for as many full plays as fit (looped playback, stopped
                    // on a play boundary, so no mid-phrase cuts); longer
                    // pieces get one full play.
                    let plays = if *dur >= 20.0 { 1.0 } else { (20.0 / dur).floor().max(1.0) };
                    // Start silent; sync_music ramps the volume up per frame.
                    play_sound(snd, PlaySoundParams { looped: true, volume: 0.0 });
                    self.game_started = get_time();
                    // Plus a short breath between styles.
                    self.game_next_at = self.game_started + plays * dur + 0.7;
                }
            }
        }
    }

    /// Random playlist index, uniformly chosen among the tracks OTHER than
    /// the current one (0 if the playlist has fewer than two tracks).
    fn pick_game_track(&mut self) -> usize {
        let n = self.sounds.as_ref().map_or(1, |s| s.game.len().max(1));
        if n <= 1 {
            return 0;
        }
        (self.game_idx + 1 + (self.rng.next() % (n as u64 - 1)) as usize) % n
    }

    /// Keep the music in sync with the current screen, rotate the gameplay
    /// playlist when the current slot ends, and drive the slot crossfade.
    /// Called once per frame. Every path stops the outgoing track before a
    /// new one starts, so at most one music track is ever audible.
    fn sync_music(&mut self) {
        let desired = match self.screen {
            Screen::Setup => MusicTrack::Menu,
            Screen::Playing => MusicTrack::Game,
            Screen::GameOver => MusicTrack::Win,
        };
        if desired != self.track {
            self.stop_music();
            self.track = desired;
            if desired == MusicTrack::Game {
                // Fresh game: random first track too.
                self.game_idx = self.pick_game_track();
            }
            self.start_music();
        } else if self.track == MusicTrack::Game && !self.muted && get_time() >= self.game_next_at {
            // Slot over: stop the outgoing track BEFORE advancing the index
            // (skipping this stop is what used to stack tracks every ~20 s),
            // then pick a random different track.
            self.stop_music();
            self.game_idx = self.pick_game_track();
            self.start_music();
        }
        // Crossfade: ramp gameplay volume up over the first FADE_SECS of a
        // slot and back down over the final FADE_SECS before the rotation.
        if self.track == MusicTrack::Game && !self.muted {
            if let Some(s) = &self.sounds {
                if let Some((snd, _)) = s.game.get(self.game_idx) {
                    let now = get_time();
                    let fade_in = ((now - self.game_started) / FADE_SECS).clamp(0.0, 1.0);
                    let fade_out = ((self.game_next_at - now) / FADE_SECS).clamp(0.0, 1.0);
                    set_sound_volume(snd, GAME_VOL * fade_in.min(fade_out) as f32);
                }
            }
        }
    }

    fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        if self.muted {
            self.stop_music();
        } else {
            self.start_music();
        }
        self.toast(if self.muted { "Sound off".into() } else { "Sound on".into() });
    }

    /// The corner-block bang; louder for bigger blocks.
    fn play_bang(&mut self, blocked: usize) {
        if self.muted {
            return;
        }
        if let Some(s) = &self.sounds {
            let volume = (0.5 + 0.12 * blocked as f32).min(1.0);
            play_sound(&s.bang, PlaySoundParams { looped: false, volume });
        }
    }

    /// Roll a fresh decorative meme + cartoon for the setup screen. Called
    /// on entering the screen only, never per frame.
    fn roll_setup_meme(&mut self) {
        let mut raw = memes::pick(self.rng.next());
        // Prefer captions without the {winner} placeholder on the front page.
        for _ in 0..8 {
            if !raw.contains("{winner}") {
                break;
            }
            raw = memes::pick(self.rng.next());
        }
        self.setup_meme = raw.replace("{winner}", "The winner");
        self.setup_cartoon = (self.rng.next() % cartoons::COUNT.max(1) as u64) as usize;
    }

    /// Uniform random float in [0, 1).
    fn rf(&mut self) -> f32 {
        (self.rng.next() >> 40) as f32 / 16_777_216.0
    }

    fn toast(&mut self, msg: String) {
        self.toasts.push((msg, 2.2));
    }

    /// Abandon whatever is on screen and return to the setup menu. Drops any
    /// AI worker handle (its result is ignored, same pattern as start_game);
    /// sync_music swaps the track on the next frame.
    fn abandon_to_menu(&mut self) {
        self.ai_handle = None;
        self.sel = None;
        self.paused = false;
        self.pace = 0.0;
        self.toasts.clear();
        self.particles.clear();
        self.popup = None;
        self.roll_setup_meme();
        self.screen = Screen::Setup;
    }

    fn start_game(&mut self) {
        self.gs = GameState::new();
        self.sel = None;
        self.paused = false;
        self.pace = 0.0;
        self.ai_handle = None; // detach any stale worker; its result is ignored
        self.toasts.clear();
        self.particles.clear();
        self.popup = None;
        self.screen = Screen::Playing;
    }

    fn finish_move(&mut self) {
        self.sel = None;
        self.pace = 0.0;
        for p in self.gs.advance_turn() {
            self.toast(format!("{} is out of moves", PLAYER_NAMES[p]));
        }
        if self.gs.is_over() {
            let mut ranked: Vec<usize> = (0..4).collect();
            ranked.sort_by_key(|&p| -self.gs.score(p));
            self.winner = ranked[0];
            self.pick_meme();
            self.fw_timer = 0.0;
            self.screen = Screen::GameOver;
        }
    }

    fn pick_meme(&mut self) {
        let raw = memes::pick(self.rng.next());
        self.meme_text = raw.replace("{winner}", PLAYER_NAMES[self.winner]);
        self.meme_cartoon = (self.rng.next() % cartoons::COUNT.max(1) as u64) as usize;
        self.meme_timer = 4.0;
    }

    // -- Fireworks particle system ---------------------------------------

    fn spawn_burst(&mut self, x: f32, y: f32, color: Color, n: u32) {
        for _ in 0..n {
            let ang = self.rf() * std::f32::consts::TAU;
            let speed = 30.0 + self.rf().sqrt() * 230.0;
            let ttl = 0.8 + self.rf() * 1.0;
            let c = if self.rf() < 0.28 { WHITE } else { color };
            let grav = 110.0 + self.rf() * 60.0;
            let size = 1.9 + self.rf() * 2.4;
            self.particles.push(Particle {
                x,
                y,
                vx: ang.cos() * speed,
                vy: ang.sin() * speed - 25.0,
                grav,
                color: c,
                ttl,
                life: ttl,
                size,
                burst: None,
            });
        }
        self.trim_particles();
    }

    fn launch_rocket(&mut self, x: f32, y: f32, color: Color, sparks: u32) {
        let ttl = 0.55 + self.rf() * 0.5;
        let vx = self.rf() * 90.0 - 45.0;
        let vy = -(330.0 + self.rf() * 260.0);
        self.particles.push(Particle {
            x,
            y,
            vx,
            vy,
            grav: 120.0,
            color: WHITE,
            ttl,
            life: ttl,
            size: 2.4,
            burst: Some((color, sparks)),
        });
        self.trim_particles();
    }

    fn trim_particles(&mut self) {
        let len = self.particles.len();
        if len > 4000 {
            self.particles.drain(0..len - 4000);
        }
    }

    fn update_particles(&mut self) {
        if self.particles.is_empty() {
            return;
        }
        let dt = get_frame_time().min(0.05);
        let mut bursts: Vec<(f32, f32, Color, u32)> = Vec::new();
        for p in &mut self.particles {
            p.life -= dt;
            p.vy += p.grav * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            if p.life <= 0.0 {
                if let Some((c, n)) = p.burst {
                    bursts.push((p.x, p.y, c, n));
                }
            }
        }
        self.particles.retain(|p| p.life > 0.0);
        for (x, y, c, n) in bursts {
            self.spawn_burst(x, y, c, n);
        }
    }

    /// Fireworks + cartoon taunt popup scaled to how many opponent corners
    /// this move just blocked, bursting at the blocked cells in the blocking
    /// player's color. Purely visual; everything scales with the craziness
    /// dial (at 0.0 only an informational toast survives for big blocks).
    fn celebrate_block(&mut self, p: usize, cells: &[(i32, i32)]) {
        let b = cells.len();
        if b == 0 {
            return;
        }
        self.play_bang(b);
        let cz = self.craziness;
        if cz <= 0.0 {
            if b >= 3 {
                self.toast(format!("{} blocked {b} corners!", PLAYER_NAMES[p]));
            }
            return;
        }
        let base: f32 = match b {
            1..=2 => 26.0,
            3..=4 => 42.0,
            _ => 55.0,
        };
        let per = (base * cz).round() as u32;
        let color = player_color(p);
        for &(r, c) in cells {
            let (x, y) = cell_xy(r, c);
            self.spawn_burst(x + CELL / 2.0, y + CELL / 2.0, color, per);
        }
        if b >= 5 {
            // Crazy volley: rockets fired up from below the board.
            let side = N as f32 * CELL;
            let sparks = (60.0 * cz.min(1.5)) as u32;
            for _ in 0..(4.0 * cz).round() as u32 {
                let x = BOARD_X + self.rf() * side;
                self.launch_rocket(x, BOARD_Y + side + 10.0, color, sparks);
            }
        }
        // Cartoon taunt popup (folds in the old ">=3 corners" toast).
        let phrase = if b == 1 { "1 corner".to_string() } else { format!("{b} corners") };
        let taunt = memes::pick_taunt(self.rng.next())
            .replace("{blocker}", PLAYER_NAMES[p])
            .replace("{n}", &phrase);
        let cartoon = (self.rng.next() % cartoons::COUNT.max(1) as u64) as usize;
        self.popup = Some(Popup {
            text: taunt,
            sub: format!("{} blocked {phrase}!", PLAYER_NAMES[p]),
            cartoon,
            color,
            ttl: POPUP_TTL,
        });
    }
}

/// Opponent anchor cells (seeds) that move `mv` of player `p` would cover,
/// summed over still-active opponents (a cell seeding two opponents counts
/// twice). Call BEFORE `gs.apply`.
fn blocked_corner_cells(gs: &GameState, p: usize, mv: &Move) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for o in 0..4 {
        if o == p || !gs.active[o] {
            continue;
        }
        let seeds = gs.seeds(o);
        for (r, c) in mv.cells() {
            if (0..N as i32).contains(&r) && (0..N as i32).contains(&c) && seeds[r as usize] >> c & 1 == 1 {
                out.push((r, c));
            }
        }
    }
    out
}

fn draw_particles(app: &App) {
    for p in &app.particles {
        let a = (p.life / p.ttl).clamp(0.0, 1.0);
        if p.burst.is_some() {
            // Rocket: bright head with a short streak behind it.
            draw_line(p.x, p.y, p.x - p.vx * 0.045, p.y - p.vy * 0.045, 2.0, Color::new(1.0, 0.95, 0.75, 0.35 + 0.5 * a));
            draw_circle(p.x, p.y, p.size, Color::new(1.0, 1.0, 1.0, 0.95));
        } else {
            draw_circle(p.x, p.y, p.size * 2.2, Color { a: 0.3 * a, ..p.color });
            draw_circle(p.x, p.y, p.size * (0.4 + 0.6 * a), Color { a: (a * 1.7).min(1.0), ..p.color });
        }
    }
}

/// Cartoon taunt popup over the top of the board. Springy pop-in, shrink +
/// fade out; visual only (never touches input, pacing, or the AI thread).
fn draw_block_popup(app: &mut App) {
    let dt = get_frame_time();
    if let Some(pp) = &mut app.popup {
        pp.ttl -= dt;
        if pp.ttl <= 0.0 {
            app.popup = None;
        }
    }
    let Some(pp) = &app.popup else { return };
    let age = POPUP_TTL - pp.ttl;
    let t1 = (age / 0.35).min(1.0);
    let mut scale = 1.0 - (1.0 - t1).powi(3) + 0.16 * (1.0 - t1) * (t1 * 12.0).sin();
    let alpha = (pp.ttl / 0.4).clamp(0.0, 1.0);
    scale *= alpha.sqrt();
    if scale <= 0.03 {
        return;
    }
    let lines = wrap_lines(&pp.text, 24, 320.0);
    let content_h = lines.len() as f32 * 27.0 + 30.0;
    let w0 = 490.0;
    let h0 = (content_h + 42.0).max(118.0);
    let ccx = BOARD_X + N as f32 * CELL / 2.0;
    let ccy = BOARD_Y + 88.0;
    let (w, h) = (w0 * scale, h0 * scale);
    let (x, y) = (ccx - w / 2.0, ccy - h / 2.0);
    draw_rectangle(x + 4.0, y + 5.0, w, h, Color::new(0.0, 0.0, 0.0, 0.4 * alpha));
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.06, 0.1, 0.93 * alpha));
    draw_rectangle_lines(x, y, w, h, 3.0, Color { a: 0.9 * alpha, ..pp.color });
    cartoons::draw(pp.cartoon, x + 66.0 * scale, ccy, 96.0 * scale, get_time() as f32);
    let tx = x + 132.0 * scale;
    let tw = (w0 - 152.0) * scale;
    let mut ty = ccy - content_h / 2.0 * scale + 18.0 * scale;
    for line in &lines {
        let fs = 24.0 * scale;
        let d = measure_text(line, None, fs.max(1.0) as u16, 1.0);
        draw_text(line, tx + (tw - d.width) / 2.0, ty, fs, Color::new(1.0, 1.0, 1.0, alpha));
        ty += 27.0 * scale;
    }
    let fs = 17.0 * scale;
    let d = measure_text(&pp.sub, None, fs.max(1.0) as u16, 1.0);
    draw_text(&pp.sub, tx + (tw - d.width) / 2.0, ty + 4.0 * scale, fs, Color { a: 0.9 * alpha, ..pp.color });
}

// ---------------------------------------------------------------------------
// Shared drawing helpers
// ---------------------------------------------------------------------------

/// A physical-looking game block filling an s x s box: drop shadow, body,
/// light top/left bevel, dark bottom edge, shaded inner plateau.
fn draw_block(x: f32, y: f32, s: f32, color: Color) {
    let p = (s * 0.05).max(1.0);
    let w = s - 2.0 * p;
    let edge = (s * 0.12).max(2.0);
    draw_rectangle(x + p + s * 0.09, y + p + s * 0.11, w, w, Color::new(0.0, 0.0, 0.0, 0.35));
    draw_rectangle(x + p, y + p, w, w, color);
    draw_rectangle(x + p, y + p, w, edge, Color { a: 0.85, ..lighten(color, 0.22) });
    draw_rectangle(x + p, y + p, (s * 0.1).max(2.0), w, Color { a: 0.5, ..lighten(color, 0.14) });
    draw_rectangle(x + p, y + p + w - edge, w, edge, Color { a: 0.75, ..dim(color, 0.6) });
    draw_rectangle(x + s * 0.26, y + s * 0.26, s * 0.48, s * 0.48, dim(color, 0.86));
}

/// Slow drifting translucent squares behind everything; count and alpha
/// scale with the craziness dial (minimal but present at zen).
fn draw_ambient(cz: f32) {
    let t = get_time() as f32;
    let sw = screen_width();
    let sh = screen_height();
    let count = (4.0 + 14.0 * cz).round() as u32;
    let alpha_scale = (0.5 + 0.5 * cz).min(1.5);
    for i in 0..count {
        let size = 50.0 + hnoise(i, 0) * 150.0;
        let x = (hnoise(i, 1) * sw + t * (4.0 + hnoise(i, 2) * 11.0)) % (sw + size) - size;
        let y = (hnoise(i, 3) * sh + t * (2.0 + hnoise(i, 4) * 7.0)) % (sh + size) - size;
        let rot = (t * (2.0 + hnoise(i, 5) * 5.0) + hnoise(i, 6) * 360.0).to_radians();
        let c = player_color((i % 4) as usize);
        draw_rectangle_ex(
            x,
            y,
            size,
            size,
            DrawRectangleParams {
                rotation: rot,
                color: Color { a: (0.045 + hnoise(i, 7) * 0.035) * alpha_scale, ..c },
                ..Default::default()
            },
        );
    }
}

fn button(x: f32, y: f32, w: f32, h: f32, label: &str, active: bool) -> bool {
    let (mx, my) = mouse_position();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let bg = if active {
        if hover { Color::from_rgba(96, 112, 152, 255) } else { Color::from_rgba(74, 88, 122, 255) }
    } else if hover {
        Color::from_rgba(58, 63, 80, 255)
    } else {
        Color::from_rgba(46, 50, 63, 255)
    };
    draw_rectangle(x + 2.0, y + 3.0, w, h, Color::new(0.0, 0.0, 0.0, 0.3));
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle(x, y, w, 3.0, Color::new(1.0, 1.0, 1.0, if active { 0.25 } else { 0.1 }));
    let border = if active {
        Color::from_rgba(170, 190, 230, 255)
    } else {
        Color::from_rgba(110, 118, 140, 255)
    };
    draw_rectangle_lines(x, y, w, h, 2.0, border);
    let size = 24.0;
    let dims = measure_text(label, None, size as u16, 1.0);
    draw_text(label, x + (w - dims.width) / 2.0, y + h / 2.0 + dims.height / 2.0 - 2.0, size, WHITE);
    hover && is_mouse_button_pressed(MouseButton::Left)
}

// ---------------------------------------------------------------------------
// Board
// ---------------------------------------------------------------------------

fn board_cell_at(mx: f32, my: f32) -> Option<(i32, i32)> {
    let c = ((mx - BOARD_X) / CELL).floor() as i32;
    let r = ((my - BOARD_Y) / CELL).floor() as i32;
    if r >= 0 && c >= 0 && r < N as i32 && c < N as i32 { Some((r, c)) } else { None }
}

fn cell_xy(r: i32, c: i32) -> (f32, f32) {
    (BOARD_X + c as f32 * CELL, BOARD_Y + r as f32 * CELL)
}

fn draw_board(app: &App) {
    let t = get_time() as f32;
    let side = N as f32 * CELL;
    // Frame with a soft glow in the current player's color.
    let glow = Color { a: 0.20 + 0.10 * (t * 2.0).sin(), ..player_color(app.gs.current) };
    for i in 0..3 {
        let k = 8.0 + i as f32 * 4.0;
        draw_rectangle_lines(BOARD_X - k, BOARD_Y - k, side + 2.0 * k, side + 2.0 * k, 2.0, Color { a: glow.a * (1.0 - i as f32 * 0.3), ..glow });
    }
    draw_rectangle(BOARD_X - 7.0, BOARD_Y - 7.0, side + 14.0, side + 14.0, Color::from_rgba(16, 18, 24, 255));
    // Checkered empty cells.
    for r in 0..N {
        for c in 0..N {
            let (x, y) = cell_xy(r as i32, c as i32);
            let base = if (r + c) % 2 == 0 {
                Color::from_rgba(43, 47, 59, 255)
            } else {
                Color::from_rgba(38, 42, 53, 255)
            };
            draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, base);
        }
    }
    // Placed pieces as physical blocks.
    for p in 0..4 {
        for r in 0..N {
            for c in 0..N {
                if app.gs.occ[p][r] >> c & 1 == 1 {
                    let (x, y) = cell_xy(r as i32, c as i32);
                    draw_block(x, y, CELL, player_color(p));
                }
            }
        }
        // Starting-corner marker until the player has placed something.
        if app.gs.is_first_move(p) && app.gs.active[p] {
            let (r, c) = START_CORNERS[p];
            let (x, y) = cell_xy(r as i32, c as i32);
            let pulse = 0.24 + 0.06 * (t * 3.0).sin();
            draw_poly(x + CELL / 2.0, y + CELL / 2.0, 4, CELL * pulse, 45.0, player_color(p));
        }
    }
    // Anchor hints: on a human's turn, pulse the seed cells they can grow from.
    let cur = app.gs.current;
    if matches!(app.screen, Screen::Playing) && !app.is_ai[cur] && app.gs.active[cur] {
        let seeds = app.gs.seeds(cur);
        let col = player_color(cur);
        for r in 0..N {
            for c in 0..N {
                if seeds[r] >> c & 1 == 1 {
                    let (x, y) = cell_xy(r as i32, c as i32);
                    let ph = t * 4.0 + (r + c) as f32 * 0.55;
                    let rad = 4.2 + 1.6 * ph.sin();
                    draw_circle(x + CELL / 2.0, y + CELL / 2.0, rad + 2.5, Color { a: 0.18, ..col });
                    draw_circle(x + CELL / 2.0, y + CELL / 2.0, rad, Color { a: 0.55 + 0.25 * ph.sin(), ..col });
                }
            }
        }
    }
    // Pulsing outline on the most recent placement.
    let last_player = (app.gs.current + 3) % 4;
    if let Some(mv) = &app.gs.last_move[last_player] {
        let a = 0.55 + 0.4 * (t * 4.5).sin();
        for (r, c) in mv.cells() {
            let (x, y) = cell_xy(r, c);
            draw_rectangle_lines(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, 3.0, Color::new(1.0, 1.0, 1.0, a));
        }
    }
}

// ---------------------------------------------------------------------------
// Tray
// ---------------------------------------------------------------------------

/// Piece thumbnail centered in a box; returns true if clicked.
fn draw_piece_thumb(piece: usize, x: f32, y: f32, w: f32, h: f32, color: Color, enabled: bool, selected: bool) -> bool {
    let (mx, my) = mouse_position();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let bg = if selected {
        Color::from_rgba(74, 84, 116, 255)
    } else if hover && enabled {
        Color::from_rgba(58, 64, 84, 255)
    } else {
        Color::from_rgba(37, 41, 53, 255)
    };
    draw_rectangle(x, y, w, h, bg);
    if selected {
        let a = 0.6 + 0.3 * (get_time() as f32 * 5.0).sin();
        draw_rectangle_lines(x, y, w, h, 3.0, Color { a, ..color });
    } else {
        draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(82, 88, 108, 255));
    }
    let cells = &pieces()[piece].orientations[0];
    let max_r = cells.iter().map(|c| c.0).max().unwrap() as f32 + 1.0;
    let max_c = cells.iter().map(|c| c.1).max().unwrap() as f32 + 1.0;
    let s = 13.0;
    let ox = x + (w - max_c * s) / 2.0;
    let oy = y + (h - max_r * s) / 2.0;
    let col = if enabled { color } else { dim(color, 0.28) };
    for &(r, c) in cells {
        draw_block(ox + c as f32 * s, oy + r as f32 * s, s, col);
    }
    enabled && hover && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_tray(app: &mut App) {
    let p = app.gs.current;
    let human_turn = !app.is_ai[p] && app.gs.active[p];
    // Panel backdrop.
    draw_rectangle(TRAY_X - 12.0, TRAY_Y - 46.0, 3.0 * TRAY_COL_W + 16.0, 7.0 * TRAY_ROW_H + 52.0, Color::new(0.06, 0.07, 0.1, 0.55));
    draw_rectangle(TRAY_X - 12.0, TRAY_Y - 46.0, 3.0 * TRAY_COL_W + 16.0, 3.0, Color { a: 0.7, ..player_color(p) });
    draw_text(&format!("{}'s pieces", PLAYER_NAMES[p]), TRAY_X, TRAY_Y - 18.0, 26.0, player_color(p));
    let mut clicked: Option<usize> = None;
    for piece in 0..NUM_PIECES {
        let col = piece % 3;
        let row = piece / 3;
        let x = TRAY_X + col as f32 * TRAY_COL_W;
        let y = TRAY_Y + row as f32 * TRAY_ROW_H;
        let available = app.gs.has_piece(p, piece);
        let selected = app.sel.as_ref().is_some_and(|s| s.piece == piece);
        if draw_piece_thumb(piece, x, y, TRAY_COL_W - 8.0, TRAY_ROW_H - 8.0, player_color(p), available && human_turn, selected) {
            clicked = Some(piece);
        }
    }
    if let Some(piece) = clicked {
        if app.sel.as_ref().is_some_and(|s| s.piece == piece) {
            app.sel = None;
        } else {
            app.sel = Some(Sel { piece, cells: pieces()[piece].orientations[0].clone() });
        }
    }
}

// ---------------------------------------------------------------------------
// Player cards + side panel
// ---------------------------------------------------------------------------

/// Player cards + controls block; returns true if the in-game Menu button
/// (shown only while playing) was clicked.
fn draw_player_cards(app: &App) -> bool {
    let t = get_time() as f32;
    let x = SIDE_X;
    draw_text("Players", x, TRAY_Y - 18.0, 26.0, LIGHTGRAY);
    let mut y = TRAY_Y;
    for p in 0..4 {
        let c = player_color(p);
        let out = !app.gs.active[p];
        let current = p == app.gs.current && matches!(app.screen, Screen::Playing);
        let bg = if current {
            Color::from_rgba(50, 56, 76, 255)
        } else {
            Color::from_rgba(34, 38, 49, 255)
        };
        draw_rectangle(x + 2.0, y + 3.0, SIDE_W, CARD_H, Color::new(0.0, 0.0, 0.0, 0.3));
        draw_rectangle(x, y, SIDE_W, CARD_H, if out { dim(bg, 0.75) } else { bg });
        // Accent bar.
        draw_rectangle(x, y, 5.0, CARD_H, if out { dim(c, 0.35) } else { c });
        if current {
            let a = 0.55 + 0.35 * (t * 3.2).sin();
            draw_rectangle_lines(x, y, SIDE_W, CARD_H, 2.0, Color { a, ..c });
        }
        let ink = if out { 0.4 } else { 1.0 };
        // Name + seat tag.
        draw_text(PLAYER_NAMES[p], x + 16.0, y + 26.0, 24.0, Color { a: ink, ..if out { GRAY } else { c } });
        let tag = if out { "OUT" } else if app.is_ai[p] { "AI" } else { "HUMAN" };
        let td = measure_text(tag, None, 16, 1.0);
        draw_text(tag, x + SIDE_W - 12.0 - td.width, y + 24.0, 16.0, Color::new(0.62, 0.66, 0.76, ink));
        // Score (live) + pieces left.
        let score = format!("{:+}", app.gs.score(p));
        let sd = measure_text(&score, None, 30, 1.0);
        draw_text(&score, x + SIDE_W - 12.0 - sd.width, y + 54.0, 30.0, Color::new(1.0, 1.0, 1.0, ink));
        let left = app.gs.pieces_left[p].count_ones();
        draw_text(&format!("{left} pieces left"), x + 16.0, y + 50.0, 17.0, Color::new(0.62, 0.66, 0.76, ink));
        // Progress: squares placed out of 89.
        let placed = (TOTAL_SQUARES - app.gs.squares_remaining(p)) as f32 / TOTAL_SQUARES as f32;
        let bw = SIDE_W - 28.0;
        draw_rectangle(x + 16.0, y + CARD_H - 16.0, bw, 6.0, Color::from_rgba(20, 22, 29, 255));
        draw_rectangle(x + 16.0, y + CARD_H - 16.0, bw * placed, 6.0, Color { a: 0.55 + 0.45 * ink, ..c });
        y += CARD_H + CARD_GAP;
    }
    // Controls.
    y += 22.0;
    draw_text("Controls", x, y, 24.0, LIGHTGRAY);
    y += 26.0;
    let lines: [&str; 7] = [
        "Click piece, then board",
        "R / right-click: rotate",
        "F: flip",
        "Esc: put piece back",
        "Space: pause AI",
        "Up/Down: AI think time",
        "M: mute sound",
    ];
    for l in lines {
        draw_text(l, x, y, 19.0, GRAY);
        y += 22.0;
    }
    y += 18.0;
    let think = if app.think_time <= 0.0 { "instant".to_string() } else { format!("{:.1}s / move", app.think_time) };
    draw_text(&format!("AI think time: {think}"), x, y, 20.0, LIGHTGRAY);
    draw_text(&format!("Craziness: {:.0}%", app.craziness * 100.0), x, y + 22.0, 17.0, GRAY);
    if app.paused {
        draw_text("PAUSED", x, y + 44.0, 20.0, GOLD);
    }
    // In-game Menu button: abandon the game and return to setup.
    let mut menu_clicked = false;
    if matches!(app.screen, Screen::Playing) {
        menu_clicked = button(x, y + 58.0, SIDE_W, 36.0, "Menu", false);
    }
    menu_clicked
}

// ---------------------------------------------------------------------------
// Ghost preview
// ---------------------------------------------------------------------------

fn draw_ghost(app: &App) -> Option<(Move, bool)> {
    let sel = app.sel.as_ref()?;
    let (mx, my) = mouse_position();
    let (r, c) = board_cell_at(mx, my)?;
    let h = sel.cells.iter().map(|c| c.0).max().unwrap() as i32;
    let w = sel.cells.iter().map(|c| c.1).max().unwrap() as i32;
    let (row, col) = (r - h / 2, c - w / 2);
    let orient = pieces()[sel.piece].orientations.iter().position(|o| *o == sel.cells)?;
    let mv = Move { piece: sel.piece, orient, row, col };
    let legal = app.gs.move_is_legal(app.gs.current, &mv);
    let pulse = 0.65 + 0.25 * (get_time() as f32 * 6.0).sin();
    let outline = if legal {
        Color::new(1.0, 1.0, 1.0, pulse)
    } else {
        Color::new(1.0, 0.25, 0.25, 0.6)
    };
    for (rr, cc) in mv.cells() {
        if (0..N as i32).contains(&rr) && (0..N as i32).contains(&cc) {
            let (x, y) = cell_xy(rr, cc);
            draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, Color { a: 0.45, ..player_color(app.gs.current) });
            draw_rectangle_lines(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, 2.0, outline);
        }
    }
    Some((mv, legal))
}

// ---------------------------------------------------------------------------
// Game loop: update + draw
// ---------------------------------------------------------------------------

fn update_playing(app: &mut App) {
    let p = app.gs.current;

    // Global controls.
    if is_key_pressed(KeyCode::Space) {
        app.paused = !app.paused;
    }
    if is_key_pressed(KeyCode::Up) {
        app.think_time = (app.think_time + 0.5).min(THINK_MAX);
    }
    if is_key_pressed(KeyCode::Down) {
        app.think_time = (app.think_time - 0.5).max(0.0);
    }

    if app.is_ai[p] {
        if app.ai_handle.is_some() {
            // Poll the worker; join only once finished so the UI never blocks.
            if app.ai_handle.as_ref().is_some_and(|h| h.is_finished()) && !app.paused {
                let mv = app.ai_handle.take().unwrap().join().unwrap_or(None);
                match mv {
                    Some(mv) => {
                        let blocked = blocked_corner_cells(&app.gs, p, &mv);
                        app.gs.apply(p, &mv);
                        app.celebrate_block(p, &blocked);
                    }
                    None => app.gs.active[p] = false,
                }
                app.finish_move();
            }
        } else if !app.paused {
            app.pace += get_frame_time();
            if app.pace >= PACE_GAP {
                let gs = app.gs.clone();
                let seed = app.rng.next() | 1;
                let think = app.think_time;
                app.ai_started = get_time();
                app.ai_handle = Some(std::thread::spawn(move || {
                    let mut rng = Rng(seed);
                    if think <= 0.0 {
                        ai::choose_move(&gs, p, &mut rng)
                    } else {
                        ai::choose_move_strong(&gs, p, Budget::Millis((think * 1000.0) as u64), &mut rng)
                    }
                }));
            }
        }
        return;
    }

    // Human turn: rotate / flip / deselect.
    if let Some(sel) = &mut app.sel {
        if is_key_pressed(KeyCode::R) || is_mouse_button_pressed(MouseButton::Right) {
            sel.cells = rotate_cw(&sel.cells);
        }
        if is_key_pressed(KeyCode::F) {
            sel.cells = flip_horizontal(&sel.cells);
        }
        if is_key_pressed(KeyCode::Escape) {
            app.sel = None;
        }
    }
}

fn draw_header(app: &App) {
    let t = get_time() as f32;
    draw_text("BLOKUS", BOARD_X, 42.0, 42.0, WHITE);
    let lw = measure_text("BLOKUS", None, 42, 1.0).width;
    let seg = lw / 4.0;
    for i in 0..4 {
        draw_rectangle(BOARD_X + i as f32 * seg, 48.0, seg - 3.0, 5.0, player_color(i));
    }
    let p = app.gs.current;
    if matches!(app.screen, Screen::Playing) {
        if app.is_ai[p] {
            let dots = ".".repeat(1 + ((t * 2.5) as usize) % 3);
            let label = if app.paused {
                format!("{} (AI) — paused", PLAYER_NAMES[p])
            } else {
                format!("{} (AI) is thinking{dots}", PLAYER_NAMES[p])
            };
            draw_text(&label, BOARD_X + 240.0, 42.0, 28.0, player_color(p));
            // Thin progress bar: elapsed vs think budget.
            if app.ai_handle.is_some() && app.think_time > 0.0 && !app.paused {
                let frac = (((get_time() - app.ai_started) as f32) / app.think_time).clamp(0.0, 1.0);
                let bw = N as f32 * CELL;
                draw_rectangle(BOARD_X, 58.0, bw, 4.0, Color::from_rgba(18, 20, 26, 255));
                draw_rectangle(BOARD_X, 58.0, bw * frac, 4.0, Color { a: 0.9, ..player_color(p) });
            }
        } else {
            draw_text(&format!("Your turn, {}", PLAYER_NAMES[p]), BOARD_X + 240.0, 42.0, 28.0, player_color(p));
        }
    }
}

fn draw_playing(app: &mut App) {
    draw_header(app);
    draw_board(app);
    draw_tray(app);
    let menu_clicked = draw_player_cards(app);

    let p = app.gs.current;
    // Ghost + placement for the human player.
    if !app.is_ai[p] {
        if let Some((mv, legal)) = draw_ghost(app) {
            if legal && is_mouse_button_pressed(MouseButton::Left) {
                let blocked = blocked_corner_cells(&app.gs, p, &mv);
                app.gs.apply(p, &mv);
                app.celebrate_block(p, &blocked);
                app.finish_move();
            }
        }
    }

    draw_particles(app);
    draw_block_popup(app);

    if app.paused {
        let msg = "PAUSED — Space to resume";
        let d = measure_text(msg, None, 30, 1.0);
        let cx = BOARD_X + (N as f32 * CELL - d.width) / 2.0;
        let cy = BOARD_Y + N as f32 * CELL / 2.0;
        draw_rectangle(cx - 18.0, cy - 32.0, d.width + 36.0, 48.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text(msg, cx, cy, 30.0, GOLD);
    }

    // Toasts (dodge the popup zone while a taunt card is up).
    let dt = get_frame_time();
    let mut ty = if app.popup.is_some() { 250.0 } else { 110.0 };
    for (msg, ttl) in &mut app.toasts {
        *ttl -= dt;
        let a = (*ttl / 0.5).min(1.0);
        let dims = measure_text(msg.as_str(), None, 26, 1.0);
        let x = BOARD_X + (N as f32 * CELL - dims.width) / 2.0;
        draw_rectangle(x - 12.0, ty - 26.0, dims.width + 24.0, 38.0, Color::new(0.0, 0.0, 0.0, 0.6 * a));
        draw_text(msg, x, ty, 26.0, Color::new(1.0, 1.0, 1.0, a));
        ty += 44.0;
    }
    app.toasts.retain(|(_, ttl)| *ttl > 0.0);

    if menu_clicked {
        app.abandon_to_menu();
    }
}

// ---------------------------------------------------------------------------
// Setup screen
// ---------------------------------------------------------------------------

/// 5-row bitmap of "BLOKUS": six 4-wide letters separated by one-column gaps.
const LOGO_ROWS: [&str; 5] = [
    "###. #... .##. #..# #..# .###",
    "#..# #... #..# #.#. #..# #...",
    "###. #... #..# ##.. #..# .##.",
    "#..# #... #..# #.#. #..# ...#",
    "###. #### .##. #..# .##. ###.",
];

fn draw_logo(cx: f32, top: f32, s: f32) {
    let cols = LOGO_ROWS[0].len() as f32;
    let x0 = cx - cols * s / 2.0;
    let t = get_time() as f32;
    for (r, row) in LOGO_ROWS.iter().enumerate() {
        for (c, ch) in row.bytes().enumerate() {
            if ch == b'#' {
                let letter = c / 5;
                let bob = (t * 1.8 + letter as f32 * 0.7).sin() * 3.0;
                draw_block(x0 + c as f32 * s, top + r as f32 * s + bob, s, player_color(letter % 4));
            }
        }
    }
}

/// Generic draggable slider snapped to `step`; returns the updated value.
/// `drag` holds the id of whichever slider currently owns the mouse.
fn slider_widget(drag: &mut Option<u8>, id: u8, x: f32, y: f32, w: f32, max: f32, step: f32, mut value: f32, accent: Color) -> f32 {
    let (mx, my) = mouse_position();
    let hot = mx >= x - 14.0 && mx <= x + w + 14.0 && (my - y).abs() <= 16.0;
    if hot && is_mouse_button_pressed(MouseButton::Left) {
        *drag = Some(id);
    }
    if !is_mouse_button_down(MouseButton::Left) && *drag == Some(id) {
        *drag = None;
    }
    if *drag == Some(id) {
        let v = ((mx - x) / w * max).clamp(0.0, max);
        value = (v / step).round() * step;
    }
    let frac = (value / max).clamp(0.0, 1.0);
    // Track, fill, ticks, knob.
    draw_rectangle(x, y - 3.0, w, 6.0, Color::from_rgba(24, 27, 35, 255));
    draw_rectangle(x, y - 3.0, frac * w, 6.0, accent);
    let ticks = (max / step).round().max(1.0) as i32;
    for i in 0..=ticks {
        let tx = x + i as f32 / ticks as f32 * w;
        draw_rectangle(tx - 1.0, y + 6.0, 2.0, 5.0, Color::from_rgba(96, 104, 126, 255));
    }
    let kx = x + frac * w;
    draw_circle(kx, y, 12.0, Color::new(0.0, 0.0, 0.0, 0.4));
    draw_circle(kx, y, 11.0, Color::from_rgba(226, 232, 240, 255));
    draw_circle(kx, y, 6.5, accent);
    value
}

/// Decorative random meme card in the free top-right corner of the setup
/// screen (rolled once per screen entry, not per frame).
fn draw_setup_meme(app: &App) {
    let x = 1006.0;
    let w = 262.0;
    let y = 44.0;
    let lines = wrap_lines(&app.setup_meme, 16, w - 28.0);
    let h = 158.0 + lines.len() as f32 * 20.0 + 6.0;
    draw_rectangle(x + 3.0, y + 4.0, w, h, Color::new(0.0, 0.0, 0.0, 0.3));
    draw_rectangle(x, y, w, h, Color::new(0.06, 0.07, 0.11, 0.85));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(90, 96, 116, 255));
    draw_text("random meme", x + 14.0, y + 20.0, 14.0, Color::from_rgba(110, 118, 140, 255));
    cartoons::draw(app.setup_cartoon, x + w / 2.0, y + 88.0, 112.0, get_time() as f32);
    let mut ty = y + 168.0;
    for line in &lines {
        let d = measure_text(line, None, 16, 1.0);
        draw_text(line, x + (w - d.width) / 2.0, ty, 16.0, Color::from_rgba(190, 196, 210, 255));
        ty += 20.0;
    }
}

fn draw_setup(app: &mut App) {
    let cx = screen_width() / 2.0;
    draw_logo(cx, 56.0, 20.0);

    let author = "by Feng-Ting Liao";
    let d = measure_text(author, None, 34, 1.0);
    draw_text(author, cx - d.width / 2.0, 216.0, 34.0, Color::from_rgba(226, 232, 240, 255));
    let sub = "Rust edition · bandit-search AI";
    let d2 = measure_text(sub, None, 21, 1.0);
    draw_text(sub, cx - d2.width / 2.0, 246.0, 21.0, GRAY);

    // Seats.
    let mut y = 288.0;
    for p in 0..4 {
        draw_block(cx - 262.0, y - 2.0, 30.0, player_color(p));
        draw_text(PLAYER_NAMES[p], cx - 218.0, y + 21.0, 28.0, WHITE);
        if button(cx - 90.0, y - 5.0, 110.0, 36.0, "Human", !app.is_ai[p]) {
            app.is_ai[p] = false;
        }
        if button(cx + 30.0, y - 5.0, 110.0, 36.0, "AI", app.is_ai[p]) {
            app.is_ai[p] = true;
        }
        y += 52.0;
    }

    // Dials: AI think time (left) and visual craziness (right).
    y += 14.0;
    let sw2 = 320.0;
    let lx = cx - 385.0;
    let rx = cx + 65.0;
    let ink = Color::from_rgba(226, 232, 240, 255);
    let tl = if app.think_time <= 0.0 {
        "AI think time: instant (greedy)".to_string()
    } else {
        format!("AI think time: {:.1} s per move", app.think_time)
    };
    let d = measure_text(&tl, None, 22, 1.0);
    draw_text(&tl, lx + (sw2 - d.width) / 2.0, y + 6.0, 22.0, ink);
    let cl = format!("Visual craziness: {:.0}%", app.craziness * 100.0);
    let d = measure_text(&cl, None, 22, 1.0);
    draw_text(&cl, rx + (sw2 - d.width) / 2.0, y + 6.0, 22.0, ink);
    let v = slider_widget(&mut app.drag_slider, 0, lx, y + 32.0, sw2, THINK_MAX, 0.5, app.think_time, Color::from_rgba(96, 165, 250, 255));
    app.think_time = v;
    let v = slider_widget(&mut app.drag_slider, 1, rx, y + 32.0, sw2, CRAZY_MAX, 0.25, app.craziness, Color::from_rgba(250, 128, 90, 255));
    app.craziness = v;
    draw_text("0 (instant)", lx, y + 62.0, 16.0, GRAY);
    let d = measure_text("5.0 s", None, 16, 1.0);
    draw_text("5.0 s", lx + sw2 - d.width, y + 62.0, 16.0, GRAY);
    draw_text("zen", rx, y + 62.0, 16.0, GRAY);
    let d = measure_text("LUDICROUS", None, 16, 1.0);
    draw_text("LUDICROUS", rx + sw2 - d.width, y + 62.0, 16.0, GRAY);
    let cap = "how long the AI may think per move, seconds";
    let cd = measure_text(cap, None, 16, 1.0);
    draw_text(cap, lx + (sw2 - cd.width) / 2.0, y + 84.0, 16.0, Color::from_rgba(120, 128, 148, 255));
    let cap2 = "fireworks, confetti, memes: how much?";
    let cd2 = measure_text(cap2, None, 16, 1.0);
    draw_text(cap2, rx + (sw2 - cd2.width) / 2.0, y + 84.0, 16.0, Color::from_rgba(120, 128, 148, 255));

    // Presets + start + exit.
    y += 110.0;
    if button(cx - 335.0, y, 200.0, 42.0, "You vs 3 AI", false) {
        app.is_ai = [false, true, true, true];
    }
    if button(cx - 120.0, y, 200.0, 42.0, "Watch 4 AI", false) {
        app.is_ai = [true; 4];
    }
    if button(cx + 95.0, y, 110.0, 42.0, "Start", true) || is_key_pressed(KeyCode::Enter) {
        app.start_game();
    }
    if button(cx + 220.0, y, 110.0, 42.0, "Exit", false) {
        std::process::exit(0);
    }

    let rules = "First piece must cover your corner. Same color touches corner-to-corner only.";
    let rd = measure_text(rules, None, 20, 1.0);
    draw_text(rules, cx - rd.width / 2.0, y + 84.0, 20.0, GRAY);
    let hint = "press Enter to start";
    let hd = measure_text(hint, None, 18, 1.0);
    draw_text(hint, cx - hd.width / 2.0, y + 110.0, 18.0, Color::from_rgba(110, 118, 140, 255));

    draw_setup_meme(app);
}

// ---------------------------------------------------------------------------
// Game over
// ---------------------------------------------------------------------------

/// Stateless confetti: every particle's position is a pure function of time.
/// Density scales with the craziness dial.
fn draw_confetti(cz: f32) {
    let t = get_time() as f32;
    let sw = screen_width();
    let sh = screen_height();
    let count = (140.0 * cz).round().min(300.0) as u32;
    for i in 0..count {
        let speed = 70.0 + hnoise(i, 1) * 170.0;
        let sway = (t * (1.2 + hnoise(i, 5) * 2.2) + hnoise(i, 6) * 6.3).sin() * (12.0 + hnoise(i, 7) * 26.0);
        let x = hnoise(i, 2) * sw + sway;
        let y = (hnoise(i, 3) * sh + t * speed) % (sh + 40.0) - 20.0;
        let rot = (t * (90.0 + hnoise(i, 4) * 220.0) * if hnoise(i, 8) > 0.5 { 1.0 } else { -1.0 }).to_radians();
        let c = player_color((i % 4) as usize);
        draw_rectangle_ex(
            x,
            y,
            7.0 + hnoise(i, 9) * 6.0,
            4.5 + hnoise(i, 10) * 4.0,
            DrawRectangleParams { rotation: rot, color: Color { a: 0.9, ..c }, ..Default::default() },
        );
    }
}

/// Continuous winner fireworks + meme rotation, run every game-over frame.
fn update_game_over(app: &mut App) {
    let dt = get_frame_time().min(0.05);
    // Launch rate, volley size, and spark counts all scale with craziness
    // (clamped away from 0 so zen mode stays sparse instead of dividing by 0).
    let cz = app.craziness;
    app.fw_timer -= dt;
    while app.fw_timer <= 0.0 {
        app.fw_timer += (0.05 + app.rf() * 0.11) / cz.max(0.12);
        // Rockets across the whole window, biased to the winner's color.
        let n = if app.rf() < 0.3 * cz { 2 } else { 1 };
        for _ in 0..n {
            let color = if app.rf() < 0.55 {
                player_color(app.winner)
            } else {
                player_color((app.rng.next() % 4) as usize)
            };
            let sparks = ((45.0 + app.rf() * 50.0) * (0.35 + 0.65 * cz)) as u32;
            let x = app.rf() * screen_width();
            app.launch_rocket(x, screen_height() + 12.0, color, sparks);
        }
    }
    app.meme_timer -= dt;
    if app.meme_timer <= 0.0 {
        app.pick_meme();
    }
    app.update_particles();
}

/// Greedy word wrap for the meme card.
fn wrap_lines(text: &str, size: u16, max_w: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        let cand = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
        if !cur.is_empty() && measure_text(&cand, None, size, 1.0).width > max_w {
            lines.push(cur);
            cur = word.to_string();
        } else {
            cur = cand;
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Speech-bubble card showing the current meme (cartoon + caption, re-rolled
/// together every ~4 s), tail pointing up at the ranking panel.
fn draw_meme_card(app: &App, x: f32, y: f32, w: f32) {
    let text_x = x + 190.0;
    let text_w = w - 210.0;
    let lines = wrap_lines(&app.meme_text, 23, text_w);
    let h = (70.0 + lines.len() as f32 * 27.0).max(172.0);
    let wc = player_color(app.winner);
    let fade = ((4.0 - app.meme_timer) / 0.35).clamp(0.0, 1.0);
    draw_rectangle(x + 3.0, y + 4.0, w, h, Color::new(0.0, 0.0, 0.0, 0.35));
    draw_rectangle(x, y, w, h, Color::from_rgba(15, 18, 27, 244));
    draw_rectangle_lines(x, y, w, h, 2.0, Color { a: 0.8, ..wc });
    draw_triangle(
        vec2(x + 58.0, y + 1.0),
        vec2(x + 94.0, y + 1.0),
        vec2(x + 76.0, y - 15.0),
        Color::from_rgba(15, 18, 27, 244),
    );
    draw_text("meme of the moment", x + 16.0, y + 22.0, 15.0, Color::from_rgba(110, 118, 140, 255));
    // Cartoon on the left, caption centered in the remaining width.
    cartoons::draw(app.meme_cartoon, x + 98.0, y + h / 2.0 + 8.0, 142.0, get_time() as f32);
    let text_h = lines.len() as f32 * 27.0;
    let mut ty = y + (h - text_h) / 2.0 + 20.0;
    for line in &lines {
        let d = measure_text(line, None, 23, 1.0);
        draw_text(line, text_x + (text_w - d.width) / 2.0, ty, 23.0, Color::new(1.0, 1.0, 1.0, fade));
        ty += 27.0;
    }
}

fn draw_game_over(app: &mut App) {
    draw_header(app);
    draw_board(app);
    draw_player_cards(app);
    // Celebration layers go behind the panel so the ranking stays readable.
    draw_confetti(app.craziness);
    draw_particles(app);
    let bx = BOARD_X + 60.0;
    let by = BOARD_Y + 90.0;
    draw_rectangle(bx - 30.0, by - 60.0, 560.0, 440.0, Color::new(0.05, 0.06, 0.09, 0.93));
    let t = get_time() as f32;
    draw_rectangle_lines(bx - 30.0, by - 60.0, 560.0, 440.0, 2.0, Color::new(0.7, 0.75, 0.88, 0.6 + 0.3 * (t * 2.0).sin()));
    draw_text("Game Over", bx, by, 52.0, WHITE);

    let mut ranked: Vec<usize> = (0..4).collect();
    ranked.sort_by_key(|&p| -app.gs.score(p));
    let mut y = by + 60.0;
    for (i, &p) in ranked.iter().enumerate() {
        let score = app.gs.score(p);
        draw_block(bx, y - 22.0, 26.0, player_color(p));
        let bonus = match (app.gs.squares_remaining(p), app.gs.last_piece[p]) {
            (0, Some(0)) => "all pieces + monomino last!",
            (0, _) => "all pieces placed!",
            _ => "",
        };
        let main = format!("{}. {}  {}", i + 1, PLAYER_NAMES[p], score);
        draw_text(&main, bx + 38.0, y, 30.0, if i == 0 { GOLD } else { WHITE });
        if !bonus.is_empty() {
            let mw = measure_text(&main, None, 30, 1.0).width;
            draw_text(bonus, bx + 48.0 + mw, y - 2.0, 16.0, Color::from_rgba(240, 210, 120, 255));
        }
        y += 46.0;
    }
    draw_text(
        &format!("Winner: {}", PLAYER_NAMES[ranked[0]]),
        bx,
        y + 16.0,
        34.0,
        player_color(ranked[0]),
    );
    if button(bx, y + 44.0, 170.0, 44.0, "Play again", true) {
        app.start_game();
    }
    if button(bx + 190.0, y + 44.0, 170.0, 44.0, "Menu", false) {
        app.abandon_to_menu();
    }
    draw_meme_card(app, bx - 30.0, by + 402.0, 560.0);
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn window_conf() -> Conf {
    Conf {
        window_title: "Blokus".to_owned(),
        window_width: 1290,
        window_height: 780,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();
    app.sounds = Some(audio::load().await);
    app.roll_setup_meme();
    // Debug/CI hooks: BLOKUS_AUTO=<think-seconds-per-move> starts a 4-AI game,
    // BLOKUS_SHOT=<path> saves a screenshot at frame BLOKUS_SHOT_FRAME,
    // BLOKUS_MUTE=1 starts with sound off.
    if std::env::var("BLOKUS_MUTE").is_ok() {
        app.muted = true;
    }
    if let Ok(auto) = std::env::var("BLOKUS_AUTO") {
        app.is_ai = [true; 4];
        app.think_time = auto.parse().unwrap_or(0.0f32).clamp(0.0, THINK_MAX);
        app.start_game();
    }
    let shot_path = std::env::var("BLOKUS_SHOT").ok();
    let shot_frame: u32 = std::env::var("BLOKUS_SHOT_FRAME").ok().and_then(|v| v.parse().ok()).unwrap_or(150);
    let mut frame: u32 = 0;
    loop {
        clear_background(Color::from_rgba(23, 25, 33, 255));
        if is_key_pressed(KeyCode::M) {
            app.toggle_mute();
        }
        app.sync_music();
        draw_ambient(app.craziness);
        match app.screen {
            Screen::Setup => draw_setup(&mut app),
            Screen::Playing => {
                update_playing(&mut app);
                app.update_particles();
                draw_playing(&mut app);
            }
            Screen::GameOver => {
                update_game_over(&mut app);
                draw_game_over(&mut app);
            }
        }
        frame += 1;
        if let Some(path) = &shot_path {
            if frame == shot_frame {
                get_screen_data().export_png(path);
                println!("screenshot saved to {path}");
            }
        }
        next_frame().await;
    }
}
