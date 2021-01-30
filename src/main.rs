mod goal_line;
mod item;
mod score;
mod velocity;

use bevy::prelude::*;

pub struct Materials {
    item_good_material: Handle<ColorMaterial>,
    item_bad_material: Handle<ColorMaterial>,
    none: Handle<ColorMaterial>,
}

pub struct Score(u32);

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(Materials {
            item_good_material: materials.add(Color::rgb(0.3, 0.9, 0.6).into()),
            item_bad_material: materials.add(Color::rgb(0.9, 0.3, 0.6).into()),
            none: materials.add(Color::NONE.into()),
        });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "C0DE".into(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1).into()))
        .add_resource(Score(0))
        .add_startup_system(setup.system())
        .add_startup_stage(
            "setup",
            SystemStage::parallel()
                .with_system(goal_line::spawn_goal_line.system())
                .with_system(score::setup_score.system()),
        )
        .add_system(item::spawn_item.system())
        .add_system(item::item_collision.system())
        .add_system(item::accelerate_item.system())
        .add_system(goal_line::goal_line_movement.system())
        .add_system(velocity::movement.system())
        .add_system(score::update_score_text.system())
        .add_plugins(DefaultPlugins)
        .run()
}
