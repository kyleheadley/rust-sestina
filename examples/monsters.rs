extern crate rand;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};

// can probably find a way to autogen these with macros
enum Monster<'a> {
	Weak{mtype: &'a str, damage: u16},
	Strong{mtype: &'a str, damage: u16},
}
struct MonsterGen<'a> {
	weak: MonsterGenInner<'a>,
	strong: MonsterGenInner<'a>,
}
struct MonsterGenInner<'a> {
	mtype: Vec<&'a str>,
	damage: Uniform<u16>,	
}
impl<'a> Distribution<Monster<'a>> for MonsterGen<'a> {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Monster<'a> {
		match rng.gen_range(0, 2) {
			0 => Monster::Weak{
				mtype: self.weak.mtype.choose(rng).unwrap(),
				damage: self.weak.damage.sample(rng),
			},
			1 => Monster::Strong{
				mtype: self.strong.mtype.choose(rng).unwrap(),
				damage: self.strong.damage.sample(rng),
			},
			_ => unreachable!(),
		}
	}
}

fn main() {
	let mut rng = thread_rng();
	
	let monstertype = vec!["Direwolf","Dragon","Vampire"];

	let strong_range = Uniform::from(5..=9);
	let weak_range = Uniform::from(1..=4);

	let monster = MonsterGen{
		weak: MonsterGenInner{mtype: monstertype.clone(), damage: weak_range},
		strong: MonsterGenInner{mtype: monstertype.clone(), damage: strong_range},
	};

	let the_attacker = monster.sample(&mut rng);
	
	let (t, _) = match the_attacker {
		Monster::Weak{mtype,damage} => (mtype,damage),
		Monster::Strong{mtype,damage} => (mtype,damage),
	};

	let damagetype = match t {
		"Direwolf" => "frost",
		"Dragon" => "fire",
		"Vampire" => "blood",
		_ => "unknown",
	};

	let attacktype = match the_attacker {
		Monster::Weak{mtype,damage} => {
			match (mtype,damage) {
				(_,1) => "glare",
				(_,2) => "bite",
				("Direwolf",3) => "tail",
				("Dragon",3) => "tail",
				("Vampire",3) => "cape",
				(_,4) => "claws",
				_ => "unknown attack"
			}
		},
		Monster::Strong{mtype,damage} => {
			match (mtype,damage) {
				("Vampire",5..=7) => "seduction",
				(_,5..=7) => "breath",
				(_,8..=9) => "spell",
				_ => "unknown"
			}
		}
	};

	println!("The {} caused {} damage from {} attack", t, damagetype, attacktype);
}
