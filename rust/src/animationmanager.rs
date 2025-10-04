use godot::prelude::*;

use godot::classes::AnimationPlayer;
use godot::classes::IAnimationPlayer;

use godot::meta::AsArg;

use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=AnimationPlayer)]
struct Manager {
	states: HashMap<String, Vec<String>>,
	current_state: String,
	animation_speeds: HashMap<String, f32>,
	callback_processor: Processor,
	base: Base<AnimationPlayer>
}

type Callback = fn();

struct Processor {
	callback: Callback,
}
impl Processor {
	fn new() -> Self {
		Self{
			callback: animation_callback
		}
	}
	fn set_callback(&mut self, c: Callback) {
		self.callback = c;
	}
	fn process_events(&mut self) {
		(self.callback)();
	}
}

fn animation_callback() {
	godot_print!("callback fired!");
}


#[godot_api]
impl IAnimationPlayer for Manager {
	fn init(base: Base<AnimationPlayer>) -> Self {
		let mut anim_speed = HashMap::new();

		anim_speed.insert("Idle_unarmed".to_string(), 1.0);

		anim_speed.insert("Pistol_equip".to_string(), 1.4);
		anim_speed.insert("Pistol_fire".to_string(), 1.8);
		anim_speed.insert("Pistol_idle".to_string(), 1.0);
		anim_speed.insert("Pistol_reload".to_string(), 1.0);
		anim_speed.insert("Pistol_unequip".to_string(), 1.4);

		anim_speed.insert("Rifle_equip".to_string(), 2.0);
		anim_speed.insert("Rifle_fire".to_string(), 6.0);
		anim_speed.insert("Rifle_idle".to_string(), 1.0);
		anim_speed.insert("Rifle_reload".to_string(), 1.45);
		anim_speed.insert("Rifle_unequip".to_string(), 2.0);

		anim_speed.insert("Knife_equip".to_string(), 1.0);
		anim_speed.insert("Knife_fire".to_string(), 1.35);
		anim_speed.insert("Knife_idle".to_string(), 1.0);
		anim_speed.insert("Knife_unequip".to_string(), 1.0);
		let mut state = HashMap::new();
		state.insert("Idle_unarmed".to_string(), vec!["Knife_equip".to_string(), "Pistol_equip".to_string(), "Rifle_equip".to_string(), "Idle_unarmed".to_string()]);

		state.insert("Pistol_equip".to_string(), vec!["Pistol_idle".to_string()]);
		state.insert("Pistol_fire".to_string(), vec!["Pistol_idle".to_string()]);
		state.insert("Pistol_idle".to_string(), vec!["Pistol_fire".to_string(), "Pistol_reload".to_string(), "Pistol_unequip".to_string(), "Pistol_idle".to_string()]);
		state.insert("Pistol_reload".to_string(), vec!["Pistol_idle".to_string()]);
		state.insert("Pistol_unequip".to_string(), vec!["Idle_unarmed".to_string()]);

		state.insert("Rifle_equip".to_string(), vec!["Rifle_idle".to_string()]);
		state.insert("Rifle_fire".to_string(), vec!["Rifle_idle".to_string()]);
		state.insert("Rifle_idle".to_string(), vec!["Rifle_fire".to_string(), "Rifle_reload".to_string(), "Rifle_unequip".to_string(), "Rifle_idle".to_string()]);
		state.insert("Rifle_reload".to_string(), vec!["Rifle_idle".to_string()]);
		state.insert("Rifle_unequip".to_string(), vec!["Idle_unarmed".to_string()]);
		state.insert("Knife_equip".to_string(), vec!["Knife_idle".to_string()]);
		state.insert("Knife_fire".to_string(), vec!["Knife_idle".to_string()]);
		state.insert("Knife_idle".to_string(), vec!["Knife_fire".to_string(), "Knife_unequip".to_string(), "Knife_idle".to_string()]);
		state.insert("Knife_unequip".to_string(), vec!["Idle_unarmed".to_string()]);

		Self {
			current_state: "".to_string(),
			animation_speeds: anim_speed,
			states: state,
			callback_processor: Processor::new(),

			base
		}
	}

	fn ready(&mut self) {
		self.callback_processor.process_events();
		self.set_animation("idle_unarmed".to_string());
		self.signals().animation_finished().connect_self(Manager::animation_ended);
	}
}

#[godot_api]
impl Manager {
	#[signal]
	fn dorp();
	fn set_animation(&mut self, animation_name: String) -> bool {
		
		

		if animation_name == self.current_state {
			godot_print!("AnimationPlayer_manager WARNING: animation is already {animation_name}");
			return true;
		}

		if self.base().has_animation(&animation_name) == true {
			if self.current_state != "" {
				let possible_animations = &self.states[&self.current_state];
				let mut animation_in = "".to_string();
				for state in possible_animations.iter() {
					if animation_name == state.to_string() {
						animation_in = state.to_string();
					}
				}
				if animation_in != "".to_string() {
					self.current_state = animation_name.clone();
					let speed = self.animation_speeds[&animation_name];
					self.base_mut().play_ex().name(&animation_name).custom_speed(speed).custom_blend(-1.0).done();
					return true;
				}else {
					godot_print!("AnimationPlayer manager: WARNING: Cannot change to {animation_name}");
					return false;
				}
					
			}else {
				self.current_state = animation_name.clone();
				let speed = self.animation_speeds[&animation_name];
				self.base_mut().play_ex().name(&animation_name).custom_speed(speed).custom_blend(-1.0).done();
				return true;
			}
		}
		return false;
	}
	
	fn animation_ended(&mut self, animation_name: StringName) {
		// UNARMED transitions
		if self.current_state == "Idle_unarmed".to_string() {
			//pass
		}
		// KNIFE transitions
		else if self.current_state == "Knife_equip".to_string() {
			self.set_animation("Knife_idle".to_string());
		}else if self.current_state == "Knife_idle".to_string() {
			//pass
		}else if self.current_state == "Knife_fire".to_string() {
			self.set_animation("Knife_idle".to_string());
		}else if self.current_state == "Knife_unequip".to_string() {
			self.set_animation("Idle_unarmed".to_string());
		}
		// PISTOL transitions
		else if self.current_state == "Pistol_equip".to_string() {
			self.set_animation("Pistol_idle".to_string());
		}else if self.current_state == "Pistol_idle".to_string() {
			//pass
		}else if self.current_state == "Pistol_fire".to_string() {
			self.set_animation("Pistol_idle".to_string());
		}else if self.current_state == "Pistol_unequip".to_string(){
			self.set_animation("Idle_unarmed".to_string());
		}else if self.current_state == "Pistol_reload".to_string(){
			self.set_animation("Pistol_idle".to_string());
		// RIFLE transitions
		}else if self.current_state == "Rifle_equip".to_string() {
			self.set_animation("Rifle_idle".to_string());
		}else if self.current_state == "Rifle_idle".to_string() {
			//pass
		}else if self.current_state == "Rifle_fire".to_string() {
			self.set_animation("Rifle_idle".to_string());
		}else if self.current_state == "Rifle_unequip".to_string() {
			self.set_animation("Idle_unarmed".to_string());
		}else if self.current_state == "Rifle_reload".to_string(){
			self.set_animation("Rifle_idle".to_string());
		}
	}
}

