//! The meme database: family-friendly victory one-liners for the game-over
//! screen. `{winner}` is replaced with the winning color's name by the UI.

pub const MEMES: &[&str] = &[
    "One does not simply place the X pentomino.",
    "Outstanding move, {winner}. Outstanding move.",
    "{winner} didn't win. Everyone else just lost simultaneously.",
    "It's not about the squares you place, it's about the friends you block along the way.",
    "Corners? Where {winner} is going, they need ALL the corners.",
    "Me: just one casual game. Also me: spreadsheet of every diagonal on the board.",
    "Blocked friends speedrun: any%. New record.",
    "{winner} is playing 4D Blokus. Everyone else brought checkers.",
    "Nobody: ... Absolutely nobody: ... {winner}: takes the last corner anyway.",
    "That wasn't a move. That was a hostile takeover with extra steps.",
    "POV: you saved the monomino for the very end like an absolute legend.",
    "Tell me you hoard pentominoes without telling me you hoard pentominoes.",
    "Instructions unclear. Blocked my own corner.",
    "This is fine. — everyone with 40 squares still in hand",
    "Winning is 10% skill and 90% pretending it was all part of the plan.",
    "{winner} has entered the corner office.",
    "Some people want world peace. {winner} wanted your corners.",
    "Breaking news: local player declares diagonal supremacy, refuses to elaborate.",
    "You either retire with zero pieces or play long enough to get walled in.",
    "Friendship ended with open corners. Blocking is my best friend now.",
    "Not all heroes wear capes. Some just place the W pentomino perfectly.",
    "Keep calm and cover the center.",
    "First rule of Blokus club: never leave {winner} an open diagonal.",
    "I came, I saw, I cornered.",
    "Achievement unlocked: certified space negotiator.",
    "My therapist: the L pentomino can't hurt you. The L pentomino:",
    "Big brain time: sacrificing one corner to steal three.",
    "They asked for a fair game. {winner} heard 'square game'.",
    "Error 404: opponent corners not found.",
    "Legend says the last monomino is still waiting to be placed.",
];

/// Deterministic pick from a random value (uniform over the database).
pub fn pick(rng_value: u64) -> &'static str {
    MEMES[(rng_value % MEMES.len() as u64) as usize]
}
