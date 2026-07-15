//! Procedural cartoon gallery: animated vector-art memes drawn with
//! macroquad primitives (the game ships no image assets).
//!
//! API contract (main.rs depends on this):
//! - `COUNT`: number of distinct cartoons.
//! - `draw(id, cx, cy, s, t)`: draw cartoon `id % COUNT` centered at
//!   (cx, cy), fitting a box of roughly s x s pixels, animated by time `t`
//!   (seconds, monotonically increasing; pass `get_time() as f32`).

use macroquad::prelude::*;
use std::f32::consts::PI;

pub const COUNT: usize = 24;

pub fn draw(id: usize, cx: f32, cy: f32, s: f32, t: f32) {
    match id % COUNT {
        0 => deal_with_it(cx, cy, s, t),
        1 => trophy(cx, cy, s, t),
        2 => this_is_fine(cx, cy, s, t),
        3 => blocked_crying(cx, cy, s, t),
        4 => mind_blown(cx, cy, s, t),
        5 => flex(cx, cy, s, t),
        6 => king(cx, cy, s, t),
        7 => rip_corner(cx, cy, s, t),
        8 => rocket(cx, cy, s, t),
        9 => robot(cx, cy, s, t),
        10 => wizard(cx, cy, s, t),
        11 => detective(cx, cy, s, t),
        12 => high_five(cx, cy, s, t),
        13 => lonely(cx, cy, s, t),
        14 => nerd(cx, cy, s, t),
        15 => sleeping(cx, cy, s, t),
        16 => party(cx, cy, s, t),
        17 => stonks(cx, cy, s, t),
        18 => facepalm(cx, cy, s, t),
        19 => angry(cx, cy, s, t),
        20 => heart_eyes(cx, cy, s, t),
        21 => juggler(cx, cy, s, t),
        22 => ninja(cx, cy, s, t),
        _ => gg(cx, cy, s, t),
    }
}

// ---------------------------------------------------------------------------
// Palette + shared helpers
// ---------------------------------------------------------------------------

const BLUE: Color = Color::new(0.231, 0.510, 0.965, 1.0);
const YELLOW: Color = Color::new(0.980, 0.800, 0.082, 1.0);
const RED: Color = Color::new(0.937, 0.267, 0.267, 1.0);
const GREEN: Color = Color::new(0.133, 0.773, 0.369, 1.0);
const INK: Color = Color::new(0.07, 0.08, 0.11, 1.0);
const GRAY: Color = Color::new(0.45, 0.48, 0.55, 1.0);

fn lighten(c: Color, k: f32) -> Color {
    Color::new((c.r + k).min(1.0), (c.g + k).min(1.0), (c.b + k).min(1.0), c.a)
}

fn dim(c: Color, k: f32) -> Color {
    Color::new(c.r * k, c.g * k, c.b * k, c.a)
}

/// Deterministic pseudo-random in [0, 1) from a small integer seed.
fn hrand(i: u32) -> f32 {
    ((i.wrapping_mul(2654435761).wrapping_add(1013904223) >> 8) % 10000) as f32 / 10000.0
}

/// A beveled game block: drop shadow, body, light top bevel, dark bottom edge.
fn block(x: f32, y: f32, w: f32, h: f32, c: Color) {
    let e = (w.min(h) * 0.16).max(2.0);
    draw_rectangle(x + w * 0.06, y + h * 0.07, w, h, Color::new(0.0, 0.0, 0.0, 0.30));
    draw_rectangle(x, y, w, h, c);
    draw_rectangle(x, y, w, e, Color { a: 0.85, ..lighten(c, 0.22) });
    draw_rectangle(x, y, e * 0.65, h, Color { a: 0.5, ..lighten(c, 0.14) });
    draw_rectangle(x, y + h - e, w, e, Color { a: 0.7, ..dim(c, 0.6) });
    draw_rectangle_lines(x, y, w, h, (w.min(h) * 0.045).max(1.5), dim(c, 0.5));
}

/// Pair of cartoon eyes with a periodic blink. `look` shifts the pupils.
fn eyes(cx: f32, cy: f32, s: f32, t: f32, phase: f32, look: (f32, f32)) {
    let sep = s * 0.15;
    let r = s * 0.085;
    let blink = ((t * 0.8 + phase) % 3.3) < 0.13;
    for k in [-1.0f32, 1.0] {
        let ex = cx + k * sep;
        if blink {
            draw_line(ex - r, cy, ex + r, cy, (s * 0.028).max(1.5), INK);
        } else {
            draw_circle(ex, cy, r, WHITE);
            draw_circle(ex + look.0 * r * 0.4, cy + r * 0.15 + look.1 * r * 0.35, r * 0.5, INK);
        }
    }
}

/// Arc drawn as short line segments (angles in radians; screen y is down,
/// so 0..PI is the lower half — a smile).
fn arc(cx: f32, cy: f32, r: f32, a0: f32, a1: f32, th: f32, color: Color) {
    let n = 10;
    let mut px = cx + a0.cos() * r;
    let mut py = cy + a0.sin() * r;
    for i in 1..=n {
        let a = a0 + (a1 - a0) * i as f32 / n as f32;
        let (x, y) = (cx + a.cos() * r, cy + a.sin() * r);
        draw_line(px, py, x, y, th, color);
        px = x;
        py = y;
    }
}

fn smile(cx: f32, cy: f32, r: f32, th: f32) {
    arc(cx, cy - r * 0.35, r, 0.45, PI - 0.45, th, INK);
}

fn frown(cx: f32, cy: f32, r: f32, th: f32) {
    arc(cx, cy + r * 0.75, r, PI + 0.45, 2.0 * PI - 0.45, th, INK);
}

/// Text horizontally centered on cx, baseline at y.
fn label(text: &str, cx: f32, y: f32, size: f32, color: Color) {
    let d = measure_text(text, None, size as u16, 1.0);
    draw_text(text, cx - d.width / 2.0, y, size, color);
}

/// Four-point twinkling star (a "+" that slowly shimmers).
fn sparkle(cx: f32, cy: f32, r: f32, t: f32, color: Color) {
    let pulse = 0.65 + 0.35 * (t * 4.0).sin();
    let rot = (t * 0.8).sin() * 0.25;
    for k in 0..2 {
        let a = rot + k as f32 * PI / 2.0;
        let (dx, dy) = (a.cos() * r * pulse, a.sin() * r * pulse);
        draw_line(cx - dx, cy - dy, cx + dx, cy + dy, (r * 0.28).max(1.0), color);
    }
    draw_circle(cx, cy, r * 0.16 * pulse.max(0.4), color);
}

fn heart(cx: f32, cy: f32, r: f32, color: Color) {
    draw_circle(cx - r * 0.5, cy - r * 0.3, r * 0.55, color);
    draw_circle(cx + r * 0.5, cy - r * 0.3, r * 0.55, color);
    draw_triangle(
        vec2(cx - r * 1.02, cy - r * 0.12),
        vec2(cx + r * 1.02, cy - r * 0.12),
        vec2(cx, cy + r * 0.95),
        color,
    );
}

/// Stick arm: shoulder to elbow to hand, with a round hand.
fn arm(x0: f32, y0: f32, x1: f32, y1: f32, th: f32, color: Color) {
    draw_line(x0, y0, x1, y1, th, color);
    draw_circle(x1, y1, th * 0.9, color);
}

