mod components;
mod systems;

use bevy::prelude::*;
use systems::animation::{animate_sprite, handle_animation_transitions};
use systems::input_handling::mouse_button_input;
use components::animation::{AnimationIndices, AnimationTimer, AnimationState};
use components::utils::{MainCamera, MyWorldCoords};
use components::utils::Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("dungeon/elf_m_idle_walk.png");
    let layout = TextureAtlasLayout::from_grid(UVec2{x: 16, y: 32}, 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices {
        idle: (0, 3),      // frames 0-3 for idle
        walking: (4, 7),   // frames 4-7 for walking
    };


    commands.init_resource::<MyWorldCoords>();
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.idle.0,
            },
        ),
        Transform::from_scale(Vec3::splat(3.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationState::Idle
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_button_input)
        .add_systems(Update, (mouse_button_input,handle_animation_transitions, animate_sprite).chain())
        .run();
}
