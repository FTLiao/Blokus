//! Blokus — macroquad UI.
//!
//! Setup screen: toggle each seat between Human and AI, then start.
//! In game: click a piece in the tray to pick it up, R (or right-click)
//! rotates, F flips, left-click on the board places. AI turns play
//! automatically; Space pauses, Up/Down changes speed.

use blokus::ai::{self, Rng};
use blokus::game::{GameState, Move, N, PLAYER_NAMES, START_CORNERS};
use blokus::pieces::{NUM_PIECES, flip_horizontal, pieces, rotate_cw};
use macroquad::prelude::*;

const CELL: f32 = 34.0;
const BOARD_X: f32 = 30.0;
const BOARD_Y: f32 = 78.0;
const TRAY_X: f32 = 748.0;
const TRAY_Y: f32 = 96.0;
const TRAY_COL_W: f32 = 96.0;
const TRAY_ROW_H: f32 = 80.0;
const SIDE_X: f32 = 1044.0;

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

enum Screen {
    Setup,
    Playing,
    GameOver,
}

struct Sel {
    piece: usize,
    cells: Vec<(i8, i8)>,
}

struct App {
    screen: Screen,
    is_ai: [bool; 4],
    gs: GameState,
    sel: Option<Sel>,
    rng: Rng,
    ai_timer: f32,
    ai_delay: f32,
    paused: bool,
    toasts: Vec<(String, f32)>,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::Setup,
            is_ai: [false, true, true, true],
            gs: GameState::new(),
            sel: None,
            rng: Rng(macroquad::miniquad::date::now().to_bits() | 1),
            ai_timer: 0.0,
            ai_delay: 0.45,
            paused: false,
            toasts: Vec::new(),
        }
    }

    fn toast(&mut self, msg: String) {
        self.toasts.push((msg, 2.2));
    }

    fn start_game(&mut self) {
        self.gs = GameState::new();
        self.sel = None;
        self.paused = false;
        self.ai_timer = 0.0;
        self.toasts.clear();
        self.screen = Screen::Playing;
    }

    fn finish_move(&mut self) {
        self.sel = None;
        self.ai_timer = 0.0;
        for p in self.gs.advance_turn() {
            self.toast(format!("{} is out of moves", PLAYER_NAMES[p]));
        }
        if self.gs.is_over() {
            self.screen = Screen::GameOver;
        }
    }
}

fn button(x: f32, y: f32, w: f32, h: f32, label: &str, active: bool) -> bool {
    let (mx, my) = mouse_position();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let bg = if active {
        if hover { Color::from_rgba(90, 105, 140, 255) } else { Color::from_rgba(70, 82, 110, 255) }
    } else {
        Color::from_rgba(48, 52, 64, 255)
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 2.0, Color::from_rgba(130, 140, 165, 255));
    let size = 24.0;
    let dims = measure_text(label, None, size as u16, 1.0);
    draw_text(label, x + (w - dims.width) / 2.0, y + h / 2.0 + dims.height / 2.0 - 2.0, size, WHITE);
    hover && is_mouse_button_pressed(MouseButton::Left)
}

fn board_cell_at(mx: f32, my: f32) -> Option<(i32, i32)> {
    let c = ((mx - BOARD_X) / CELL).floor() as i32;
    let r = ((my - BOARD_Y) / CELL).floor() as i32;
    if r >= 0 && c >= 0 && r < N as i32 && c < N as i32 { Some((r, c)) } else { None }
}

fn draw_cell(r: i32, c: i32, color: Color) {
    let x = BOARD_X + c as f32 * CELL;
    let y = BOARD_Y + r as f32 * CELL;
    draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, color);
    draw_rectangle(x + 4.0, y + 4.0, CELL - 8.0, CELL - 8.0, dim(color, 0.82));
}

fn draw_board(app: &App) {
    draw_rectangle(BOARD_X - 6.0, BOARD_Y - 6.0, N as f32 * CELL + 12.0, N as f32 * CELL + 12.0, Color::from_rgba(20, 22, 28, 255));
    for r in 0..N {
        for c in 0..N {
            let x = BOARD_X + c as f32 * CELL;
            let y = BOARD_Y + r as f32 * CELL;
            draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, Color::from_rgba(44, 48, 60, 255));
        }
    }
    for p in 0..4 {
        for r in 0..N {
            for c in 0..N {
                if app.gs.occ[p][r] >> c & 1 == 1 {
                    draw_cell(r as i32, c as i32, player_color(p));
                }
            }
        }
        // Starting-corner marker until the player has placed something.
        if app.gs.is_first_move(p) && app.gs.active[p] {
            let (r, c) = START_CORNERS[p];
            let x = BOARD_X + c as f32 * CELL + CELL / 2.0;
            let y = BOARD_Y + r as f32 * CELL + CELL / 2.0;
            draw_poly(x, y, 4, CELL * 0.28, 45.0, player_color(p));
        }
    }
    // Outline the most recent placement.
    let last_player = (app.gs.current + 3) % 4;
    if let Some(mv) = &app.gs.last_move[last_player] {
        for (r, c) in mv.cells() {
            let x = BOARD_X + c as f32 * CELL;
            let y = BOARD_Y + r as f32 * CELL;
            draw_rectangle_lines(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, 3.0, WHITE);
        }
    }
}

