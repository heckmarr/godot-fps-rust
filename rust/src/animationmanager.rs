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
		self.dorp();
	}
}

impl Manager {
	fn dorp(&mut self) {
		godot_print!("Dorpen dorp dorp");
	}
}