// ---------------------------------------------------------------------------
// 0: "Deal with it" — sunglasses drop onto a smug blue block.
// ---------------------------------------------------------------------------
fn deal_with_it(cx: f32, cy: f32, s: f32, t: f32) {
    let bob = (t * 2.4).sin() * s * 0.02;
    let b = s * 0.58;
    let by = cy - s * 0.02 + bob;
    block(cx - b / 2.0, by - b / 2.0, b, b, BLUE);
    let ey = by - b * 0.13;
    eyes(cx, ey, s, t + 10.0, 0.0, (0.0, 0.0)); // hidden once glasses land
    draw_line(cx - s * 0.1, by + b * 0.22, cx + s * 0.12, by + b * 0.20, s * 0.03, INK);
    // Sunglasses slide down on a 3 s loop, then sit on the eyes.
    let prog = ((t % 3.0) * 1.6).min(1.0);
    let ease = 1.0 - (1.0 - prog) * (1.0 - prog);
    let gy = ey - (1.0 - ease) * s * 0.42;
    let gw = s * 0.145;
    draw_rectangle(cx - gw - s * 0.06, gy - s * 0.055, gw, s * 0.12, INK);
    draw_rectangle(cx + s * 0.06, gy - s * 0.055, gw, s * 0.12, INK);
    draw_line(cx - s * 0.06, gy - s * 0.02, cx + s * 0.06, gy - s * 0.02, s * 0.03, INK);
    draw_line(cx - gw - s * 0.06, gy - s * 0.02, cx - b * 0.5, gy - s * 0.04, s * 0.025, INK);
    draw_line(cx + gw + s * 0.06, gy - s * 0.02, cx + b * 0.5, gy - s * 0.04, s * 0.025, INK);
    // Lens glint
    draw_line(cx + s * 0.08, gy - s * 0.03, cx + s * 0.13, gy + s * 0.02, s * 0.015, Color::new(1.0, 1.0, 1.0, 0.6));
    label("DEAL WITH IT", cx, cy + s * 0.46, s * 0.13, Color::new(1.0, 1.0, 1.0, ease));
}

// ---------------------------------------------------------------------------
// 1: Golden trophy with a proud face and a twinkle.
// ---------------------------------------------------------------------------
fn trophy(cx: f32, cy: f32, s: f32, t: f32) {
    let bob = (t * 2.0).sin() * s * 0.015;
    let y = cy + bob;
    let gold = Color::new(0.95, 0.75, 0.15, 1.0);
    // Handles
    draw_circle_lines(cx - s * 0.27, y - s * 0.16, s * 0.11, s * 0.045, dim(gold, 0.85));
    draw_circle_lines(cx + s * 0.27, y - s * 0.16, s * 0.11, s * 0.045, dim(gold, 0.85));
    // Cup (tapered)
    draw_triangle(
        vec2(cx - s * 0.22, y - s * 0.34),
        vec2(cx + s * 0.22, y - s * 0.34),
        vec2(cx, y + s * 0.14),
        gold,
    );
    draw_rectangle(cx - s * 0.22, y - s * 0.36, s * 0.44, s * 0.22, gold);
    draw_rectangle(cx - s * 0.24, y - s * 0.38, s * 0.48, s * 0.06, lighten(gold, 0.2));
    // Stem + base
    draw_rectangle(cx - s * 0.035, y + 0.10 * s, s * 0.07, s * 0.14, dim(gold, 0.8));
    draw_rectangle(cx - s * 0.17, y + 0.24 * s, s * 0.34, s * 0.08, dim(gold, 0.7));
    label("1", cx, y + s * 0.315, s * 0.12, lighten(gold, 0.3));
    // Face
    eyes(cx, y - s * 0.22, s * 0.8, t, 0.3, (0.0, 0.0));
    smile(cx, y - s * 0.06, s * 0.09, s * 0.028);
    sparkle(cx + s * 0.3, y - s * 0.4, s * 0.08, t, WHITE);
    sparkle(cx - s * 0.33, y + s * 0.1, s * 0.05, t + 1.3, WHITE);
}

// ---------------------------------------------------------------------------
// 2: "This is fine" — green block sips a mug amid flames.
// ---------------------------------------------------------------------------
fn this_is_fine(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.5;
    let by = cy - s * 0.05;
    block(cx - b / 2.0, by - b / 2.0, b, b, GREEN);
    // Half-lidded calm eyes
    let ey = by - b * 0.1;
    for k in [-1.0f32, 1.0] {
        let ex = cx + k * s * 0.12;
        draw_circle(ex, ey, s * 0.07, WHITE);
        draw_circle(ex, ey + s * 0.02, s * 0.035, INK);
        draw_rectangle(ex - s * 0.075, ey - s * 0.08, s * 0.15, s * 0.055, GREEN);
        draw_line(ex - s * 0.07, ey - s * 0.025, ex + s * 0.07, ey - s * 0.025, s * 0.02, INK);
    }
    draw_line(cx - s * 0.07, by + b * 0.24, cx + s * 0.07, by + b * 0.24, s * 0.025, INK);
    // Arm + mug with wobbling steam
    let mx = cx + b * 0.62;
    let my = by + b * 0.1;
    arm(cx + b * 0.5, by + b * 0.2, mx, my, s * 0.03, dim(GREEN, 0.8));
    draw_rectangle(mx - s * 0.05, my - s * 0.08, s * 0.11, s * 0.13, WHITE);
    draw_circle_lines(mx + s * 0.075, my - s * 0.015, s * 0.035, s * 0.02, WHITE);
    for i in 0..2 {
        let p = (t * 0.9 + i as f32 * 0.5) % 1.0;
        let sy = my - s * 0.1 - p * s * 0.14;
        let sx = mx + ((t * 4.0 + i as f32 * 2.0).sin()) * s * 0.02;
        draw_circle(sx, sy, s * 0.018, Color::new(1.0, 1.0, 1.0, 0.7 * (1.0 - p)));
    }
    // Flames along the bottom
    for i in 0..5 {
        let fx = cx - s * 0.42 + i as f32 * s * 0.21;
        let h = s * (0.16 + 0.06 * (t * 6.0 + i as f32 * 1.7).sin());
        let base = cy + s * 0.42;
        draw_triangle(vec2(fx - s * 0.09, base), vec2(fx + s * 0.09, base), vec2(fx, base - h), Color::new(0.95, 0.45, 0.1, 1.0));
        draw_triangle(vec2(fx - s * 0.05, base), vec2(fx + s * 0.05, base), vec2(fx, base - h * 0.6), Color::new(0.99, 0.8, 0.2, 1.0));
    }
    label("THIS IS FINE", cx, cy - s * 0.42, s * 0.12, Color::new(1.0, 1.0, 1.0, 0.9));
}

// ---------------------------------------------------------------------------
// 3: Cornered! A crying blue block hemmed in by gray blocks.
// ---------------------------------------------------------------------------
fn blocked_crying(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.30;
    // Victim block, large and clearly visible, shivering
    let sh = (t * 9.0).sin() * s * 0.012;
    let v = s * 0.42;
    let fx = cx - s * 0.34 + sh; // left edge
    let vy = cy - s * 0.03; // top edge
    // Gray wall blocks seal off its top and right corners (drawn behind)
    block(fx, vy - b - s * 0.01, b, b, GRAY); // above
    block(fx + v + s * 0.01, vy - b - s * 0.01, b, b, GRAY); // diagonal corner steal
    block(fx + v + s * 0.01, vy + v - b, b, b, GRAY); // right
    block(fx, vy, v, v, BLUE);
    // Worried eyes + brows + frown
    let ex = fx + v * 0.5;
    let ey = vy + v * 0.38;
    eyes(ex, ey, s * 0.68, t, 1.0, (0.4, -0.3));
    draw_line(ex - s * 0.14, ey - s * 0.11, ex - s * 0.04, ey - s * 0.08, s * 0.02, INK);
    draw_line(ex + s * 0.04, ey - s * 0.08, ex + s * 0.14, ey - s * 0.11, s * 0.02, INK);
    frown(ex, ey + s * 0.11, s * 0.06, s * 0.022);
    // Tears streaming
    for k in [-1.0f32, 1.0] {
        for i in 0..2 {
            let p = (t * 1.5 + i as f32 * 0.5 + (k + 1.0) * 0.2) % 1.0;
            let ty = ey + s * 0.06 + p * s * 0.24;
            draw_circle(ex + k * s * 0.11, ty, s * 0.028, Color::new(0.45, 0.75, 1.0, 0.9 * (1.0 - p)));
        }
    }
    label("BLOCKED!", cx, cy - s * 0.42, s * 0.15, Color::new(1.0, 0.55, 0.55, 0.85 + 0.15 * (t * 4.0).sin()));
}

