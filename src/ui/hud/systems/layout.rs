use bevy::prelude::*;

use crate::ui::{
  hud::{
    components::{
      EnemyText,
      HUD,
      ScoreText
    },
    styles::{
      HUD_STYLE,
      IMAGE_STYLE,
      LHS_STYLE,
      RHS_STYLE
    }
  },
  styles::{
    text_style,
    BACKGROUND_COLOR
  }
};

pub fn spawn_hud(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  build_hud(&mut commands, &asset_server);
}

pub fn build_hud(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>
) -> Entity {
    commands.
      spawn((
        NodeBundle {
          style: HUD_STYLE,
          ..default()
        },
        HUD,
      )).
      with_children(|parent| {
        // LHS
        parent.
          spawn(NodeBundle {
              style: LHS_STYLE,
              background_color: BACKGROUND_COLOR.into(),
              ..default()
          }).
          with_children(|parent| {
            // Star Image
            parent.
              spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/star.png").into(),
                ..default()
              });
            // Score Text
            parent.
              spawn((
                TextBundle {
                  text: Text {
                    sections: vec![TextSection::new(
                      "0",
                      text_style(64.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                },
                ScoreText,
              ));
        });
        // RHS
        parent.
          spawn(NodeBundle {
            style: RHS_STYLE,
            background_color: BACKGROUND_COLOR.into(),
            ..default()
          }).
          with_children(|parent| {
            // Enemy Text
            parent.
              spawn((
                TextBundle {
                  style: Style { ..default() },
                  text: Text {
                    sections: vec![TextSection::new(
                      "0",
                      text_style(64.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                },
                EnemyText,
              ));
            // Enemy Image
            parent.
              spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/ball_red_large.png").into(),
                ..default()
              });
          });
        })
        .id()
}

pub fn despawn_hud(
  mut commands: Commands,
  hud_query: Query<Entity, With<HUD>>
) {
  for entity in hud_query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}