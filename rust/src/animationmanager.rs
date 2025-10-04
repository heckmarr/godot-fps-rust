use godot::prelude::*;

use godot::classes::AnimationPlayer;
use godot::classes::IAnimationPlayer;

use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=AnimationPlayer)]
struct Manager {
    idle_unarmed:Vec<String>,

    pistol_equip:Vec<String>,
    pistol_fire:Vec<String>,
    pistol_idle:Vec<String>,
    pistol_reload:Vec<String>,
    pistol_unequip:Vec<String>,

    rifle_equip:Vec<String>,
    rifle_fire:Vec<String>,
    rifle_idle:Vec<String>,
    rifle_reload:Vec<String>,
    rifle_unequip:Vec<String>,

    knife_equip: Vec<String>,
    knife_fire: Vec<String>,
    knife_idle: Vec<String>,
    knife_unequip:Vec<String>,

	animation_speeds: HashMap<String, f32>,
	base: Base<AnimationPlayer>
}

#[godot_api]
impl IAnimationPlayer for Manager {
	fn init(base: Base<AnimationPlayer>) -> Self {
		let mut anim_speed = HashMap::new();

		anim_speed.insert("idle_unarmed".to_string(), 1.0);

		anim_speed.insert("pistol_equip".to_string(), 1.4);
		anim_speed.insert("pistol_fire".to_string(), 1.8);
		anim_speed.insert("pistol_idle".to_string(), 1.0);
		anim_speed.insert("pistol_reload".to_string(), 1.0);
		anim_speed.insert("pistol_unequip".to_string(), 1.4);

		anim_speed.insert("rifle_equip".to_string(), 2.0);
		anim_speed.insert("rifle_fire".to_string(), 6.0);
		anim_speed.insert("rifle_idle".to_string(), 1.0);
		anim_speed.insert("rifle_reload".to_string(), 1.45);
		anim_speed.insert("rifle_unequip".to_string(), 2.0);

		anim_speed.insert("knife_equip".to_string(), 1.0);
		anim_speed.insert("knife_fire".to_string(), 1.35);
		anim_speed.insert("knife_idle".to_string(), 1.0);
		anim_speed.insert("knife_unequip".to_string(), 1.0);
		Self {
			animation_speeds: anim_speed,
			idle_unarmed: vec!["knife_equip".to_string(), "pistol_equip".to_string(), "rifle_equip".to_string(), "idle_unarmed".to_string()],

			pistol_equip:vec!["pistol_idle".to_string()],
			pistol_fire:vec!["pistol_idle".to_string()],
			pistol_idle:vec!["pistol_fire".to_string(), "pistol_reload".to_string(), "pistol_unequip".to_string(), "pistol_idle".to_string()],
			pistol_reload:vec!["pistol_idle".to_string()],
			pistol_unequip:vec!["idle_unarmed".to_string()],

			rifle_equip:vec!["rifle_idle".to_string()],
			rifle_fire:vec!["rifle_idle".to_string()],
			rifle_idle:vec!["rifle_fire".to_string(), "rifle_reload".to_string(), "rifle_unequip".to_string(), "rifle_idle".to_string()],
			rifle_reload:vec!["rifle_idle".to_string()],
			rifle_unequip:vec!["idle_unarmed".to_string()],

			knife_equip:vec!["knife_idle".to_string()],
			knife_fire:vec!["knife_idle".to_string()],
			knife_idle:vec!["knife_fire".to_string(), "knife_unequip".to_string(), "knife_idle".to_string()],
			knife_unequip:vec!["idle_unarmed".to_string()],




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
