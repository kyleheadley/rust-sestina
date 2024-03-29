extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

// This could be handled by macro
trait GenType where Self: Sized + Copy {
	fn first() -> Self;
	fn next(&self) -> Option<Self>;
}

#[derive(Debug,Clone,Copy)]
enum Suit {
	Pentacles,
	Cups,
	Swords,
	Wands,
}
impl GenType for Suit {
	fn first() -> Self {Suit::Pentacles}
	fn next(&self) -> Option<Self> {
		match self {
			Suit::Pentacles => Some(Suit::Cups),
			Suit::Cups => Some(Suit::Swords),
			Suit::Swords => Some(Suit::Wands),
			Suit::Wands => None,
		}
	}
}

#[derive(Debug,Clone,Copy)]
enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Page,
	Knight,
	Queen,
	King,
}

impl GenType for Rank {
	fn first() -> Self {Rank::Ace}
	fn next(&self) -> Option<Self> {
		match self {
			Rank::Ace => Some(Rank::Two),
			Rank::Two => Some(Rank::Three),
			Rank::Three => Some(Rank::Four),
			Rank::Four => Some(Rank::Five),
			Rank::Five => Some(Rank::Six),
			Rank::Six => Some(Rank::Seven),
			Rank::Seven => Some(Rank::Eight),
			Rank::Eight => Some(Rank::Nine),
			Rank::Nine => Some(Rank::Ten),
			Rank::Ten => Some(Rank::Page),
			Rank::Page => Some(Rank::Knight),
			Rank::Knight => Some(Rank::Queen),
			Rank::Queen => Some(Rank::King),
			Rank::King => None,
		}
	}
}

#[derive(Debug,Clone,Copy)]
struct Minor {
	suit: Suit,
	rank: Rank,
}

impl GenType for Minor {
	fn first() -> Self { Minor {
		suit: Suit::first(),
		rank: Rank::first(),
	}}
	fn next(&self) -> Option<Self> {
		match self { Minor{suit,rank} => {
			if let Some(s) = suit.next() {
				Some(Minor{suit:s,rank:*rank})
			} else if let Some(r) = rank.next() {
				Some(Minor{suit:<Suit as GenType>::first(),rank:r})
			} else { None }
		}}
	}
}

#[derive(Debug,Clone,Copy)]
enum Major {
	Fool,
	Magician,
	HighPriestess,
	Empress,
	Emperor,
	Hierophant,
	Lovers,
	Chariot,
	Justice,
	Hermit,
	WheelOfFortune,
	Strength,
	HangedMan,
	Death,
	Temperance,
	Devil,
	Tower,
	Star,
	Moon,
	Sun,
	Judgement,
	World,
}

impl GenType for Major {
	fn first() -> Self {Major::Fool}
	fn next(&self) -> Option<Self> {
		match self {
			Major::Fool => Some(Major::Magician),
			Major::Magician => Some(Major::HighPriestess),
			Major::HighPriestess => Some(Major::Empress),
			Major::Empress => Some(Major::Emperor),
			Major::Emperor => Some(Major::Hierophant),
			Major::Hierophant => Some(Major::Lovers),
			Major::Lovers => Some(Major::Chariot),
			Major::Chariot => Some(Major::Justice),
			Major::Justice => Some(Major::Hermit),
			Major::Hermit => Some(Major::WheelOfFortune),
			Major::WheelOfFortune => Some(Major::Strength),
			Major::Strength => Some(Major::HangedMan),
			Major::HangedMan => Some(Major::Death),
			Major::Death => Some(Major::Temperance),
			Major::Temperance => Some(Major::Devil),
			Major::Devil => Some(Major::Tower),
			Major::Tower => Some(Major::Star),
			Major::Star => Some(Major::Moon),
			Major::Moon => Some(Major::Sun),
			Major::Sun => Some(Major::Judgement),
			Major::Judgement => Some(Major::World),
			Major::World => None,
		}
	}
}

#[derive(Debug,Clone,Copy)]
enum Card {
	Major(Major),
	Minor(Minor),
}

impl GenType for Card {
	fn first() -> Self {Card::Major(Major::first())}
	fn next(&self) -> Option<Self> {
		match self {
			Card::Major(maj) => if let Some(m) = maj.next()
				{Some(Card::Major(m))} else {Some(Card::Minor(Minor::first()))},
			Card::Minor(min) => if let Some(m) = min.next()
				{Some(Card::Minor(m))} else {None}
		}
	}
}

struct GenIter<G:GenType> {
	last: Option<G>,
}
impl<G:GenType> GenIter<G> {
	fn new() -> GenIter<G> {GenIter{last:None}}
}

impl<G:GenType> Iterator for GenIter<G> {
	type Item = G;
	fn next(&mut self) -> Option<Self::Item> {
		match self.last {
			None => self.last = Some(G::first()),
			Some(g) => self.last = g.next(),
		};
		self.last
	}
}

fn main() {
	let mut rng = thread_rng();

	let mut deck: Vec<Card> = GenIter::new().collect();
	println!("random card: {:?}", deck.choose(&mut rng).unwrap());

	deck.shuffle(&mut rng);
	println!("shuffled deck");
	println!("first card: {:?}",deck.remove(0));
	println!("second card: {:?}",deck.remove(0));
	println!("third card: {:?}",deck.remove(0));
}