// ---------------------------------------------------------------------------
// 4: Mind blown — the block's lid pops off with explosion rays.
// ---------------------------------------------------------------------------
fn mind_blown(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.5;
    let by = cy + s * 0.12;
    // Rays pulsing out of the open head
    let pulse = 0.75 + 0.25 * (t * 5.0).sin();
    for i in 0..7 {
        let a = -PI * 0.83 + i as f32 * PI * 0.11;
        let r0 = s * 0.24;
        let r1 = s * (0.34 + 0.1 * pulse + 0.03 * (t * 7.0 + i as f32).sin());
        let hx = cx;
        let hy = by - b * 0.4;
        let col = if i % 2 == 0 { Color::new(1.0, 0.6, 0.15, 1.0) } else { YELLOW };
        draw_line(hx + a.cos() * r0, hy + a.sin() * r0, hx + a.cos() * r1, hy + a.sin() * r1, s * 0.03, col);
    }
    // Body with the top edge blown open
    block(cx - b / 2.0, by - b * 0.4, b, b * 0.9, YELLOW);
    // Flying lid (dark outline so it stands out against the rays)
    let ly = by - b * 0.85 - pulse * s * 0.1;
    let rot = (t * 2.0).sin() * 0.25;
    draw_rectangle_ex(cx, ly, b * 1.05, b * 0.2, DrawRectangleParams { offset: vec2(0.5, 0.5), rotation: rot, color: dim(YELLOW, 0.75) });
    let (rc, rs) = (rot.cos(), rot.sin());
    let (hw, hh) = (b * 0.525, b * 0.1);
    let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)];
    for i in 0..4 {
        let (x0, y0) = corners[i];
        let (x1, y1) = corners[(i + 1) % 4];
        draw_line(cx + x0 * rc - y0 * rs, ly + x0 * rs + y0 * rc, cx + x1 * rc - y1 * rs, ly + x1 * rs + y1 * rc, s * 0.018, dim(YELLOW, 0.4));
    }
    // Shocked face: wide eyes, round open mouth
    let fy = by - b * 0.05;
    for k in [-1.0f32, 1.0] {
        draw_circle(cx + k * s * 0.12, fy, s * 0.075, WHITE);
        draw_circle(cx + k * s * 0.12, fy, s * 0.032, INK);
    }
    draw_circle(cx, fy + s * 0.15, s * 0.055 + s * 0.01 * (t * 5.0).sin(), INK);
}

// ---------------------------------------------------------------------------
// 5: Flexing strong block with pulsing biceps.
// ---------------------------------------------------------------------------
fn flex(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.52;
    let by = cy + (t * 3.0).sin() * s * 0.015;
    block(cx - b / 2.0, by - b / 2.0, b, b, RED);
    // Determined face
    let fy = by - b * 0.1;
    eyes(cx, fy, s * 0.7, t, 2.0, (0.0, 0.0));
    draw_line(cx - s * 0.15, fy - s * 0.11, cx - s * 0.04, fy - s * 0.075, s * 0.022, INK);
    draw_line(cx + s * 0.04, fy - s * 0.075, cx + s * 0.15, fy - s * 0.11, s * 0.022, INK);
    smile(cx, by + b * 0.18, s * 0.07, s * 0.025);
    // Flexing arms: big biceps bulging beside the head, fists near the top
    let flexed = 0.5 + 0.5 * (t * 4.0).sin();
    let br = s * (0.09 + 0.03 * flexed);
    for k in [-1.0f32, 1.0] {
        let sx = cx + k * b * 0.48;
        let sy = by + b * 0.12;
        let ex = cx + k * b * 0.78;
        let ey = by - b * 0.05;
        let hx = cx + k * b * 0.62;
        let hy = by - b * 0.55 - flexed * s * 0.035;
        draw_line(sx, sy, ex, ey, s * 0.05, dim(RED, 0.8));
        draw_line(ex, ey, hx, hy, s * 0.05, dim(RED, 0.8));
        draw_circle(ex - k * s * 0.01, ey, br, dim(RED, 0.9)); // bicep bulge
        draw_circle(ex - k * s * 0.035, ey - s * 0.02, br * 0.4, lighten(RED, 0.15)); // highlight
        draw_circle(hx, hy, s * 0.055, dim(RED, 0.8)); // fist
    }
    label("GG", cx, cy + s * 0.47, s * 0.14, Color::new(1.0, 1.0, 1.0, 0.55 + 0.35 * flexed));
}

// ---------------------------------------------------------------------------
// 6: King block — crown, smug grin, royal sparkles.
// ---------------------------------------------------------------------------
fn king(cx: f32, cy: f32, s: f32, t: f32) {
    let bob = (t * 2.2).sin() * s * 0.02;
    let b = s * 0.54;
    let by = cy + s * 0.08 + bob;
    block(cx - b / 2.0, by - b / 2.0, b, b, YELLOW);
    // Crown
    let gold = Color::new(0.98, 0.8, 0.2, 1.0);
    let cyy = by - b * 0.5 - s * 0.02;
    draw_rectangle(cx - b * 0.4, cyy - s * 0.055, b * 0.8, s * 0.06, gold);
    for i in 0..3 {
        let px = cx - b * 0.3 + i as f32 * b * 0.3;
        draw_triangle(vec2(px - b * 0.13, cyy - s * 0.05), vec2(px + b * 0.13, cyy - s * 0.05), vec2(px, cyy - s * 0.22), gold);
        draw_circle(px, cyy - s * 0.20, s * 0.025, RED);
    }
    // Smug face: lidded eyes + lopsided grin
    let fy = by - b * 0.08;
    for k in [-1.0f32, 1.0] {
        let ex = cx + k * s * 0.12;
        draw_circle(ex, fy, s * 0.065, WHITE);
        draw_circle(ex + s * 0.015, fy + s * 0.012, s * 0.032, INK);
        draw_rectangle(ex - s * 0.07, fy - s * 0.075, s * 0.14, s * 0.05, YELLOW);
    }
    arc(cx + s * 0.02, by + b * 0.14, s * 0.09, 0.2, PI * 0.55, s * 0.025, INK);
    sparkle(cx - s * 0.36, cy - s * 0.3, s * 0.07, t, WHITE);
    sparkle(cx + s * 0.38, cy + s * 0.05, s * 0.055, t + 2.0, WHITE);
}

