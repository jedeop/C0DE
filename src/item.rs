const ITEM_SIZE: f32 = 50.0;

use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

use crate::{Materials, Score, goal_line::GoalLine, velocity::Velocity};

pub enum Item {
    GOOD,
    BAD,
}

pub struct ItemSpawnTimer(Timer);
impl Default for ItemSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(750), true))
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: Local<ItemSpawnTimer>,
    windows: Res<Windows>,
) {
    if timer.0.tick(time.delta_seconds()).finished() {
        let window = windows.get_primary().unwrap();
        let x = (rand::random::<f32>() * window.width()) - (window.width() / 2.0);
        let y = window.height() / 2.0 + ITEM_SIZE / 2.0;

        let (item, material) = if rand::thread_rng().gen_range(0..3) == 0 {
            (Item::GOOD, materials.item_good_material.clone())
        } else {
            (Item::BAD, materials.item_bad_material.clone())
        };
        commands
            .spawn(SpriteBundle {
                material,
                sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            })
            .with(item)
            .with(Velocity(Vec3::new(0.0, 0.0, 0.0)));
    }
}

pub fn item_collision(
    commands: &mut Commands,
    mut items: Query<(Entity, &Transform, &Sprite, &Item)>,
    goal_lines: Query<(&Transform, &Sprite), With<GoalLine>>,
    windows: Res<Windows>,
    mut score: ResMut<Score>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, transform, sprite, item) in items.iter_mut() {
        if transform.translation.y < 0.0 - window.height() / 2.0 - ITEM_SIZE / 2.0 {
            commands.despawn(entity);
        }

        for (goal_line_transform, goal_line_sprite) in goal_lines.iter() {
            if let Some(_) = collide(
                transform.translation,
                sprite.size,
                goal_line_transform.translation,
                goal_line_sprite.size,
            ) {
                match item {
                    Item::GOOD => score.0 += 1,
                    Item::BAD => if score.0 != 0 { score.0 -= 1 },
                };
                println!("score: {}", score.0);
                commands.despawn(entity);
            }
        }
    }
}

pub fn accelerate_item(mut velocities: Query<&mut Velocity, With<Item>>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut velocity in velocities.iter_mut() {
        velocity.0 += Vec3::new(0.0, -1.0, 0.0) * 800.0 * delta;
    }
}
