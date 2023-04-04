use bevy::prelude::*;

use crate::ui::{
  components::{
    QuitButton,
    MainMenuButton,
  },
  elements::{
    btn,
    centered_text,
  },
  game_over_menu::{
    components::{
      GameOverMenu,
      FinalScoreText,
      RestartButton,
    },
    styles::{
      ELEMENT_STYLE,
      CONTAINER_STYLE,
    }
  }
};

pub fn spawn_game_over_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  commands.
    spawn((
      NodeBundle {
        style: ELEMENT_STYLE,
        z_index: ZIndex::Local(2), // See Ref. 1
        ..default()
      },
      GameOverMenu,
    )).
    with_children(|parent| {
      parent.
        spawn(NodeBundle {
          style: CONTAINER_STYLE,
          background_color: Color::BLACK.into(),
          ..default()
        }).
        with_children(|parent| {
          parent.spawn(centered_text("Game Over", 64.0, &asset_server));

          parent.spawn( (
            centered_text("Doesn't matter", 32.0, &asset_server),
            FinalScoreText,
          ));

          parent.
            spawn(btn::<RestartButton>()).
            with_children(|parent| {
              parent.spawn(centered_text("Restart", 32.0, &asset_server));
            });

          parent.
            spawn(btn::<MainMenuButton>()).
            with_children(|parent| {
              parent.spawn(centered_text("Main Menu", 32.0, &asset_server));
            });

          parent.
            spawn(btn::<QuitButton>()).
            with_children(|parent| {
              parent.spawn(centered_text("Quit", 32.0, &asset_server));
            });
        });
    });
}

pub fn despawn_game_over_menu(
  mut commands: Commands,
  game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
  if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
    commands.entity(game_over_menu_entity).despawn_recursive();
  }
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs