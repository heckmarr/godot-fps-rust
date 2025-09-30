use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
	gravity: f64,
	vel: Vector3,
	max_speed: i64,
	jump_speed: i64,
	accel: f64,
	dir: Vector3,
	deaccel: i64,
	max_slope_angle: i64,
	mouse_sensitivity: f64,
	base: Base<CharacterBody3D>
}
#[godot_api]
impl ICharacterBody3D for Player {

	fn init(base: Base<CharacterBody3D>) -> Self {
		Self {
			gravity: -24.8,
			vel: Vector3::new(0.0, 0.0, 0.0),
			max_speed: 20,
			jump_speed: 18,
			accel: 4.5,
			dir: Vector3::new(0.0, 0.0, 0.0),
			deaccel: 16,
			max_slope_angle: 40,
			mouse_sensitivity: 0.05,
			base
		}
	}

	fn ready(&mut self) {
		self.signals().dorp().connect_self(Player::dorping);
		self.signals().dorp().emit();
	}
}

#[godot_api]
impl Player {

	#[signal]
	fn dorp();

	fn dorping(&mut self) {
		godot_print!("Dorp dorp dorp");
	}
}
