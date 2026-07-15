//! Chiptune audio: a tiny software synthesizer that renders MIDI-style note
//! sequences (square lead, triangle bass, noise percussion) into WAV data at
//! startup, so the game ships music with zero binary assets.
//!
//! Tracks: menu theme, gameplay theme, victory fanfare (all seamless loops),
//! plus a "bang" sound effect for corner blocks.

use macroquad::audio::{Sound, load_sound_from_bytes};

const SR: u32 = 22_050;

fn midi_hz(n: f32) -> f32 {
    440.0 * 2f32.powf((n - 69.0) / 12.0)
}

#[derive(Clone, Copy)]
enum Wave {
    /// Square with duty cycle (0..1). Classic chip lead.
    Square(f32),
    /// Triangle. Round chip bass.
    Triangle,
    /// LFSR-ish noise. Hats and hits.
    Noise,
}

struct Note {
    chan: usize,
    beat: f32,
    dur: f32,
    pitch: f32, // MIDI note number; ignored for Noise except as brightness
}

struct Chan {
    wave: Wave,
    vol: f32,
    /// Attack and release, in seconds.
    att: f32,
    rel: f32,
}

struct Song {
    bpm: f32,
    beats: f32,
    chans: Vec<Chan>,
    notes: Vec<Note>,
}

impl Song {
    fn new(bpm: f32, beats: f32, chans: Vec<Chan>) -> Self {
        Song { bpm, beats, chans, notes: Vec::new() }
    }

    fn n(&mut self, chan: usize, beat: f32, dur: f32, pitch: f32) {
        self.notes.push(Note { chan, beat, dur, pitch });
    }

    /// Render the song to mono f32 samples (one full loop).
    fn render(&self) -> Vec<f32> {
        let spb = 60.0 / self.bpm; // seconds per beat
        let total = (self.beats * spb * SR as f32) as usize;
        let mut buf = vec![0f32; total];
        for note in &self.notes {
            let ch = &self.chans[note.chan];
            let start = (note.beat * spb * SR as f32) as usize;
            let dur_s = note.dur * spb;
            let len = (dur_s * SR as f32) as usize;
            let hz = midi_hz(note.pitch);
            let mut noise_state: u32 = 0x1234_5678 ^ (note.pitch as u32) << 3;
            let mut phase = 0f32;
            for i in 0..len {
                let idx = start + i;
                if idx >= total {
                    break;
                }
                let t = i as f32 / SR as f32;
                // Envelope: linear attack, flat sustain, linear release.
                let env = (t / ch.att).min(1.0) * ((dur_s - t) / ch.rel).clamp(0.0, 1.0);
                let s = match ch.wave {
                    Wave::Square(duty) => {
                        phase = (phase + hz / SR as f32).fract();
                        if phase < duty { 1.0 } else { -1.0 }
                    }
                    Wave::Triangle => {
                        phase = (phase + hz / SR as f32).fract();
                        4.0 * (phase - 0.5).abs() - 1.0
                    }
                    Wave::Noise => {
                        // Update the LFSR at a pitch-dependent rate for
                        // "brighter" or "darker" noise.
                        let rate = (hz / 8.0).max(1.0);
                        if (t * rate * SR as f32) as u32 != ((t - 1.0 / SR as f32) * rate * SR as f32) as u32 {
                            noise_state ^= noise_state << 13;
                            noise_state ^= noise_state >> 17;
                            noise_state ^= noise_state << 5;
                        }
                        (noise_state & 0xffff) as f32 / 32768.0 - 1.0
                    }
                };
                buf[idx] += s * env * ch.vol;
            }
        }
        buf
    }
}

/// Soft-clip and encode mono f32 samples as a 16-bit PCM WAV file.
fn to_wav(samples: &[f32]) -> Vec<u8> {
    let n = samples.len() as u32;
    let data_len = n * 2;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SR.to_le_bytes());
    out.extend_from_slice(&(SR * 2).to_le_bytes()); // byte rate
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for &s in samples {
        let c = (s.tanh() * 0.85 * i16::MAX as f32) as i16;
        out.extend_from_slice(&c.to_le_bytes());
    }
    out
}

fn lead() -> Chan {
    Chan { wave: Wave::Square(0.25), vol: 0.16, att: 0.012, rel: 0.10 }
}
fn harmony() -> Chan {
    Chan { wave: Wave::Square(0.5), vol: 0.09, att: 0.02, rel: 0.15 }
}
fn bass() -> Chan {
    Chan { wave: Wave::Triangle, vol: 0.30, att: 0.008, rel: 0.08 }
}
fn hats() -> Chan {
    Chan { wave: Wave::Noise, vol: 0.05, att: 0.002, rel: 0.03 }
}

