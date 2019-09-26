extern crate rand;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};

enum Suit {
	Pentacles,
	Cups,
	Swords,
	Wands,
}
// gentype numeric_ranks = range(2,10)

// gentype rank = OR {"ace", tmap toString numeric_ranks, "page", "knight",
// "queen", "king"}

// gentype minor = AND {Suit: suit, Rank: rank}

// gentype major = OR {"fool", "magician", "strength", "world", "chariot", "high
// priestess", "tower", "sun", "star", "moon", "justice", "hermit"} (* etc *)



fn main() {
}
