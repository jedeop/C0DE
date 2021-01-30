use bevy::prelude::*;

use crate::{Materials, Score};

pub struct ScoreText;

pub fn setup_score(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.none.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "0".to_string(),
                        font: asset_server.load("fonts/FiraCode-SemiBold.ttf"),
                        style: TextStyle {
                            font_size: 200.0,
                            color: Color::rgba(0.5, 0.5, 0.5, 0.1),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}

pub fn update_score_text(mut texts: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in texts.iter_mut() {
        text.value = format!("{}", score.0);
    }
}