/// Menu theme: easy-going I-V-vi-IV at 88 BPM, 16 beats.
pub fn wav_menu() -> Vec<u8> {
    let mut s = Song::new(88.0, 16.0, vec![lead(), harmony(), bass(), hats()]);
    let roots = [48.0, 43.0, 45.0, 41.0]; // C3 G2 A2 F2
    for (bar, &r) in roots.iter().enumerate() {
        let b = bar as f32 * 4.0;
        // Bass: root, fifth, octave.
        s.n(2, b, 1.6, r - 12.0);
        s.n(2, b + 2.0, 0.9, r - 5.0);
        s.n(2, b + 3.0, 0.9, r);
        // Soft hats on the offbeats.
        s.n(3, b + 1.0, 0.05, 90.0);
        s.n(3, b + 3.0, 0.05, 90.0);
    }
    // Melody (pentatonic over the chords).
    let mel: [(f32, f32, f32); 16] = [
        (0.0, 1.0, 64.0), (1.0, 0.5, 67.0), (1.5, 1.5, 72.0), (3.0, 1.0, 71.0),
        (4.0, 1.0, 67.0), (5.0, 0.5, 62.0), (5.5, 1.5, 71.0), (7.0, 1.0, 69.0),
        (8.0, 1.0, 64.0), (9.0, 0.5, 60.0), (9.5, 1.5, 69.0), (11.0, 1.0, 67.0),
        (12.0, 1.0, 65.0), (13.0, 0.5, 69.0), (13.5, 1.5, 72.0), (15.0, 1.0, 67.0),
    ];
    for (b, d, p) in mel {
        s.n(0, b, d, p);
    }
    // Whole-note harmony pads a tenth above the bass.
    for (bar, &r) in roots.iter().enumerate() {
        s.n(1, bar as f32 * 4.0, 3.6, r + 16.0);
    }
    to_wav(&s.render())
}

/// Gameplay theme A: driving vi-IV-I-V at 104 BPM, 16 beats.
pub fn wav_game() -> Vec<u8> {
    let mut s = Song::new(104.0, 16.0, vec![lead(), harmony(), bass(), hats()]);
    let roots = [45.0, 41.0, 48.0, 43.0]; // A2 F2 C3 G2
    for (bar, &r) in roots.iter().enumerate() {
        let b = bar as f32 * 4.0;
        // Arpeggiated eighth-note bass: root fifth octave fifth, twice.
        for half in 0..2 {
            let h = b + half as f32 * 2.0;
            s.n(2, h, 0.45, r - 12.0);
            s.n(2, h + 0.5, 0.45, r - 5.0);
            s.n(2, h + 1.0, 0.45, r);
            s.n(2, h + 1.5, 0.45, r - 5.0);
        }
        // Hats: downbeats soft, offbeats bright.
        for k in 0..4 {
            s.n(3, b + k as f32, 0.04, 80.0);
            s.n(3, b + k as f32 + 0.5, 0.03, 100.0);
        }
    }
    let mel: [(f32, f32, f32); 14] = [
        (0.0, 0.5, 76.0), (0.5, 0.5, 72.0), (1.0, 1.0, 74.0), (3.0, 0.5, 72.0),
        (4.0, 0.5, 72.0), (5.0, 1.0, 69.0), (6.5, 0.5, 72.0), (7.0, 0.5, 74.0),
        (8.0, 1.0, 76.0), (10.0, 0.5, 79.0), (10.5, 0.5, 76.0), (11.0, 1.0, 74.0),
        (12.0, 1.5, 71.0), (14.0, 1.5, 67.0),
    ];
    for (b, d, p) in mel {
        s.n(0, b, d, p);
    }
    to_wav(&s.render())
}

