use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;

use godot::classes::Camera3D;
use godot::classes::Input;
use godot::classes::input::MouseMode;

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
		let mut inp = Input::singleton();
		inp.set_mouse_mode(MouseMode::CAPTURED);
//don't forget to dorp
		self.signals().dorp().connect_self(Player::dorping);
		self.signals().dorp().emit();
	}
	fn physics_process(&mut self, delta: f64) {
		self.process_input(delta);
		self.process_movement(delta);
	}


}

#[godot_api]
impl Player {

	#[signal]
	fn dorp();
	fn process_input(&mut self, delta: f64) {
		self.dir = Vector3::new(0.0, 0.0, 0.0);
		let camera: Gd<Camera3D> = self.base().get_node_as("/root/Testing_Area/Player/Rotation_Helper/Camera");
		let cam_xform = camera.get_global_transform();

		let mut input_movement_vector: Vector2 = Vector2::new( 0.0, 0.0);
		let mut inp = Input::singleton();

		if inp.is_action_pressed("movement_forward") {
			input_movement_vector.y += 1.0;
		}else if inp.is_action_pressed("movement_backward") {
			input_movement_vector.y -= 1.0;
		}else if inp.is_action_pressed("movement_left") {
			input_movement_vector.x -= 1.0;
		}else if inp.is_action_pressed("movement_right") {
			input_movement_vector.x = 1.0;
		}

		input_movement_vector = input_movement_vector.normalized();

		self.dir += -cam_xform.basis.col_c().normalized() * input_movement_vector.y;
		self.dir += cam_xform.basis.col_a().normalized() * input_movement_vector.x;

		//jump!
		if self.base().is_on_floor() {
			if inp.is_action_just_pressed("movement_jump") {
				self.vel.y = self.jump_speed as f32;
			}
		}

		//capture/free the cursor
		if inp.is_action_just_pressed("ui_cancel") {
			let mm = inp.get_mouse_mode();
			if mm == MouseMode::VISIBLE {
				inp.set_mouse_mode(MouseMode::CAPTURED);
			}else {
				inp.set_mouse_mode(MouseMode::VISIBLE);
			}
		}

	}
	fn process_movement(&mut self, delta: f64) {
		godot_print!("It's moving!");
	}

	fn dorping(&mut self) {
		godot_print!("Dorp dorp dorp");
	}
}