// ---------------------------------------------------------------------------
// 7: Tombstone "RIP CORNER" with a little ghost.
// ---------------------------------------------------------------------------
fn rip_corner(cx: f32, cy: f32, s: f32, t: f32) {
    let stone = Color::new(0.55, 0.57, 0.62, 1.0);
    let gx = cx - s * 0.1;
    // Ground + grass
    draw_line(cx - s * 0.45, cy + s * 0.38, cx + s * 0.45, cy + s * 0.38, s * 0.025, Color::new(0.2, 0.5, 0.25, 1.0));
    for i in 0..4 {
        let px = cx - s * 0.38 + i as f32 * s * 0.25;
        let sway = (t * 2.5 + i as f32).sin() * s * 0.015;
        draw_line(px, cy + s * 0.38, px + sway, cy + s * 0.3, s * 0.018, GREEN);
    }
    // Stone
    draw_circle(gx, cy - s * 0.12, s * 0.20, stone);
    draw_rectangle(gx - s * 0.20, cy - s * 0.12, s * 0.40, s * 0.5, stone);
    draw_rectangle(gx - s * 0.20, cy - s * 0.12, s * 0.06, s * 0.5, lighten(stone, 0.12));
    label("RIP", gx, cy - s * 0.08, s * 0.17, INK);
    label("CORNER", gx, cy + s * 0.08, s * 0.10, INK);
    draw_line(gx - s * 0.12, cy + s * 0.16, gx + s * 0.12, cy + s * 0.16, s * 0.015, dim(stone, 0.6));
    // Ghost bobbing beside the grave
    let gy = cy - s * 0.05 + (t * 2.0).sin() * s * 0.05;
    let ghx = cx + s * 0.33;
    let ga = 0.75 + 0.2 * (t * 3.0).sin();
    let gc = Color::new(1.0, 1.0, 1.0, ga);
    draw_circle(ghx, gy, s * 0.09, gc);
    draw_rectangle(ghx - s * 0.09, gy, s * 0.18, s * 0.1, gc);
    for i in 0..3 {
        draw_circle(ghx - s * 0.06 + i as f32 * s * 0.06, gy + s * 0.1, s * 0.03, gc);
    }
    draw_circle(ghx - s * 0.03, gy - s * 0.01, s * 0.014, INK);
    draw_circle(ghx + s * 0.03, gy - s * 0.01, s * 0.014, INK);
}

// ---------------------------------------------------------------------------
// 8: Rocket block blasting off.
// ---------------------------------------------------------------------------
fn rocket(cx: f32, cy: f32, s: f32, t: f32) {
    let ry = cy + (t * 3.5).sin() * s * 0.03;
    // Streaking stars
    for i in 0..5u32 {
        let p = (t * 0.8 + hrand(i)) % 1.0;
        let sx = cx - s * 0.45 + hrand(i + 10) * s * 0.9;
        let sy = cy - s * 0.45 + p * s * 0.9;
        draw_line(sx, sy, sx, sy + s * 0.06, s * 0.012, Color::new(1.0, 1.0, 1.0, 0.5 * (1.0 - p)));
    }
    // Body
    let w = s * 0.26;
    draw_rectangle(cx - w / 2.0, ry - s * 0.22, w, s * 0.4, Color::new(0.85, 0.88, 0.93, 1.0));
    draw_triangle(vec2(cx - w / 2.0, ry - s * 0.22), vec2(cx + w / 2.0, ry - s * 0.22), vec2(cx, ry - s * 0.42), RED);
    draw_triangle(vec2(cx - w / 2.0, ry + s * 0.18), vec2(cx - w / 2.0, ry - s * 0.02), vec2(cx - w * 1.1, ry + s * 0.18), RED);
    draw_triangle(vec2(cx + w / 2.0, ry + s * 0.18), vec2(cx + w / 2.0, ry - s * 0.02), vec2(cx + w * 1.1, ry + s * 0.18), RED);
    // Porthole with a tiny blue block face
    draw_circle(cx, ry - s * 0.06, s * 0.095, dim(BLUE, 0.5));
    draw_rectangle(cx - s * 0.06, ry - s * 0.12, s * 0.12, s * 0.12, BLUE);
    draw_circle(cx - s * 0.025, ry - s * 0.08, s * 0.016, WHITE);
    draw_circle(cx + s * 0.025, ry - s * 0.08, s * 0.016, WHITE);
    smile(cx, ry - s * 0.015, s * 0.028, s * 0.012);
    // Flame
    let fh = s * (0.18 + 0.07 * (t * 11.0).sin());
    draw_triangle(vec2(cx - w * 0.4, ry + s * 0.18), vec2(cx + w * 0.4, ry + s * 0.18), vec2(cx, ry + s * 0.18 + fh), Color::new(1.0, 0.55, 0.1, 1.0));
    draw_triangle(vec2(cx - w * 0.22, ry + s * 0.18), vec2(cx + w * 0.22, ry + s * 0.18), vec2(cx, ry + s * 0.18 + fh * 0.6), YELLOW);
}

// ---------------------------------------------------------------------------
// 9: Robot AI block — antenna, glowing eyes, equalizer mouth.
// ---------------------------------------------------------------------------
fn robot(cx: f32, cy: f32, s: f32, t: f32) {
    let steel = Color::new(0.52, 0.58, 0.70, 1.0);
    let by = cy + s * 0.06 + (t * 2.0).sin() * s * 0.015;
    let b = s * 0.54;
    block(cx - b / 2.0, by - b / 2.0, b, b, steel);
    // Antenna with blinking bulb
    draw_line(cx, by - b / 2.0, cx, by - b / 2.0 - s * 0.16, s * 0.025, dim(steel, 0.7));
    let on = 0.4 + 0.6 * ((t * 3.0).sin() * 0.5 + 0.5);
    draw_circle(cx, by - b / 2.0 - s * 0.19, s * 0.045, Color::new(1.0, 0.3, 0.3, on));
    draw_circle(cx, by - b / 2.0 - s * 0.19, s * 0.08, Color::new(1.0, 0.3, 0.3, on * 0.25));
    // Square LED eyes flickering cyan
    let glow = 0.7 + 0.3 * (t * 8.0).sin();
    for k in [-1.0f32, 1.0] {
        draw_rectangle(cx + k * s * 0.14 - s * 0.055, by - b * 0.16, s * 0.11, s * 0.1, INK);
        draw_rectangle(cx + k * s * 0.14 - s * 0.035, by - b * 0.14, s * 0.07, s * 0.06, Color::new(0.3, 0.95, 1.0, glow));
    }
    // Equalizer mouth
    for i in 0..5 {
        let h = s * (0.03 + 0.035 * ((t * 6.0 + i as f32 * 1.3).sin() * 0.5 + 0.5));
        draw_rectangle(cx - s * 0.13 + i as f32 * s * 0.06, by + b * 0.22 - h, s * 0.035, h, Color::new(0.3, 0.95, 1.0, 0.9));
    }
    // Side bolts
    draw_circle(cx - b * 0.5, by, s * 0.035, dim(steel, 0.6));
    draw_circle(cx + b * 0.5, by, s * 0.035, dim(steel, 0.6));
}

