mod item;
mod velocity;

use bevy::prelude::*;

pub struct Materials {
    item_material: Handle<ColorMaterial>,
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            item_material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "C0DE".into(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1).into()))
        .add_startup_system(setup.system())
        .add_system(item::spawn_item.system())
        .add_system(item::despawn_item.system())
        .add_system(item::accelerate_item.system())
        .add_system(velocity::movement.system())
        .add_plugins(DefaultPlugins)
        .run()
}
