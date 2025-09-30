use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;

use godot::classes::Camera3D;
use godot::classes::Input;
use godot::classes::InputEvent;
use godot::classes::InputEventMouseMotion;
use godot::classes::input::MouseMode;

use godot::global::deg_to_rad;
use godot::global::clamp;

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
	fn input(&mut self, event: Gd<InputEvent>) {
		let input = Input::singleton();
		let event_mouse = event.try_cast::<InputEventMouseMotion>();

		match event_mouse {
			Ok(event) => {
				let mm = input.get_mouse_mode();
				if mm == MouseMode::CAPTURED {
					let mut rot_helper: Gd<Node3D> = self.base_mut().get_node_as("/root/Testing_Area/Player/Rotation_Helper");
					let rel = event.get_relative();
					let ms = self.mouse_sensitivity;
					rot_helper.rotate_x(deg_to_rad((rel.y * ms).into()) as f32);
					self.base_mut().rotate_y(deg_to_rad((rel.x * ms * -1.0).into()) as f32);
					let mut camera_rot = rot_helper.get_rotation_degrees();
					camera_rot.x = clamp(&(camera_rot.x).to_variant(), &(-70).to_variant(), &(70).to_variant()).to::<f32>();
					rot_helper.set_rotation_degrees(camera_rot);
				}
			},
			Err(_err) => {
				//pass
			},
		}

	}


}

#[godot_api]
impl Player {

	#[signal]
	fn dorp();
	fn process_input(&mut self, _event: Gd<InputEvent>) {
		let mut event = Input::singleton();
		let mut dir = Vector3::ZERO;
		let camera: Gd<Camera3D> = self.base().get_node_as("/root/Testing_Area/Player/Rotation_Helper/Camera");
		let cam_xform = camera.get_global_transform();

		let mut input_movement_vector: Vector2 = Vector2::ZERO;
//		let mut inp = Input::singleton();

		if event.is_action_pressed("movement_forward") {
//			godot_print!("Move forward!");
			input_movement_vector.y += 1.0;
		}else if event.is_action_pressed("movement_backward") {
//			godot_print!("Move backward!");
			input_movement_vector.y -= 1.0;
		}else if event.is_action_pressed("movement_left") {
//			godot_print!("Move left!");
			input_movement_vector.x -= 1.0;
		}else if event.is_action_pressed("movement_right") {
//			godot_print!("Move right!");
			input_movement_vector.x = 1.0;
		}
		if input_movement_vector != Vector2::ZERO {
			input_movement_vector = input_movement_vector.normalized();
		}
		dir += -cam_xform.basis.col_c().normalized() * input_movement_vector.y;
		dir += cam_xform.basis.col_a().normalized() * input_movement_vector.x;

		//jump!
		if self.base().is_on_floor() {
			if event.is_action_pressed("movement_jump") {
				self.vel.y = self.jump_speed as f32;
			}
		}

		//capture/free the cursor
		if event.is_action_pressed("ui_cancel") {
			//These three calls are the only ones that need Input rather than InputEvent
			let mm = event.get_mouse_mode();
			if mm == MouseMode::VISIBLE {
				event.set_mouse_mode(MouseMode::CAPTURED);
			}else {
				event.set_mouse_mode(MouseMode::VISIBLE);
			}
		}
		self.dir = dir

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
