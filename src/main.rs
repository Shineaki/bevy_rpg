mod components;
mod systems;
mod plugins;
use avian2d::math::{Scalar, Vector};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::physics::TiledPhysicsPlugin;
use bevy_ecs_tiled::physics::backend::avian::TiledPhysicsAvianBackend;
use bevy_ecs_tiled::physics::collider::{ColliderCreated, TiledColliderOf, TiledColliderSource};
use bevy_ecs_tiled::prelude::TilemapAnchor;
use bevy_ecs_tiled::tiled::TiledPlugin;
use bevy_ecs_tiled::tiled::event::TiledEvent;
use bevy_ecs_tiled::tiled::map::TiledMap;
use bevy_ecs_tiled::tiled::map::asset::TiledMapAsset;
use components::animation::{AnimationIndices, AnimationState, AnimationTimer};
use components::utils::Player;
use components::utils::{MainCamera, MyWorldCoords};
use systems::animation::{animate_sprite, handle_animation_transitions};
use systems::input_handling::mouse_button_input;
use plugins::movement_plugin::CharacterControllerPlugin;

use crate::plugins::movement_plugin::CharacterControllerBundle;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("dungeon/elf_m_idle_walk.png");
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 16, y: 32 }, 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices {
        idle: (0, 3),    // frames 0-3 for idle
        walking: (4, 7), // frames 4-7 for walking
    };

    let map_handle: Handle<TiledMapAsset> = asset_server.load("map_1.tmx");

    // Spawn a new entity with the TiledMap component

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
        Collider::capsule(12.5, 20.0),
        AnimationState::Idle,
    ));
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
        Transform::from_scale(Vec3::splat(3.0)),
    )).observe(|collider_created: On<TiledEvent<ColliderCreated>>, mut commands: Commands| {
            // Filter collider created from Tiled objects
            println!("Collider created from Tiled: {:?}", collider_created.event());
            if collider_created.event().event.source == TiledColliderSource::TilesLayer {
                // Add a RigidBody::Static to the collider entity
                commands
                    .entity(collider_created.event().origin)
                    .insert(RigidBody::Static);
            }
        });
    // player2
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
            100.0,
            25.0,
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.7),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(50.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(TiledPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
        .add_plugins(CharacterControllerPlugin)
        .insert_resource(Gravity(Vector::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_button_input)
        .add_systems(
            Update,
            (
                mouse_button_input,
                handle_animation_transitions,
                animate_sprite,
            )
                .chain(),
        )
        
        .run();
}