// ---------------------------------------------------------------------------
// 10: Wizard block casting sparkles with a wand.
// ---------------------------------------------------------------------------
fn wizard(cx: f32, cy: f32, s: f32, t: f32) {
    let purple = Color::new(0.55, 0.35, 0.85, 1.0);
    let b = s * 0.5;
    let by = cy + s * 0.12 + (t * 2.0).sin() * s * 0.015;
    block(cx - b / 2.0, by - b / 2.0, b, b, BLUE);
    // Hat
    let hy = by - b * 0.5;
    draw_rectangle(cx - b * 0.55, hy - s * 0.02, b * 1.1, s * 0.05, purple);
    draw_triangle(vec2(cx - b * 0.33, hy), vec2(cx + b * 0.33, hy), vec2(cx + b * 0.08, hy - s * 0.3), purple);
    sparkle(cx - s * 0.02, hy - s * 0.15, s * 0.035, t * 1.5, YELLOW);
    // Face
    eyes(cx, by - b * 0.05, s * 0.65, t, 0.5, (0.6, -0.2));
    smile(cx, by + b * 0.2, s * 0.06, s * 0.022);
    // Wand + orbiting magic
    let wx = cx + b * 0.85;
    let wy = by - b * 0.35;
    arm(cx + b * 0.5, by + b * 0.1, cx + b * 0.72, by - b * 0.12, s * 0.03, dim(BLUE, 0.8));
    draw_line(cx + b * 0.72, by - b * 0.12, wx, wy, s * 0.022, Color::new(0.55, 0.35, 0.2, 1.0));
    sparkle(wx, wy, s * 0.06, t * 2.0, WHITE);
    for i in 0..3 {
        let a = t * 3.0 + i as f32 * 2.1;
        let r = s * 0.1;
        draw_circle(wx + a.cos() * r, wy + a.sin() * r * 0.7, s * 0.02, Color::new(1.0, 0.9, 0.4, 0.85));
    }
}

// ---------------------------------------------------------------------------
// 11: Detective block scanning for open corners.
// ---------------------------------------------------------------------------
fn detective(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.52;
    let by = cy + s * 0.06;
    block(cx - b / 2.0, by - b / 2.0, b, b, YELLOW);
    // Hat: dome + brim
    draw_rectangle(cx - b * 0.58, by - b * 0.5 - s * 0.02, b * 1.16, s * 0.045, Color::new(0.45, 0.3, 0.2, 1.0));
    draw_rectangle(cx - b * 0.34, by - b * 0.5 - s * 0.14, b * 0.68, s * 0.125, Color::new(0.55, 0.38, 0.25, 1.0));
    // Glass sweeps back and forth in front
    let sweep = (t * 1.6).sin();
    let gx = cx - b * 0.15 + sweep * s * 0.12;
    let gy = by - b * 0.02;
    let gr = s * 0.14;
    // Face: pupils track the glass
    eyes(cx, by - b * 0.12, s * 0.65, t, 1.5, (sweep * 0.9, 0.4));
    draw_line(cx - s * 0.06, by + b * 0.24, cx + s * 0.08, by + b * 0.24, s * 0.022, INK);
    // Magnifier (lens tint + rim + handle)
    draw_circle(gx, gy, gr, Color::new(0.75, 0.9, 1.0, 0.35));
    draw_circle_lines(gx, gy, gr, s * 0.03, Color::new(0.35, 0.25, 0.15, 1.0));
    draw_line(gx + gr * 0.7, gy + gr * 0.7, gx + gr * 1.5, gy + gr * 1.5, s * 0.04, Color::new(0.35, 0.25, 0.15, 1.0));
    // Question mark it is inspecting
    label("?", gx, gy + s * 0.045, s * 0.15, Color::new(1.0, 1.0, 1.0, 0.6 + 0.4 * (t * 3.0).sin().abs()));
}

// ---------------------------------------------------------------------------
// 12: Two blocks high-fiving with an impact star.
// ---------------------------------------------------------------------------
fn high_five(cx: f32, cy: f32, s: f32, t: f32) {
    let hop = ((t * 4.0).sin().max(0.0)) * s * 0.04;
    let b = s * 0.36;
    let ly = cy + s * 0.12 - hop;
    let lx = cx - s * 0.26;
    let rx = cx + s * 0.26;
    // Bodies leaning inward
    draw_rectangle_ex(lx, ly, b, b, DrawRectangleParams { offset: vec2(0.5, 0.5), rotation: 0.18, color: BLUE });
    draw_rectangle_ex(rx, ly, b, b, DrawRectangleParams { offset: vec2(0.5, 0.5), rotation: -0.18, color: GREEN });
    // Faces
    for (fx, k) in [(lx, 1.0f32), (rx, -1.0f32)] {
        let fy = ly - b * 0.08;
        draw_circle(fx - k * s * 0.0 - s * 0.055, fy, s * 0.045, WHITE);
        draw_circle(fx + s * 0.055, fy, s * 0.045, WHITE);
        draw_circle(fx - s * 0.055 + k * s * 0.02, fy + s * 0.01, s * 0.022, INK);
        draw_circle(fx + s * 0.055 + k * s * 0.02, fy + s * 0.01, s * 0.022, INK);
        smile(fx, fy + s * 0.11, s * 0.045, s * 0.018);
    }
    // Arms meeting at the top
    let hx = cx;
    let hy = cy - s * 0.22 - hop;
    arm(lx + b * 0.3, ly - b * 0.3, hx - s * 0.03, hy, s * 0.032, dim(BLUE, 0.8));
    arm(rx - b * 0.3, ly - b * 0.3, hx + s * 0.03, hy, s * 0.032, dim(GREEN, 0.8));
    // Impact star
    let ir = s * (0.09 + 0.045 * (t * 4.0).sin().max(0.0));
    for i in 0..4 {
        let a = i as f32 * PI / 4.0 + 0.4;
        draw_line(hx - a.cos() * ir, hy - s * 0.04 - a.sin() * ir, hx + a.cos() * ir, hy - s * 0.04 + a.sin() * ir, s * 0.02, YELLOW);
    }
    // Lower arms
    draw_line(lx - b * 0.3, ly + b * 0.1, lx - b * 0.55, ly + b * 0.35, s * 0.03, dim(BLUE, 0.8));
    draw_line(rx + b * 0.3, ly + b * 0.1, rx + b * 0.55, ly + b * 0.35, s * 0.03, dim(GREEN, 0.8));
}

// ---------------------------------------------------------------------------
// 13: Sad lonely monomino under a spotlight.
// ---------------------------------------------------------------------------
fn lonely(cx: f32, cy: f32, s: f32, t: f32) {
    // Dark vignette
    draw_rectangle(cx - s * 0.5, cy - s * 0.5, s, s, Color::new(0.02, 0.02, 0.05, 0.55));
    // Spotlight cone flickering gently, with a bright pool on the floor
    let a = 0.14 + 0.03 * (t * 1.5).sin();
    draw_triangle(vec2(cx - s * 0.06, cy - s * 0.5), vec2(cx + s * 0.06, cy - s * 0.5), vec2(cx - s * 0.3, cy + s * 0.36), Color::new(1.0, 1.0, 0.85, a));
    draw_triangle(vec2(cx + s * 0.06, cy - s * 0.5), vec2(cx + s * 0.3, cy + s * 0.36), vec2(cx - s * 0.3, cy + s * 0.36), Color::new(1.0, 1.0, 0.85, a));
    draw_rectangle(cx - s * 0.32, cy + s * 0.34, s * 0.64, s * 0.05, Color::new(1.0, 1.0, 0.85, a * 1.4));
    // Tiny shivering block
    let sh = (t * 7.0).sin() * s * 0.008;
    let b = s * 0.26;
    let by = cy + s * 0.2;
    block(cx - b / 2.0 + sh, by - b / 2.0, b, b, GRAY);
    let fy = by - b * 0.1;
    for k in [-1.0f32, 1.0] {
        draw_circle(cx + sh + k * s * 0.055, fy, s * 0.035, WHITE);
        draw_circle(cx + sh + k * s * 0.055, fy + s * 0.012, s * 0.018, INK);
    }
    frown(cx + sh, fy + s * 0.07, s * 0.035, s * 0.015);
    // One slow tear
    let p = (t * 0.7) % 1.0;
    draw_circle(cx + sh - s * 0.055, fy + s * 0.04 + p * s * 0.12, s * 0.016, Color::new(0.45, 0.75, 1.0, 1.0 - p));
    label("no corners left", cx, cy - s * 0.38, s * 0.10, Color::new(0.8, 0.8, 0.9, 0.7 + 0.2 * (t * 2.0).sin()));
}

