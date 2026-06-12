use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::utils::{MainCamera, MyWorldCoords};

pub fn mouse_button_input(
    mut mycoords: ResMut<MyWorldCoords>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *camera_query;
    if let Some(cursor_position) = window.cursor_position()
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
    {
        mycoords.x = ray.origin.x;
        mycoords.y = ray.origin.y;
    }

    if buttons.just_pressed(MouseButton::Left) {
        println!(
            "{:?} is just_pressed at {:?} - world position is {:?}",
            buttons,
            window.cursor_position(),
            mycoords.0
        );
    }
    if buttons.just_released(MouseButton::Left) {
        println!(
            "{:?} is just_released at {:?} - world position is {:?}",
            buttons,
            window.cursor_position(),
            mycoords.0
        );
    }
    if buttons.pressed(MouseButton::Right) {
        println!(
            "{:?} is pressed at {:?} - world position is {:?}",
            buttons,
            window.cursor_position(),
            mycoords.0
        );
    }
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Middle]) {
        println!(
            "{:?} is any just pressed at {:?} - world position is {:?}",
            buttons,
            window.cursor_position(),
            mycoords.0
        );
    }
}