use bevy::prelude::*;

use belts::events::{FindEdgesEvent, InputEvent, ModifiedEvent, UpdateEdgesEvent};
use belts::setup::setup;
use belts::systems::{
    animate_sprites, find_edges, handle_inputs, handle_modifications, keyboard_input,
    mouse_button_input, update_edges,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_event::<InputEvent>()
        .add_event::<ModifiedEvent>()
        .add_event::<FindEdgesEvent>()
        .add_event::<UpdateEdgesEvent>()
        .add_startup_system(setup)
        .add_system(mouse_button_input)
        .add_system(keyboard_input)
        .add_system(handle_inputs)
        .add_system(handle_modifications)
        .add_system(find_edges)
        .add_system(update_edges.after(find_edges))
        .add_system(animate_sprites)
        .run();
}
