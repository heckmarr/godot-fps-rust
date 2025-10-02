use godot::prelude::*;

use godot::classes::AnimationPlayer;
use godot::classes::IAnimationPlayer;

#[derive(GodotClass)]
#[class(base=AnimationPlayer)]
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
		let dorp = vec!("dorpen", "dorpeth", "dorp");
		Manager::dorp(dorp);
	}
}

impl Manager {
	fn dorp(to_dorp: Vec<&str>) {
		let d = &to_dorp[0];
		let dd = &to_dorp[1];
		let ddd = &to_dorp[2];

		godot_print!("{d}, {dd}, {ddd}");
	}
}
