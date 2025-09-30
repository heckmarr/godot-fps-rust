use godot::prelude::*;

use godot::classes::Node3D;
use godot::classes::INode3D;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Dorper {
	base: Base<Node3D>
}
#[godot_api]
impl INode3D for Dorper {

	fn init(base: Base<Node3D>) -> Self {
		Self {
			base
		}
	}

	fn ready(&mut self) {
		self.signals().dorp().connect_self(Dorper::dorping);
		self.signals().dorp().emit();
	}
}

#[godot_api]
impl Dorper {

	#[signal]
	fn dorp();

	fn dorping(&mut self) {
		godot_print!("Dorp dorp dorp");
	}
}