/// Gameplay theme B — CLUB: four-on-the-floor techno at 126 BPM, 16 beats.
/// Kick on every beat, open hats on the offbeats, hypnotic octave bass,
/// a two-note rave stab that refuses to leave.
pub fn wav_club() -> Vec<u8> {
    let kick = Chan { wave: Wave::Noise, vol: 0.16, att: 0.002, rel: 0.10 };
    let mut s = Song::new(126.0, 16.0, vec![lead(), harmony(), bass(), hats(), kick]);
    // Kick every beat; open hat on every offbeat.
    for k in 0..16 {
        s.n(4, k as f32, 0.28, 26.0);
        s.n(3, k as f32 + 0.5, 0.07, 115.0);
    }
    // Octave-pump bass: Am for 8 beats, F for 4, G for 4.
    let seg = [(0.0, 8.0, 45.0), (8.0, 4.0, 41.0), (12.0, 4.0, 43.0)];
    for (start, len, root) in seg {
        let mut k = 0.0;
        while k < len {
            s.n(2, start + k, 0.22, root - 12.0);
            s.n(2, start + k + 0.5, 0.22, root);
            k += 1.0;
        }
    }
    // Rave stab riff, one bar, repeated with a lift in the last bar.
    for bar in 0..4 {
        let b = bar as f32 * 4.0;
        let up = if bar == 3 { 3.0 } else { 0.0 }; // C major lift at the end
        s.n(0, b, 0.20, 81.0 + up);
        s.n(0, b + 0.5, 0.20, 81.0 + up);
        s.n(0, b + 1.5, 0.20, 79.0 + up);
        s.n(0, b + 2.0, 0.45, 76.0 + up);
        // Chord stab answer on the and-of-three.
        s.n(1, b + 3.5, 0.18, 69.0 + up);
        s.n(1, b + 3.5, 0.18, 72.0 + up);
    }
    to_wav(&s.render())
}

/// Gameplay theme C — FUNK: E7 vamp at 102 BPM, 16 beats. Syncopated bass
/// with ghost notes, sixteenth-note hats, backbeat snare, staccato horn stabs.
pub fn wav_funk() -> Vec<u8> {
    let drums = Chan { wave: Wave::Noise, vol: 0.11, att: 0.002, rel: 0.06 };
    let mut s = Song::new(102.0, 16.0, vec![lead(), harmony(), bass(), hats(), drums]);
    for bar in 0..4 {
        let b = bar as f32 * 4.0;
        // The pocket: kick on 1 and 3, snare crack on 2 and 4.
        s.n(4, b, 0.12, 38.0);
        s.n(4, b + 2.0, 0.12, 38.0);
        s.n(4, b + 1.0, 0.09, 72.0);
        s.n(4, b + 3.0, 0.09, 72.0);
        // Sixteenth hats, accents on the e's.
        for k in 0..16 {
            let t = b + k as f32 * 0.25;
            s.n(3, t, if k % 4 == 1 { 0.05 } else { 0.02 }, 100.0);
        }
        // Slap-style bass line on E (E2=40): root pops, octave ghosts, the
        // b7 walk-up that makes it funk.
        s.n(2, b, 0.30, 40.0);
        s.n(2, b + 0.75, 0.15, 52.0);
        s.n(2, b + 1.5, 0.30, 47.0);
        s.n(2, b + 2.25, 0.15, 50.0);
        s.n(2, b + 2.5, 0.30, 52.0);
        s.n(2, b + 3.25, 0.20, 50.0);
        s.n(2, b + 3.75, 0.20, 38.0);
        // Horn-section stab (E9 flavor) on the and-of-one.
        if bar % 2 == 0 {
            s.n(0, b + 1.75, 0.14, 68.0);
            s.n(1, b + 1.75, 0.14, 74.0);
        }
    }
    // One cheeky lick to turn the loop around.
    for (b, d, p) in [(13.5, 0.2, 79.0), (13.75, 0.2, 78.0), (14.0, 0.35, 76.0), (14.5, 0.6, 74.0)] {
        s.n(0, b, d, p);
    }
    to_wav(&s.render())
}

/// Gameplay theme D — LINDY HOP: swing in C at 152 BPM, 16 beats.
/// Walking bass, swung ride (offbeats at the 2/3 point), a bouncing
/// six-th-flavored melody over ii-V-I.
pub fn wav_swing() -> Vec<u8> {
    let mut s = Song::new(152.0, 16.0, vec![lead(), harmony(), bass(), hats()]);
    // Walking bass, quarter notes: Dm7 / G7 / Cmaj / A7 turnaround.
    let walk: [f32; 16] = [
        50.0, 53.0, 57.0, 55.0, // D F A G   (Dm7)
        43.0, 47.0, 50.0, 53.0, // G B D F   (G7)
        48.0, 52.0, 55.0, 57.0, // C E G A   (C6)
        45.0, 49.0, 52.0, 43.0, // A C# E G  (A7 back to the top)
    ];
    for (i, &p) in walk.iter().enumerate() {
        s.n(2, i as f32, 0.85, p - 12.0);
    }
    // Swung ride: ding on the beat, skip note at the 2/3 point of 2 and 4.
    for k in 0..16 {
        s.n(3, k as f32, 0.05, 88.0);
        if k % 2 == 1 {
            s.n(3, k as f32 + 0.67, 0.04, 96.0);
        }
    }
    // Melody with swung eighths (pairs split long-short at 0.67).
    let mel: [(f32, f32, f32); 16] = [
        (0.0, 0.6, 76.0), (0.67, 0.3, 74.0), (1.0, 0.9, 72.0), (2.0, 0.6, 69.0), (2.67, 0.3, 72.0), (3.0, 0.9, 74.0),
        (4.0, 0.6, 71.0), (4.67, 0.3, 67.0), (5.0, 1.8, 65.0),
        (8.0, 0.6, 72.0), (8.67, 0.3, 76.0), (9.0, 0.9, 79.0), (10.0, 1.8, 81.0),
        (12.0, 0.6, 79.0), (12.67, 0.3, 76.0), (13.0, 1.8, 73.0),
    ];
    for (b, d, p) in mel {
        s.n(0, b, d, p);
    }
    // Comping stabs on 2 and 4, like a rhythm guitar.
    for k in [1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0] {
        s.n(1, k + 0.05, 0.15, 60.0);
        s.n(1, k + 0.05, 0.15, 65.0);
    }
    to_wav(&s.render())
}