// ---------------------------------------------------------------------------
// 14: Nerd block pushing up its glasses.
// ---------------------------------------------------------------------------
fn nerd(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.54;
    let by = cy + s * 0.02 + (t * 2.3).sin() * s * 0.015;
    block(cx - b / 2.0, by - b / 2.0, b, b, GREEN);
    let fy = by - b * 0.1;
    // Glasses bounce up when "pushed" on a 2.5 s cycle
    let push = (((t % 2.5) * 4.0).min(PI)).sin();
    let gy = fy - push * s * 0.025;
    // Eyes behind big round glasses
    for k in [-1.0f32, 1.0] {
        let ex = cx + k * s * 0.13;
        draw_circle(ex, gy, s * 0.1, Color::new(1.0, 1.0, 1.0, 0.9));
        draw_circle(ex + s * 0.01, gy + s * 0.015, s * 0.038, INK);
        draw_circle_lines(ex, gy, s * 0.1, s * 0.025, INK);
        // Glint
        draw_line(ex - s * 0.05, gy - s * 0.05, ex - s * 0.015, gy - s * 0.015, s * 0.014, Color::new(1.0, 1.0, 1.0, 0.9));
    }
    draw_line(cx - s * 0.03, gy, cx + s * 0.03, gy, s * 0.022, INK);
    // Pushing hand appears during the push
    if push > 0.15 {
        arm(cx + b * 0.5, by + b * 0.15, cx + s * 0.24, gy + s * 0.06, s * 0.028, dim(GREEN, 0.8));
    }
    // Buck teeth
    draw_line(cx - s * 0.07, by + b * 0.2, cx + s * 0.07, by + b * 0.2, s * 0.02, INK);
    draw_rectangle(cx - s * 0.045, by + b * 0.2, s * 0.04, s * 0.045, WHITE);
    draw_rectangle(cx + s * 0.005, by + b * 0.2, s * 0.04, s * 0.045, WHITE);
    label("ACTUALLY...", cx, cy - s * 0.42, s * 0.11, Color::new(1.0, 1.0, 1.0, 0.5 + 0.4 * push));
}

// ---------------------------------------------------------------------------
// 15: Sleeping block with floating Z Z Z.
// ---------------------------------------------------------------------------
fn sleeping(cx: f32, cy: f32, s: f32, t: f32) {
    let breathe = (t * 1.5).sin();
    let b = s * 0.54;
    let by = cy + s * 0.12 + breathe * s * 0.012;
    block(cx - b / 2.0, by - b / 2.0, b, b + breathe * s * 0.01, BLUE);
    // Nightcap
    let hy = by - b * 0.5;
    draw_triangle(vec2(cx - b * 0.4, hy + s * 0.01), vec2(cx + b * 0.28, hy + s * 0.01), vec2(cx + b * 0.55, hy - s * 0.2), RED);
    let pom = vec2(cx + b * 0.55 + (t * 2.0).sin() * s * 0.02, hy - s * 0.2 + s * 0.02);
    draw_circle(pom.x, pom.y, s * 0.045, WHITE);
    draw_rectangle(cx - b * 0.42, hy - s * 0.015, b * 0.72, s * 0.05, dim(RED, 0.85));
    // Closed eyes + tiny open mouth
    let fy = by - b * 0.05;
    for k in [-1.0f32, 1.0] {
        arc(cx + k * s * 0.12, fy - s * 0.02, s * 0.05, 0.4, PI - 0.4, s * 0.02, INK);
    }
    draw_circle(cx, by + b * 0.2, s * 0.028, INK);
    // Rising Z Z Z
    for i in 0..3 {
        let p = (t * 0.45 + i as f32 * 0.33) % 1.0;
        let zx = cx + b * 0.5 + s * 0.05 + p * s * 0.16 + (p * 9.0).sin() * s * 0.02;
        let zy = by - b * 0.3 - p * s * 0.34;
        let al = (1.0 - p) * (p * 8.0).min(1.0);
        draw_text("Z", zx, zy, s * (0.1 + p * 0.1), Color::new(1.0, 1.0, 1.0, al));
    }
}

// ---------------------------------------------------------------------------
// 16: Party block with a cone hat and confetti.
// ---------------------------------------------------------------------------
fn party(cx: f32, cy: f32, s: f32, t: f32) {
    let hop = ((t * 5.0).sin().abs()) * s * 0.03;
    let b = s * 0.48;
    let by = cy + s * 0.12 - hop;
    // Confetti rain
    let cols = [BLUE, YELLOW, RED, GREEN, WHITE];
    for i in 0..12u32 {
        let p = (t * (0.35 + hrand(i) * 0.3) + hrand(i + 40)) % 1.0;
        let px = cx - s * 0.48 + hrand(i + 20) * s * 0.96;
        let py = cy - s * 0.5 + p * s;
        draw_rectangle_ex(px, py, s * 0.035, s * 0.02, DrawRectangleParams {
            offset: vec2(0.5, 0.5),
            rotation: t * 4.0 + i as f32,
            color: Color { a: 0.9, ..cols[(i % 5) as usize] },
        });
    }
    block(cx - b / 2.0, by - b / 2.0, b, b, RED);
    // Cone hat with stripes
    let hy = by - b * 0.5;
    draw_triangle(vec2(cx - b * 0.28, hy), vec2(cx + b * 0.28, hy), vec2(cx, hy - s * 0.26), BLUE);
    draw_triangle(vec2(cx - b * 0.14, hy - s * 0.13), vec2(cx + b * 0.14, hy - s * 0.13), vec2(cx, hy - s * 0.26), YELLOW);
    draw_circle(cx, hy - s * 0.26, s * 0.03, YELLOW);
    // Cheering face: happy closed eyes + big open mouth
    let fy = by - b * 0.08;
    for k in [-1.0f32, 1.0] {
        arc(cx + k * s * 0.11, fy + s * 0.02, s * 0.05, PI + 0.4, 2.0 * PI - 0.4, s * 0.02, INK);
    }
    draw_circle(cx, by + b * 0.16, s * 0.06, INK);
    draw_circle(cx, by + b * 0.19, s * 0.03, Color::new(0.9, 0.4, 0.4, 1.0));
    // Arms up
    arm(cx - b * 0.5, by, cx - b * 0.8, by - b * 0.55, s * 0.03, dim(RED, 0.8));
    arm(cx + b * 0.5, by, cx + b * 0.8, by - b * 0.55, s * 0.03, dim(RED, 0.8));
}