/// Piece thumbnail centered in a box; returns true if clicked.
fn draw_piece_thumb(piece: usize, x: f32, y: f32, w: f32, h: f32, color: Color, enabled: bool, selected: bool) -> bool {
    let (mx, my) = mouse_position();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let bg = if selected {
        Color::from_rgba(90, 100, 130, 255)
    } else if hover && enabled {
        Color::from_rgba(62, 68, 86, 255)
    } else {
        Color::from_rgba(40, 44, 56, 255)
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(90, 96, 116, 255));
    let cells = &pieces()[piece].orientations[0];
    let max_r = cells.iter().map(|c| c.0).max().unwrap() as f32 + 1.0;
    let max_c = cells.iter().map(|c| c.1).max().unwrap() as f32 + 1.0;
    let s = 13.0;
    let ox = x + (w - max_c * s) / 2.0;
    let oy = y + (h - max_r * s) / 2.0;
    let col = if enabled { color } else { dim(color, 0.3) };
    for &(r, c) in cells {
        draw_rectangle(ox + c as f32 * s + 1.0, oy + r as f32 * s + 1.0, s - 2.0, s - 2.0, col);
    }
    enabled && hover && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_tray(app: &mut App) {
    let p = app.gs.current;
    let human_turn = !app.is_ai[p] && app.gs.active[p];
    draw_text(
        &format!("{}'s pieces", PLAYER_NAMES[p]),
        TRAY_X,
        TRAY_Y - 14.0,
        26.0,
        player_color(p),
    );
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

fn draw_side_panel(app: &App) {
    let mut y = TRAY_Y;
    draw_text("Players", SIDE_X, y - 14.0, 26.0, LIGHTGRAY);
    for p in 0..4 {
        let c = player_color(p);
        draw_rectangle(SIDE_X, y, 20.0, 20.0, c);
        let marker = if p == app.gs.current && matches!(app.screen, Screen::Playing) { ">" } else { " " };
        let status = if app.gs.active[p] {
            format!("x{}", app.gs.pieces_left[p].count_ones())
        } else {
            "out".to_string()
        };
        let ai = if app.is_ai[p] { "AI" } else { "You" };
        draw_text(
            &format!("{marker} {} ({ai}) {} {status}", PLAYER_NAMES[p], app.gs.score(p)),
            SIDE_X + 28.0,
            y + 16.0,
            20.0,
            if app.gs.active[p] { WHITE } else { GRAY },
        );
        y += 34.0;
    }
    y += 30.0;
    draw_text("Controls", SIDE_X, y, 24.0, LIGHTGRAY);
    y += 26.0;
    let lines: [&str; 6] = [
        "Click piece, then board",
        "R / right-click: rotate",
        "F: flip",
        "Esc: put piece back",
        "Space: pause AI",
        "Up/Down: AI speed",
    ];
    for l in lines {
        draw_text(l, SIDE_X, y, 19.0, GRAY);
        y += 22.0;
    }
    y += 16.0;
    let speed = if app.paused { "paused".to_string() } else { format!("{:.2}s / move", app.ai_delay) };
    draw_text(&format!("AI speed: {speed}"), SIDE_X, y, 20.0, LIGHTGRAY);
}

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
    let outline = if legal {
        Color::new(1.0, 1.0, 1.0, 0.85)
    } else {
        Color::new(1.0, 0.25, 0.25, 0.6)
    };
    for (rr, cc) in mv.cells() {
        if (0..N as i32).contains(&rr) && (0..N as i32).contains(&cc) {
            let x = BOARD_X + cc as f32 * CELL;
            let y = BOARD_Y + rr as f32 * CELL;
            draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, Color { a: 0.45, ..player_color(app.gs.current) });
            draw_rectangle_lines(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, 2.0, outline);
        }
    }
    Some((mv, legal))
}

