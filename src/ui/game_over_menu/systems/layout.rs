use bevy::prelude::*;

use crate::ui::components::{
  QuitButton,
  MainMenuButton
};
use crate::ui::styles::*;
use crate::ui::game_over_menu::{
  components::{
    GameOverMenu,
    FinalScoreText,
    RestartButton
  },
  styles::{
    GAME_OVER_MENU_STYLE,
    GAME_OVER_MENU_CONTAINER_STYLE
  },
};

pub fn spawn_game_over_menu(
  mut commands: Commands, asset_server: Res<AssetServer>
) {
  build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
  commands.
    spawn((
      NodeBundle {
        style: GAME_OVER_MENU_STYLE,
        z_index: ZIndex::Local(2), // See Ref. 1
        ..default()
      },
      GameOverMenu,
    )).
    with_children(|parent| {
      parent.
        spawn(NodeBundle {
          style: GAME_OVER_MENU_CONTAINER_STYLE,
          background_color: Color::BLACK.into(),
          ..default()
        }).
        with_children(|parent| {
          // === TITLE ===
          parent.
            spawn(TextBundle {
              text: Text {
                sections: vec![TextSection::new(
                  "Game Over",
                  text_style(64.0, &asset_server),
                )],
                alignment: TextAlignment::Center,
                ..default()
              },
              ..default()
            });

          // === FINAL SCORE TEXT ===
          parent.
            spawn((
              TextBundle {
                text: Text {
                  sections: vec![TextSection::new(
                    "Your final score was:",
                    text_style(32.0, &asset_server),
                  )],
                  alignment: TextAlignment::Center,
                  ..default()
                },
                ..default()
              },
              FinalScoreText,
            ));

          // === RESTART BUTTON ===
          parent.
            spawn((
              ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
              },
              RestartButton,
            )).
            with_children(|parent| {
              parent.
                spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new(
                      "Restart",
                      text_style(32.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
            });

          // === MAIN MENU BUTTON ===
          parent.
            spawn((
              ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
              },
              MainMenuButton,
            )).
            with_children(|parent| {
              parent.
                spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new(
                      "Main Menu",
                      text_style(32.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
            });

          // === QUIT BUTTON ===
          parent.
            spawn((
              ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
              },
              QuitButton,
            )).
            with_children(|parent| {
              parent.
                spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new(
                      "Quit",
                      text_style(32.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
            });
        });
    })
    .id()
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