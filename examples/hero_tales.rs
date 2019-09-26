extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

struct PronounSet<'a> {
  they: &'a str,
  them: &'a str,
  their: &'a str,
}

fn main() {
	let mut rng = thread_rng();

  let they_pronouns = PronounSet{they: "they",them: "them",their: "their"};
  let she_pronouns = PronounSet{they: "she",them: "her",their: "her"};
  let he_pronouns = PronounSet{they: "he",them: "him",their: "his"};

	let pronouns = vec![they_pronouns, she_pronouns, he_pronouns];
	let name = vec!["Robin", "Ash", "Casey", "Kris", "Alex", "Riley", "Taylor"];

	let hero_name = name.choose(&mut rng).unwrap();
	let pronouns = pronouns.choose(&mut rng).unwrap();

	let story = format!{
		"Our hero {heroName} went into the dungeon to find treasure. \
		{they} descended into the final cave, drew \
		{their} sword, and fought the beast who faced \
		{them}.",
		heroName = hero_name, they = pronouns.they,
		their = pronouns.their, them = pronouns.them,
	};

	println!("{}", story);
}
