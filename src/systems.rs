use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{AnimationIndices, AnimationTimer, Belt, Position, SpriteSelector};
use crate::events::{FindEdgesEvent, InputEvent, ModifiedEvent, UpdateEdgesEvent};

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

pub fn update_edges(
    mut in_events: EventReader<UpdateEdgesEvent>,
    mut query: Query<(&mut Belt, &mut AnimationIndices, &SpriteSelector)>,
) {
    for event in in_events.iter() {
        let (mut belt, mut indecies, selector) = query.get_mut(event.entity).unwrap();
        belt.start = event.new_start.clone();
        indecies.update(selector.get_indecies(&belt));
    }
}

pub fn handle_modifications(
    mut in_events: EventReader<ModifiedEvent>,
    mut out_events: EventWriter<FindEdgesEvent>,
    main_query: Query<(Entity, &Position)>,
) {
    for event in in_events.iter() {
        if let Some(entity) = event.entity {
            out_events.send(FindEdgesEvent { entity });
        }

        for adj_position in event.position.adjacent() {
            if let Some(adj_entity) = get_entity_at_position(adj_position, &main_query) {
                out_events.send(FindEdgesEvent { entity: adj_entity });
            }
        }
    }
}

pub fn find_edges(
    mut in_events: EventReader<FindEdgesEvent>,
    mut out_events: EventWriter<UpdateEdgesEvent>,
    main_query: Query<(&Position, &mut Belt)>,
    other_query: Query<(Entity, &Position)>,
) {
    for event in in_events.iter() {
        // Should be fine to unwrap here, since the event is only sent
        // when the entity is found, unlikley to delete it in the same frame
        let (center_position, center_belt) =
            main_query.get(event.entity).expect("Entity not found");

        let ends: Vec<_> = center_position
            .adjacent()
            .iter()
            .filter_map(|adj_position| {
                let entity = get_entity_at_position(adj_position.clone(), &other_query)?;
                let (adj_position, adj_belt) = main_query.get(entity).ok()?;

                // remove the front belt
                if center_position.direction_to(adj_position).unwrap() == center_belt.end {
                    return None;
                }

                // can unwrap here as we know belts are adjacent
                if adj_position.direction_to(center_position).unwrap() == adj_belt.end {
                    return Some(adj_belt.end.clone());
                } else {
                    return None;
                }
            })
            .collect();

        if ends.len() == 1 {
            // eprintln!("Found one end: {:?}", ends.get(0).unwrap());
            let end = ends.get(0).unwrap();

            // if the one left is not behind current belt then belt is curved
            if *end != center_belt.end {
                out_events.send(UpdateEdgesEvent::new(event.entity, end.clone().opposite()));
                continue;
            }
        }

        // eprintln!("Belt should be straight");
        let start = center_belt.end.opposite();
        out_events.send(UpdateEdgesEvent::new(event.entity, start));
    }
}

fn get_entity_at_position(
    position: Position,
    query: &Query<(Entity, &Position)>,
) -> Option<Entity> {
    for (entity, pos) in query.iter() {
        if *pos == position {
            return Some(entity);
        }
    }

    None
}

pub fn handle_inputs(
    mut in_events: EventReader<InputEvent>,
    mut out_events: EventWriter<ModifiedEvent>,
    mut main_query: Query<(Entity, &mut Belt)>,
    other_query: Query<(Entity, &Position)>,
) {
    for event in in_events.iter() {
        match event {
            InputEvent::Create(position) => {
                todo!("Create belt at {:?}", position);
            }
            InputEvent::Delete(position) => {
                todo!("Delete belt at {:?}", position);
            }
            InputEvent::Rotate(position) => {
                if let Some(entity) = get_entity_at_position(position.clone(), &other_query) {
                    let (entity, mut belt) = main_query.get_mut(entity).unwrap();
                    let new_end = belt.end.rotate_clockwise();
                    belt.rotate(new_end);

                    out_events.send(ModifiedEvent::new(position.clone(), Some(entity)));
                }
            }
        }
    }
}

pub fn keyboard_input(
    keyboard: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut events: EventWriter<InputEvent>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if keyboard.just_pressed(KeyCode::R) {
        if let Some(position) = cursor_position(&window, &camera, &camera_transform) {
            let position = Position::from_world_position(position);
            eprintln!("Rotate at {:?}", position);
            events.send(InputEvent::Rotate(position));
        }
    }
}

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut events: EventWriter<InputEvent>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = cursor_position(&window, &camera, &camera_transform) {
            let position = Position::from_world_position(position);
            eprintln!("Left click at {:?}", position);
            // place belt
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        if let Some(position) = cursor_position(&window, &camera, &camera_transform) {
            let position = Position::from_world_position(position);
            eprintln!("Right click at {:?}", position);
            // destroy belt
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