// ---------------------------------------------------------------------------
// 17: STONKS — rising chart with a smug little block.
// ---------------------------------------------------------------------------
fn stonks(cx: f32, cy: f32, s: f32, t: f32) {
    // Axes
    let x0 = cx - s * 0.42;
    let y0 = cy + s * 0.34;
    draw_line(x0, cy - s * 0.34, x0, y0, s * 0.02, Color::new(0.7, 0.75, 0.85, 1.0));
    draw_line(x0, y0, cx + s * 0.44, y0, s * 0.02, Color::new(0.7, 0.75, 0.85, 1.0));
    // Jagged rising line, drawn progressively on a loop
    let pts = [
        (0.0f32, 0.0f32), (0.16, -0.10), (0.28, -0.04), (0.45, -0.22),
        (0.58, -0.14), (0.78, -0.42), (0.9, -0.56),
    ];
    let prog = ((t * 0.5) % 1.4).min(1.0) * (pts.len() - 1) as f32;
    let up = Color::new(0.2, 0.9, 0.45, 1.0);
    for i in 0..pts.len() - 1 {
        let seg = (prog - i as f32).clamp(0.0, 1.0);
        if seg <= 0.0 { break; }
        let (ax, ay) = (x0 + pts[i].0 * s, y0 + pts[i].1 * s * 1.1);
        let (bx2, by2) = (x0 + pts[i + 1].0 * s, y0 + pts[i + 1].1 * s * 1.1);
        draw_line(ax, ay, ax + (bx2 - ax) * seg, ay + (by2 - ay) * seg, s * 0.035, up);
    }
    // Arrow head at the tip once complete
    if prog >= (pts.len() - 1) as f32 - 0.01 {
        let (tx, ty) = (x0 + 0.9 * s, y0 - 0.56 * s * 1.1);
        let pulse = 1.0 + 0.15 * (t * 5.0).sin();
        draw_triangle(
            vec2(tx + s * 0.07 * pulse, ty - s * 0.07 * pulse),
            vec2(tx - s * 0.055 * pulse, ty - s * 0.01),
            vec2(tx + s * 0.015, ty + s * 0.06 * pulse),
            up,
        );
    }
    // Smug block watching the chart
    let b = s * 0.24;
    let bx = cx + s * 0.24;
    let by = cy + s * 0.19;
    block(bx - b / 2.0, by - b / 2.0, b, b, YELLOW);
    draw_circle(bx - s * 0.05, by - s * 0.03, s * 0.03, WHITE);
    draw_circle(bx + s * 0.05, by - s * 0.03, s * 0.03, WHITE);
    draw_circle(bx - s * 0.045, by - s * 0.025, s * 0.015, INK);
    draw_circle(bx + s * 0.055, by - s * 0.025, s * 0.015, INK);
    arc(bx + s * 0.01, by + s * 0.05, s * 0.04, 0.2, PI * 0.6, s * 0.014, INK);
    label("STONKS", cx - s * 0.1, cy - s * 0.36, s * 0.14, Color::new(1.0, 1.0, 1.0, 0.8 + 0.2 * (t * 3.0).sin()));
}

// ---------------------------------------------------------------------------
// 18: Facepalm block, shaking its head.
// ---------------------------------------------------------------------------
fn facepalm(cx: f32, cy: f32, s: f32, t: f32) {
    let shake = (t * 2.2).sin() * s * 0.025;
    let b = s * 0.54;
    let bx = cx + shake;
    let by = cy + s * 0.04;
    block(bx - b / 2.0, by - b / 2.0, b, b, YELLOW);
    let fy = by - b * 0.1;
    // Visible eye: closed, weary
    draw_line(bx + s * 0.07, fy, bx + s * 0.17, fy, s * 0.022, INK);
    draw_line(bx + s * 0.07, fy - s * 0.06, bx + s * 0.17, fy - s * 0.08, s * 0.018, INK);
    frown(bx + s * 0.03, by + b * 0.2, s * 0.06, s * 0.022);
    // Palm slaps over the other eye: rounded palm + four finger stubs on top
    let slap = (t * 2.2).sin() * s * 0.012;
    let px = bx - s * 0.11;
    let py = fy + slap;
    let hand = dim(YELLOW, 0.68);
    draw_line(bx - b * 0.5, by + b * 0.28, px - s * 0.05, py + s * 0.09, s * 0.045, hand);
    draw_circle(px, py, s * 0.085, hand);
    draw_rectangle(px - s * 0.085, py - s * 0.02, s * 0.17, s * 0.1, hand);
    for i in 0..4 {
        let fx2 = px - s * 0.066 + i as f32 * s * 0.044;
        draw_circle(fx2, py - s * 0.075, s * 0.024, hand);
        draw_rectangle(fx2 - s * 0.022, py - s * 0.075, s * 0.044, s * 0.06, hand);
    }
    // Sweat drop sliding down
    let p = (t * 0.8) % 1.0;
    draw_circle(bx + b * 0.42, by - b * 0.35 + p * s * 0.2, s * 0.028, Color::new(0.45, 0.75, 1.0, 0.9 * (1.0 - p * 0.7)));
    label("bruh", cx, cy + s * 0.46, s * 0.13, Color::new(1.0, 1.0, 1.0, 0.7));
}

// ---------------------------------------------------------------------------
// 19: Furious block venting steam.
// ---------------------------------------------------------------------------
fn angry(cx: f32, cy: f32, s: f32, t: f32) {
    let jx = (t * 17.0).sin() * s * 0.012;
    let jy = (t * 23.0).cos() * s * 0.01;
    let b = s * 0.54;
    let bx = cx + jx;
    let by = cy + s * 0.06 + jy;
    // Steam puffs rising from both top corners
    for k in [-1.0f32, 1.0] {
        for i in 0..2 {
            let p = (t * 1.1 + i as f32 * 0.5 + (k + 1.0) * 0.25) % 1.0;
            let sx = bx + k * b * 0.42 + (p * 10.0).sin() * s * 0.02;
            let sy = by - b * 0.5 - p * s * 0.22;
            draw_circle(sx, sy, s * (0.03 + p * 0.045), Color::new(0.85, 0.85, 0.9, 0.75 * (1.0 - p)));
        }
    }
    block(bx - b / 2.0, by - b / 2.0, b, b, RED);
    let fy = by - b * 0.08;
    // Angry slanted brows + glaring eyes
    for k in [-1.0f32, 1.0] {
        let ex = bx + k * s * 0.12;
        draw_circle(ex, fy, s * 0.06, WHITE);
        draw_circle(ex - k * s * 0.008, fy + s * 0.008, s * 0.03, INK);
        draw_line(ex - k * s * 0.08, fy - s * 0.1, ex + k * s * 0.045, fy - s * 0.045, s * 0.028, INK);
    }
    // Gritted teeth
    let mw = s * 0.2;
    let my = by + b * 0.16;
    draw_rectangle(bx - mw / 2.0, my, mw, s * 0.07, WHITE);
    draw_rectangle_lines(bx - mw / 2.0, my, mw, s * 0.07, s * 0.02, INK);
    draw_line(bx - mw / 6.0, my, bx - mw / 6.0, my + s * 0.07, s * 0.012, INK);
    draw_line(bx + mw / 6.0, my, bx + mw / 6.0, my + s * 0.07, s * 0.012, INK);
    // Anger vein
    let va = 0.6 + 0.4 * (t * 6.0).sin();
    draw_line(bx + b * 0.32, by - b * 0.38, bx + b * 0.42, by - b * 0.28, s * 0.025, Color::new(0.6, 0.05, 0.05, va));
    draw_line(bx + b * 0.42, by - b * 0.38, bx + b * 0.32, by - b * 0.28, s * 0.025, Color::new(0.6, 0.05, 0.05, va));
}

