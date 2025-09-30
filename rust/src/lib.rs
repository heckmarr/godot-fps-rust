use godot::prelude::*;

struct Thinger;

#[gdextension]
unsafe impl ExtensionLibrary for Thinger {}

mod dorp;
mod player;
