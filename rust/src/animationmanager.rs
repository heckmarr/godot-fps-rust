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
	base: Base<AnimationPlayer>
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


			base
		}
	}

	fn ready(&mut self) {
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
			}
		}
		return false;
	}
	
	fn animation_ended(&mut self, animation_name: StringName) {
		// UNARMED transitions
		if self.current_state == "idle_unarmed" {
			//pass
		}
		// KNIFE transitions
		else if self.current_state == "knife_equip" {
			self.set_animation("knife_idle".to_string());
		}else if self.current_state == "knife_idle" {
			//pass
		}else if self.current_state == "knife_fire" {
			self.set_animation("knife_idle".to_string());
		}else if self.current_state == "knife_unequip" {
			self.set_animation("idle_unarmed".to_string());
		}
		// PISTOL transitions
		else if self.current_state == "pistol_equip" {
			self.set_animation("pistol_idle".to_string());
		}else if self.current_state == "pistol_idle" {
			//pass
		}else if self.current_state == "pistol_fire" {
			self.set_animation("pistol_idle".to_string());
		}else if self.current_state == "pistol_unequip"{
			self.set_animation("idle_unarmed".to_string());
		}else if self.current_state == "pistol_reload"{
			self.set_animation("pistol_idle".to_string());
		// RIFLE transitions
		}else if self.current_state == "rifle_equip" {
			self.set_animation("rifle_idle".to_string());
		}else if self.current_state == "rifle_idle" {
			//pass
		}else if self.current_state == "rifle_fire" {
			self.set_animation("rifle_idle".to_string());
		}else if self.current_state == "rifle_unequip" {
			self.set_animation("idle_unarmed".to_string());
		}else if self.current_state == "rifle_reload"{
			self.set_animation("rifle_idle".to_string());
		}
	}
}