fn update_playing(app: &mut App) {
    let p = app.gs.current;

    // Global controls.
    if is_key_pressed(KeyCode::Space) {
        app.paused = !app.paused;
    }
    if is_key_pressed(KeyCode::Up) {
        app.ai_delay = (app.ai_delay - 0.15).max(0.0);
    }
    if is_key_pressed(KeyCode::Down) {
        app.ai_delay = (app.ai_delay + 0.15).min(1.5);
    }

    if app.is_ai[p] {
        if !app.paused {
            app.ai_timer += get_frame_time();
            if app.ai_timer >= app.ai_delay {
                match ai::choose_move(&app.gs, p, &mut app.rng) {
                    Some(mv) => {
                        app.gs.apply(p, &mv);
                        app.finish_move();
                    }
                    None => {
                        app.gs.active[p] = false;
                        app.finish_move();
                    }
                }
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

fn draw_playing(app: &mut App) {
    draw_text("BLOKUS", BOARD_X, 46.0, 44.0, WHITE);
    let p = app.gs.current;
    let turn_label = if app.is_ai[p] {
        format!("{} (AI) is thinking...", PLAYER_NAMES[p])
    } else {
        format!("Your turn, {}", PLAYER_NAMES[p])
    };
    draw_text(&turn_label, BOARD_X + 230.0, 46.0, 28.0, player_color(p));

    draw_board(app);
    draw_tray(app);
    draw_side_panel(app);

    // Ghost + placement for the human player.
    if !app.is_ai[p] {
        if let Some((mv, legal)) = draw_ghost(app) {
            if legal && is_mouse_button_pressed(MouseButton::Left) {
                app.gs.apply(p, &mv);
                app.finish_move();
            }
        }
    }

    // Toasts.
    let dt = get_frame_time();
    let mut ty = 110.0;
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
}

fn draw_setup(app: &mut App) {
    let cx = screen_width() / 2.0;
    let title = "BLOKUS";
    let dims = measure_text(title, None, 80, 1.0);
    draw_text(title, cx - dims.width / 2.0, 130.0, 80.0, WHITE);
    let sub = "official 20 x 20 board - Rust edition";
    let d2 = measure_text(sub, None, 24, 1.0);
    draw_text(sub, cx - d2.width / 2.0, 165.0, 24.0, GRAY);

    let mut y = 240.0;
    for p in 0..4 {
        draw_rectangle(cx - 260.0, y, 28.0, 28.0, player_color(p));
        draw_text(PLAYER_NAMES[p], cx - 220.0, y + 22.0, 28.0, WHITE);
        if button(cx - 90.0, y - 4.0, 110.0, 36.0, "Human", !app.is_ai[p]) {
            app.is_ai[p] = false;
        }
        if button(cx + 30.0, y - 4.0, 110.0, 36.0, "AI", app.is_ai[p]) {
            app.is_ai[p] = true;
        }
        y += 56.0;
    }
    y += 10.0;
    if button(cx - 260.0, y, 190.0, 40.0, "You vs 3 AI", false) {
        app.is_ai = [false, true, true, true];
    }
    if button(cx - 50.0, y, 190.0, 40.0, "Watch 4 AI", false) {
        app.is_ai = [true; 4];
    }
    if button(cx + 160.0, y, 100.0, 40.0, "Start", true) || is_key_pressed(KeyCode::Enter) {
        app.start_game();
    }
    draw_text(
        "First piece must cover your corner. Same color touches corner-to-corner only.",
        cx - 330.0,
        y + 90.0,
        20.0,
        GRAY,
    );
}

fn draw_game_over(app: &mut App) {
    draw_board(app);
    let bx = BOARD_X + 60.0;
    let by = BOARD_Y + 90.0;
    draw_rectangle(bx - 30.0, by - 60.0, 560.0, 440.0, Color::new(0.05, 0.06, 0.09, 0.93));
    draw_rectangle_lines(bx - 30.0, by - 60.0, 560.0, 440.0, 2.0, Color::from_rgba(130, 140, 165, 255));
    draw_text("Game Over", bx, by, 52.0, WHITE);

    let mut ranked: Vec<usize> = (0..4).collect();
    ranked.sort_by_key(|&p| -app.gs.score(p));
    let mut y = by + 60.0;
    for (i, &p) in ranked.iter().enumerate() {
        let score = app.gs.score(p);
        draw_rectangle(bx, y - 20.0, 24.0, 24.0, player_color(p));
        let bonus = match (app.gs.squares_remaining(p), app.gs.last_piece[p]) {
            (0, Some(0)) => "  (all pieces + monomino last!)",
            (0, _) => "  (all pieces placed!)",
            _ => "",
        };
        draw_text(
            &format!("{}. {}  {}{}", i + 1, PLAYER_NAMES[p], score, bonus),
            bx + 36.0,
            y,
            30.0,
            if i == 0 { GOLD } else { WHITE },
        );
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
        app.screen = Screen::Setup;
    }
    draw_side_panel(app);
}

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
    // Debug/CI hooks: BLOKUS_AUTO=<seconds-per-move> starts a 4-AI game,
    // BLOKUS_SHOT=<path> saves a screenshot shortly after startup.
    if let Ok(auto) = std::env::var("BLOKUS_AUTO") {
        app.is_ai = [true; 4];
        app.ai_delay = auto.parse().unwrap_or(0.0);
        app.start_game();
    }
    let shot_path = std::env::var("BLOKUS_SHOT").ok();
    let shot_frame: u32 = std::env::var("BLOKUS_SHOT_FRAME").ok().and_then(|v| v.parse().ok()).unwrap_or(150);
    let mut frame: u32 = 0;
    loop {
        clear_background(Color::from_rgba(28, 30, 38, 255));
        match app.screen {
            Screen::Setup => draw_setup(&mut app),
            Screen::Playing => {
                update_playing(&mut app);
                draw_playing(&mut app);
            }
            Screen::GameOver => draw_game_over(&mut app),
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
