use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;

use godot::classes::Camera3D;
use godot::classes::Input;
use godot::classes::InputEvent;
use godot::classes::input::MouseMode;

use godot::global::deg_to_rad;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
	gravity: f32,
	vel: Vector3,
	max_speed: i64,
	jump_speed: i64,
	accel: f32,
	dir: Vector3,
	deaccel: i64,
	max_slope_angle: i64,
	mouse_sensitivity: f32,
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
	fn physics_process(&mut self, delta: f32) {
		self.process_movement(delta);
	}
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		self.process_input(event);
	}


}

#[godot_api]
impl Player {

	#[signal]
	fn dorp();
	fn process_input(&mut self, event: Gd<InputEvent>) {
		self.dir = Vector3::ZERO;
		let camera: Gd<Camera3D> = self.base().get_node_as("/root/Testing_Area/Player/Rotation_Helper/Camera");
		let cam_xform = camera.get_global_transform();

		let mut input_movement_vector: Vector2 = Vector2::ZERO;
//		let mut inp = Input::singleton();

		if event.is_action_pressed("movement_forward") {
			godot_print!("Move forward!");
			input_movement_vector.y += 1.0;
		}else if event.is_action_pressed("movement_backward") {
			godot_print!("Move backward!");
			input_movement_vector.y -= 1.0;
		}else if event.is_action_pressed("movement_left") {
			godot_print!("Move left!");
			input_movement_vector.x -= 1.0;
		}else if event.is_action_pressed("movement_right") {
			godot_print!("Move right!");
			input_movement_vector.x = 1.0;
		}
		if input_movement_vector != Vector2::ZERO {
			input_movement_vector = input_movement_vector.normalized();
		}
		self.dir += -cam_xform.basis.col_c().normalized() * input_movement_vector.y;
		self.dir += cam_xform.basis.col_a().normalized() * input_movement_vector.x;

		//jump!
		if self.base().is_on_floor() {
			if event.is_action_pressed("movement_jump") {
				self.vel.y = self.jump_speed as f32;
			}
		}

		//capture/free the cursor
		let mut input = Input::singleton();
		if event.is_action_pressed("ui_cancel") {
			let mm = input.get_mouse_mode();
			if mm == MouseMode::VISIBLE {
				input.set_mouse_mode(MouseMode::CAPTURED);
			}else {
				input.set_mouse_mode(MouseMode::VISIBLE);
			}
		}

	}
	fn process_movement(&mut self, delta: f32) {
		self.dir.y = 0.0;
		if self.dir != Vector3::ZERO {
			self.dir = self.dir.normalized();
		}
		self.vel.y += delta*self.gravity;

		let mut hvel = self.vel;
		hvel.y = 0.0;

		let mut target = self.dir;
		target *= self.max_speed as f32;

		let mut accel = 0.0;
		if self.dir.dot(hvel) > 0.0 {
			accel = self.accel as f32;
		}else {
			accel = self.deaccel as f32;
		}
		hvel = hvel.lerp(target, accel*delta);
		self.vel.x = hvel.x;
		self.vel.z = hvel.z;
		let target_vel = self.vel;
		self.base_mut().set_velocity(target_vel);
		self.base_mut().move_and_slide();
//		self.vel = self.base_mut().move_and_slide(self.vel, Vector3::new(0.0,1.0,0.0), 0.05, 4.0, deg_to_rad(self.max_slope_angle));

	}

	fn dorping(&mut self) {
		godot_print!("Dorp dorp dorp");
	}
}