/// Gameplay theme E — CLASSICAL: a minuet-ish waltz in C at 88 BPM,
/// 3/4 time (24 beats = 8 bars). Oom-pah-pah accompaniment, no drums,
/// an ornamented melody with a proper cadence.
pub fn wav_waltz() -> Vec<u8> {
    let violin = Chan { wave: Wave::Square(0.5), vol: 0.13, att: 0.025, rel: 0.14 };
    let viola = Chan { wave: Wave::Square(0.5), vol: 0.07, att: 0.03, rel: 0.12 };
    let cello = Chan { wave: Wave::Triangle, vol: 0.26, att: 0.012, rel: 0.10 };
    let mut s = Song::new(88.0, 24.0, vec![violin, viola, cello]);
    // Chords: C G Am Em F C Dm-G C. (root, third, fifth) per bar.
    let chords: [(f32, f32, f32); 8] = [
        (48.0, 52.0, 55.0), // C
        (43.0, 47.0, 50.0), // G
        (45.0, 48.0, 52.0), // Am
        (40.0, 43.0, 47.0), // Em
        (41.0, 45.0, 48.0), // F
        (48.0, 52.0, 55.0), // C
        (50.0, 53.0, 57.0), // Dm
        (43.0, 47.0, 50.0), // G — dominant, resolves on the loop
    ];
    for (bar, &(r, t3, t5)) in chords.iter().enumerate() {
        let b = bar as f32 * 3.0;
        // Oom (cello) ... pah-pah (viola double stops).
        s.n(2, b, 0.9, r - 12.0);
        for k in [1.0, 2.0] {
            s.n(1, b + k, 0.55, t3 + 12.0);
            s.n(1, b + k, 0.55, t5 + 12.0);
        }
    }
    // Melody: graceful descents, a turn ornament, and a V-I cadence.
    let mel: [(f32, f32, f32); 19] = [
        (0.0, 1.0, 76.0), (1.0, 0.5, 74.0), (1.5, 0.5, 72.0), (2.0, 1.0, 74.0),
        (3.0, 1.5, 71.0), (4.5, 0.5, 74.0), (5.0, 1.0, 79.0),
        (6.0, 1.0, 76.0), (7.0, 0.5, 72.0), (7.5, 0.5, 69.0), (8.0, 1.0, 71.0),
        (9.0, 2.0, 67.0),
        (12.0, 1.0, 77.0), (13.0, 1.0, 76.0), (14.0, 1.0, 74.0),
        (15.0, 2.0, 76.0), (17.0, 1.0, 74.0),
        // Turn ornament into the cadence, then rest on the leading tone.
        (21.0, 0.25, 74.0), (21.5, 2.0, 71.0),
    ];
    for (b, d, p) in mel {
        s.n(0, b, d, p);
    }
    // The ornament's fast notes (written out: D-E-D-C).
    s.n(0, 21.25, 0.12, 76.0);
    s.n(0, 21.37, 0.13, 72.0);
    to_wav(&s.render())
}

/// The rotating gameplay playlist: (wav bytes, seconds), in rotation order.
/// Chip-pop, club, funk, lindy hop, classical — wildly different on purpose.
pub fn game_playlist() -> Vec<(Vec<u8>, f64)> {
    vec![
        (wav_game(), 16.0 * 60.0 / 104.0),
        (wav_club(), 16.0 * 60.0 / 126.0),
        (wav_funk(), 16.0 * 60.0 / 102.0),
        (wav_swing(), 16.0 * 60.0 / 152.0),
        (wav_waltz(), 24.0 * 60.0 / 88.0),
    ]
}

