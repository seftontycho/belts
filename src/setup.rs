use bevy::prelude::*;

use crate::components::{
    AnimationIndices, AnimationTimer, Position, SpriteCardinal, SpriteDirection, SpriteSelector,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("hr-transport-belt.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(68.0, 68.0),
        16,
        25,
        Some(Vec2::new(60.0, 60.0)),
        Some(Vec2::new(30.0, 38.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut animation_indecies = AnimationIndices::new(0, 0);
    let indecies: Vec<usize> = vec![2, 3, 1, 0, 4, 11, 7, 8, 6, 9, 10, 5];
    let indecies: Vec<Option<(usize, usize)>> = indecies
        .iter()
        .map(|i| Some((i * 16, (i + 1) * 16 - 1)))
        .collect();
    let sprite_selector = SpriteSelector::from_indecies(indecies.try_into().unwrap());
    let direction = SpriteDirection::Straight(SpriteCardinal::Right);

    animation_indecies.update(sprite_selector.get_indecies(direction.clone()).unwrap());

    for i in 0..10 {
        eprintln!("Spawning entity at: {:?}", Position::new(i as i32, 0));

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indecies.first()),
                transform: Transform::from_translation(Vec3::new(64.0 * i as f32, 0.0, 0.0)),
                ..default()
            })
            .insert(animation_indecies.clone())
            .insert(direction.clone())
            .insert(AnimationTimer(Timer::from_seconds(
                1.0 / 60.0,
                TimerMode::Repeating,
            )))
            .insert(Position::new(i as i32, 0))
            .insert(sprite_selector.clone());
    }
}
