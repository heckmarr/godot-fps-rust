use godot::prelude::*;

use godot::classes::AnimationPlayer;
use godot::classes::IAnimationPlayer;

use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=AnimationPlayer)]
//This will be a huge object, because it's the entire state machine for the game's
//animaiton, bear with me while I build this
struct Manager {
	base: Base<AnimationPlayer>
}

#[godot_api]
impl IAnimationPlayer for Manager {
	fn init(base: Base<AnimationPlayer>) -> Self {
		Self {
			base
		}
	}

	fn ready(&mut self) {
		let dorp = vec!["dorpen", "dorpeth", "dorp"];
		let mut dorp_map = HashMap::new();
		dorp_map.insert("first", dorp[0]);
		dorp_map.insert("tuesday", dorp[1]);
		dorp_map.insert("three", dorp[2]);
		Manager::dorp(dorp);
		Manager::find_dorp(dorp_map);
	}
}

impl Manager {
	fn find_dorp(to_dorp: HashMap<&str, &str>) {
		let dorped = vec!("first", "tuesday", "missingno", "three");
		let mut dor = Vec::new();
		for &thing in &dorped {
			match to_dorp.get(thing) {
				Some(speak) => dor.push(speak),
				None => godot_print!("{thing} is no thing to dorp!")
			}
			for d in &dor {
				godot_print!("{d}!");
			}
		}
	}

	fn dorp(to_dorp: Vec<&str>) {
		let d = &to_dorp[0];
		let dd = &to_dorp[1];
		let ddd = &to_dorp[2];

		godot_print!("{d}, {dd}, {ddd}");
	}
}