/// Victory fanfare: bright and brassy at 132 BPM, 8 beats.
pub fn wav_win() -> Vec<u8> {
    let mut s = Song::new(132.0, 8.0, vec![lead(), harmony(), bass(), hats()]);
    // Fanfare: C E G C', hold; then D G B C', hold higher.
    let hits: [(f32, f32, f32); 8] = [
        (0.0, 0.3, 72.0), (0.33, 0.3, 76.0), (0.66, 0.3, 79.0), (1.0, 1.6, 84.0),
        (4.0, 0.3, 74.0), (4.33, 0.3, 79.0), (4.66, 0.3, 83.0), (5.0, 2.4, 84.0),
    ];
    for (b, d, p) in hits {
        s.n(0, b, d, p);
        s.n(1, b + 0.02, d, p - 5.0); // harmony a fourth below, slightly delayed
    }
    // Bass marches on the beat: C C F G | C C G C.
    let bl: [f32; 8] = [48.0, 48.0, 53.0, 55.0, 48.0, 48.0, 43.0, 48.0];
    for (i, &p) in bl.iter().enumerate() {
        s.n(2, i as f32, 0.8, p - 12.0);
    }
    // Snare-ish roll into each fanfare.
    for k in 0..6 {
        s.n(3, 3.4 + k as f32 * 0.1, 0.05, 60.0 + k as f32 * 6.0);
    }
    s.n(3, 0.0, 0.3, 40.0);
    s.n(3, 4.0, 0.3, 40.0);
    to_wav(&s.render())
}

/// Corner-block bang: a noise burst over a falling sine thump.
pub fn wav_bang() -> Vec<u8> {
    let len = (0.38 * SR as f32) as usize;
    let mut buf = vec![0f32; len];
    let mut noise: u32 = 0xbeef_cafe;
    for (i, out) in buf.iter_mut().enumerate() {
        let t = i as f32 / SR as f32;
        noise ^= noise << 13;
        noise ^= noise >> 17;
        noise ^= noise << 5;
        let n = (noise & 0xffff) as f32 / 32768.0 - 1.0;
        // Sweeping thump: 160 Hz falling to ~40 Hz.
        let hz = 160.0 * (-t * 6.0).exp() + 40.0;
        let thump = (std::f32::consts::TAU * hz * t).sin();
        *out = n * 0.55 * (-t * 16.0).exp() + thump * 0.8 * (-t * 9.0).exp();
    }
    to_wav(&buf)
}

pub struct Sounds {
    pub menu: Sound,
    /// Rotating gameplay playlist: (sound, duration in seconds).
    pub game: Vec<(Sound, f64)>,
    pub win: Sound,
    pub bang: Sound,
}

/// Synthesize and decode all tracks. Call once at startup.
pub async fn load() -> Sounds {
    let mut game = Vec::new();
    for (bytes, dur) in game_playlist() {
        game.push((load_sound_from_bytes(&bytes).await.expect("game track"), dur));
    }
    Sounds {
        menu: load_sound_from_bytes(&wav_menu()).await.expect("menu track"),
        game,
        win: load_sound_from_bytes(&wav_win()).await.expect("win track"),
        bang: load_sound_from_bytes(&wav_bang()).await.expect("bang sfx"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_wav(bytes: &[u8], min_secs: f32) {
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..16], b"WAVEfmt ");
        let data_len = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
        assert_eq!(bytes.len(), 44 + data_len as usize);
        assert!(data_len as f32 / 2.0 / SR as f32 >= min_secs);
        // Not silent, and 16-bit samples are bounded by construction.
        let mut peak = 0i16;
        for ch in bytes[44..].chunks_exact(2) {
            let v = i16::from_le_bytes([ch[0], ch[1]]);
            peak = peak.max(v.saturating_abs());
        }
        assert!(peak > 2000, "track is nearly silent (peak {peak})");
    }

    #[test]
    fn tracks_are_valid_wav() {
        check_wav(&wav_menu(), 8.0);
        check_wav(&wav_win(), 3.0);
        check_wav(&wav_bang(), 0.3);
    }

    #[test]
    fn game_playlist_tracks_match_declared_durations() {
        let playlist = game_playlist();
        assert_eq!(playlist.len(), 5);
        for (bytes, secs) in playlist {
            check_wav(&bytes, 6.0);
            let data_len = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
            let actual = data_len as f64 / 2.0 / SR as f64;
            assert!((actual - secs).abs() < 0.02, "declared {secs:.2}s, wav is {actual:.2}s");
        }
    }
}
