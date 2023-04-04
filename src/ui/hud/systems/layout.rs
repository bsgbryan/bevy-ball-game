use bevy::prelude::*;

use crate::ui::{
  hud::{
    components::{
      EnemyText,
      HUD,
      ScoreText,
    },
    styles::{
      HUD_STYLE,
      LHS_STYLE,
      RHS_STYLE,
    }
  },
  styles::{
    BACKGROUND_COLOR
  },
  elements::{
    centered_text,
    img,
  }
};

pub fn spawn_hud(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
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
          parent.spawn(img("star", 48.0, &asset_server));
          parent.spawn((
            centered_text("0", 64.0, &asset_server),
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
          parent.spawn((
            centered_text("0", 64.0, &asset_server),
            EnemyText,
          ));
          parent.spawn(img("ball_red_large", 48.0, &asset_server));
        });
      });
}

pub fn despawn_hud(
  mut commands: Commands,
  hud_query: Query<Entity, With<HUD>>
) {
  for entity in hud_query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}