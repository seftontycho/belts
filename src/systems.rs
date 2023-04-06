use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{
    AnimationIndices, AnimationTimer, Position, SpriteCardinal, SpriteDirection, SpriteSelector,
};
use crate::events::{ModifiedEvent, RotateEvent};

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut indices, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = indices.next();
        }
    }
}

pub fn do_rotation(
    mut in_events: EventReader<RotateEvent>,
    mut out_events: EventWriter<ModifiedEvent>,
    mut query: Query<(
        &Position,
        &SpriteSelector,
        &mut SpriteDirection,
        &mut AnimationIndices,
    )>,
) {
    for event in in_events.iter() {
        for (position, selector, mut direction, mut indecies) in query.iter_mut() {
            if *position != event.0 {
                continue;
            }

            if let SpriteDirection::Straight(cardinal) = &*direction {
                let new_direction = SpriteDirection::Straight(match cardinal {
                    SpriteCardinal::Up => SpriteCardinal::Right,
                    SpriteCardinal::Right => SpriteCardinal::Down,
                    SpriteCardinal::Down => SpriteCardinal::Left,
                    SpriteCardinal::Left => SpriteCardinal::Up,
                });

                if let Some(new_indecies) = selector.get_indecies(new_direction.clone()) {
                    indecies.update(new_indecies);
                    *direction = new_direction;

                    out_events.send(ModifiedEvent(event.0.clone()));
                }
            }
        }
    }
}

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut events: EventWriter<RotateEvent>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = cursor_position(&window, &camera, &camera_transform) {
            let position = Position::from_world_position(position);
            eprintln!("Left click at {:?}", position);
            events.send(RotateEvent::new(position));
        }
    }
}

fn cursor_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
