use bevy::prelude::*;

use belts::events::{ModifiedEvent, RotateEvent};
use belts::setup::setup;
use belts::systems::{animate_sprites, do_rotation, mouse_button_input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_event::<RotateEvent>()
        .add_event::<ModifiedEvent>()
        .add_startup_system(setup)
        .add_system(mouse_button_input)
        .add_system(do_rotation.after(mouse_button_input))
        .add_system(animate_sprites.after(do_rotation))
        .run();
}
