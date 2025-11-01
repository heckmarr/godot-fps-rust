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

struct Processor {
	callback: Box<dyn FnMut()>,
}
impl Processor{
	fn new() -> Self {
		Self {
			callback: Box::new(Manager::manager_callback)
		}
	}
	fn set_callback(&mut self, c: impl FnMut() + 'static ) {
		self.callback = Box::new(c);
	}
	fn process_events(&mut self) {
		(self.callback)();
	}
}



#[godot_api]
impl IAnimationPlayer for Manager {
	fn init(base: Base<AnimationPlayer>) -> Self {
		let mut anim_speed = HashMap::new();

		anim_speed.insert("Idle_unarmed".to_string(), 1.0);

		anim_speed.insert("Equip_pistol".to_string(), 1.4);
		anim_speed.insert("Fire_pistol".to_string(), 1.8);
		anim_speed.insert("Idle_pistol".to_string(), 1.0);
		anim_speed.insert("Reload_pistol".to_string(), 1.0);
		anim_speed.insert("Unequip_pistol".to_string(), 1.4);

		anim_speed.insert("Equip_rifle".to_string(), 2.0);
		anim_speed.insert("Fire_rifle".to_string(), 6.0);
		anim_speed.insert("Idle_rifle".to_string(), 1.0);
		anim_speed.insert("Reload_rifle".to_string(), 1.45);
		anim_speed.insert("Unequip_rifle".to_string(), 2.0);

		anim_speed.insert("Equip_sword".to_string(), 1.0);
		anim_speed.insert("Fire_sword".to_string(), 1.35);
		anim_speed.insert("Idle_knife".to_string(), 1.0);
		anim_speed.insert("Unequip_sword".to_string(), 1.0);
		let mut state = HashMap::new();
		state.insert("Idle_unarmed".to_string(), vec!["Equip_sword".to_string(), "Equip_pistol".to_string(), "Equip_rifle".to_string(), "Idle_unarmed".to_string()]);

		state.insert("Equip_pistol".to_string(), vec!["Idle_pistol".to_string()]);
		state.insert("Fire_pistol".to_string(), vec!["Idle_pistol".to_string()]);
		state.insert("Idle_pistol".to_string(), vec!["Fire_pistol".to_string(), "Reload_pistol".to_string(), "Unequip_pistol".to_string(), "Idle_pistol".to_string()]);
		state.insert("Reload_pistol".to_string(), vec!["Idle_pistol".to_string()]);
		state.insert("Unequip_pistol".to_string(), vec!["Idle_unarmed".to_string()]);

		state.insert("Equip_rifle".to_string(), vec!["Idle_rifle".to_string()]);
		state.insert("Fire_rifle".to_string(), vec!["Idle_rifle".to_string()]);
		state.insert("Idle_rifle".to_string(), vec!["Fire_rifle".to_string(), "Reload_rifle".to_string(), "Unequip_rifle".to_string(), "Idle_rifle".to_string()]);
		state.insert("Reload_rifle".to_string(), vec!["Idle_rifle".to_string()]);
		state.insert("Unequip_rifle".to_string(), vec!["Idle_unarmed".to_string()]);
		state.insert("Equip_sword".to_string(), vec!["Idle_knife".to_string()]);
		state.insert("Fire_sword".to_string(), vec!["Idle_knife".to_string()]);
		state.insert("Idle_knife".to_string(), vec!["Fire_sword".to_string(), "Unequip_sword".to_string(), "Idle_knife".to_string()]);
		state.insert("Unequip_sword".to_string(), vec!["Idle_unarmed".to_string()]);

		Self {
			current_state: "".to_string(),
			animation_speeds: anim_speed,
			states: state,
			callback_processor: Processor::new(),
			base
		}
	}

	fn ready(&mut self) {
//		self.callback_processor.set_callback(Manager::manager_callback);
		self.callback_processor.process_events();
		self.set_animation("idle_unarmed".to_string());
		self.signals().animation_finished().connect_self(Manager::animation_ended);
	}
}

#[godot_api]
impl Manager {
	fn manager_callback() {
		godot_print!("I'd like to speak to your task manager.");
	}
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
		else if self.current_state == "Equip_sword".to_string() {
			self.set_animation("Idle_knife".to_string());
		}else if self.current_state == "Idle_knife".to_string() {
			//pass
		}else if self.current_state == "Fire_sword".to_string() {
			self.set_animation("Idle_knife".to_string());
		}else if self.current_state == "Unequip_sword".to_string() {
			self.set_animation("Idle_unarmed".to_string());
		}
		// PISTOL transitions
		else if self.current_state == "Equip_pistol".to_string() {
			self.set_animation("Idle_pistol".to_string());
		}else if self.current_state == "Idle_pistol".to_string() {
			//pass
		}else if self.current_state == "Fire_pistol".to_string() {
			self.set_animation("Idle_pistol".to_string());
		}else if self.current_state == "Unequip_pistol".to_string(){
			self.set_animation("Idle_unarmed".to_string());
		}else if self.current_state == "Reload_pistol".to_string(){
			self.set_animation("Idle_pistol".to_string());
		// RIFLE transitions
		}else if self.current_state == "Equip_rifle".to_string() {
			self.set_animation("Idle_rifle".to_string());
		}else if self.current_state == "Idle_rifle".to_string() {
			//pass
		}else if self.current_state == "Fire_rifle".to_string() {
			self.set_animation("Idle_rifle".to_string());
		}else if self.current_state == "Unequip_rifle".to_string() {
			self.set_animation("Idle_unarmed".to_string());
		}else if self.current_state == "Reload_rifle".to_string(){
			self.set_animation("Idle_rifle".to_string());
		}
	}
}

