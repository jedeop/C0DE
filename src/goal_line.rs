use bevy::prelude::*;

use crate::Materials;

pub struct GoalLine;

pub fn spawn_goal_line(commands: &mut Commands, windows: Res<Windows>, materials: Res<Materials>) {
    let window = windows.get_primary().unwrap();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(400.0, 50.0)),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                window.height() / -2.0 + 25.0,
                0.0,
            )),
            material: materials.item_good_material.clone(),
            ..Default::default()
        })
        .with(GoalLine);
}

pub fn goal_line_movement(
    mut goal_lines: Query<&mut Transform, With<GoalLine>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for mut transform in goal_lines.iter_mut() {
        if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
            transform.translation.x -= 750.0 * delta;
        }
        if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
            transform.translation.x += 750.0 * delta;
        }
    }
}