// ---------------------------------------------------------------------------
// 20: X-pentomino in love — heart eyes, floating hearts.
// ---------------------------------------------------------------------------
fn heart_eyes(cx: f32, cy: f32, s: f32, t: f32) {
    let bob = (t * 2.5).sin() * s * 0.02;
    let u = s * 0.21;
    let y = cy + bob;
    let pink = Color::new(0.95, 0.45, 0.6, 1.0);
    // Plus-shaped pentomino
    for (dx, dy) in [(0.0f32, 0.0f32), (-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)] {
        block(cx + dx * u - u / 2.0, y + dy * u - u / 2.0, u, u, pink);
    }
    // Pulsing heart eyes on the center block
    let pulse = 1.0 + 0.18 * (t * 4.5).sin();
    heart(cx - u * 0.30, y - u * 0.12, s * 0.068 * pulse, RED);
    heart(cx + u * 0.30, y - u * 0.12, s * 0.068 * pulse, RED);
    // Smile on the bottom cell
    smile(cx, y + u * 0.55, s * 0.06, s * 0.022);
    // Floating hearts drifting up
    for i in 0..2 {
        let p = (t * 0.55 + i as f32 * 0.5) % 1.0;
        let hx = cx + u * 1.75 * (if i == 0 { 1.0 } else { -1.0 }) + (p * 7.0).sin() * s * 0.02;
        let hy = y + u - p * s * 0.5;
        heart(hx, hy, s * 0.04, Color { a: (1.0 - p) * 0.95, ..pink });
    }
}

// ---------------------------------------------------------------------------
// 21: Juggler block keeping three tiles in the air.
// ---------------------------------------------------------------------------
fn juggler(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.44;
    let by = cy + s * 0.18;
    block(cx - b / 2.0, by - b / 2.0, b, b, GREEN);
    // Juggled tiles on a circular path above
    let cols = [BLUE, YELLOW, RED];
    let mut top = (cx, cy);
    let mut top_y = f32::MAX;
    for i in 0..3 {
        let a = t * 2.6 + i as f32 * 2.0 * PI / 3.0;
        let px = cx + a.cos() * s * 0.26;
        let py = cy - s * 0.16 + a.sin() * s * 0.17;
        let d = s * 0.1;
        draw_rectangle_ex(px, py, d, d, DrawRectangleParams { offset: vec2(0.5, 0.5), rotation: a * 1.5, color: cols[i] });
        if py < top_y {
            top_y = py;
            top = (px, py);
        }
    }
    // Eyes track the highest tile
    let fy = by - b * 0.1;
    let look = (((top.0 - cx) / s).clamp(-1.0, 1.0), -0.8);
    eyes(cx, fy, s * 0.6, t, 0.7, look);
    smile(cx, by + b * 0.18, s * 0.055, s * 0.02);
    // Arms sway with the juggle
    let sway = (t * 2.6).sin() * s * 0.05;
    arm(cx - b * 0.5, by, cx - b * 0.72 + sway, by - b * 0.6, s * 0.03, dim(GREEN, 0.8));
    arm(cx + b * 0.5, by, cx + b * 0.72 + sway, by - b * 0.6, s * 0.03, dim(GREEN, 0.8));
}

// ---------------------------------------------------------------------------
// 22: Ninja block with a spinning shuriken.
// ---------------------------------------------------------------------------
fn ninja(cx: f32, cy: f32, s: f32, t: f32) {
    let dark = Color::new(0.30, 0.33, 0.42, 1.0);
    let b = s * 0.52;
    let by = cy + s * 0.04 + (t * 2.8).sin() * s * 0.015;
    block(cx - b / 2.0, by - b / 2.0, b, b, dark);
    // Headband + flapping tails
    let hy = by - b * 0.22;
    draw_rectangle(cx - b / 2.0, hy - s * 0.045, b, s * 0.09, RED);
    let flap1 = (t * 5.0).sin() * s * 0.05;
    let flap2 = (t * 5.0 + 1.2).sin() * s * 0.05;
    draw_line(cx + b / 2.0, hy, cx + b / 2.0 + s * 0.16, hy - s * 0.07 + flap1, s * 0.035, RED);
    draw_line(cx + b / 2.0, hy, cx + b / 2.0 + s * 0.13, hy + s * 0.05 + flap2, s * 0.035, RED);
    // Focused eyes peeking over the band
    for k in [-1.0f32, 1.0] {
        let ex = cx + k * s * 0.12;
        draw_circle(ex, hy, s * 0.055, WHITE);
        draw_rectangle(ex - s * 0.06, hy - s * 0.07, s * 0.12, s * 0.035, dark);
        draw_circle(ex + (t * 1.7).sin() * s * 0.02, hy + s * 0.01, s * 0.026, INK);
    }
    // Spinning shuriken
    let sx = cx - b * 0.85;
    let sy = by - b * 0.35 + (t * 3.0).sin() * s * 0.03;
    for i in 0..4 {
        let a = t * 6.0 + i as f32 * PI / 2.0;
        draw_triangle(
            vec2(sx, sy),
            vec2(sx + (a - 0.35).cos() * s * 0.09, sy + (a - 0.35).sin() * s * 0.09),
            vec2(sx + (a + 0.35).cos() * s * 0.09, sy + (a + 0.35).sin() * s * 0.09),
            Color::new(0.75, 0.78, 0.85, 1.0),
        );
    }
    draw_circle(sx, sy, s * 0.025, dark);
    arm(cx - b * 0.5, by + b * 0.05, cx - b * 0.72, by - b * 0.2, s * 0.03, dim(dark, 0.8));
}

// ---------------------------------------------------------------------------
// 23: Winking block with a "GG!" speech bubble.
// ---------------------------------------------------------------------------
fn gg(cx: f32, cy: f32, s: f32, t: f32) {
    let b = s * 0.5;
    let by = cy + s * 0.14 + (t * 2.0).sin() * s * 0.015;
    block(cx - b / 2.0, by - b / 2.0, b, b, BLUE);
    let fy = by - b * 0.08;
    // Wink: left eye open, right eye winks on a cycle
    draw_circle(cx - s * 0.11, fy, s * 0.06, WHITE);
    draw_circle(cx - s * 0.105, fy + s * 0.012, s * 0.03, INK);
    let wink = ((t * 0.9) % 2.6) < 0.5;
    if wink {
        arc(cx + s * 0.11, fy + s * 0.01, s * 0.05, PI + 0.4, 2.0 * PI - 0.4, s * 0.02, INK);
    } else {
        draw_circle(cx + s * 0.11, fy, s * 0.06, WHITE);
        draw_circle(cx + s * 0.115, fy + s * 0.012, s * 0.03, INK);
    }
    arc(cx + s * 0.01, by + b * 0.15, s * 0.07, 0.2, PI * 0.65, s * 0.024, INK);
    // Thumbs-up arm
    arm(cx + b * 0.5, by + b * 0.1, cx + b * 0.78, by - b * 0.15, s * 0.035, dim(BLUE, 0.8));
    draw_rectangle(cx + b * 0.72, by - b * 0.38, s * 0.045, s * 0.09, dim(BLUE, 0.8));
    // Speech bubble pulsing gently
    let k = 1.0 + 0.04 * (t * 3.5).sin();
    let bw = s * 0.36 * k;
    let bh = s * 0.22 * k;
    let bx = cx - s * 0.27;
    let byy = cy - s * 0.3;
    draw_rectangle(bx - bw / 2.0, byy - bh / 2.0, bw, bh, WHITE);
    draw_triangle(vec2(bx + s * 0.02, byy + bh / 2.0), vec2(bx + s * 0.1, byy + bh / 2.0), vec2(bx + s * 0.1, byy + bh / 2.0 + s * 0.08), WHITE);
    label("GG!", bx, byy + s * 0.055 * k, s * 0.16 * k, INK);
    sparkle(cx + s * 0.34, cy - s * 0.28, s * 0.06, t, YELLOW);
}
