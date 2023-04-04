use bevy::prelude::*;

use crate::{ui::pause_menu::components::{
  PauseMenu,
  ResumeButton,
}, ui::{pause_menu::styles::{PAUSE_MENU_STYLE, PAUSE_MENU_CONTAINER_STYLE}, styles::{BACKGROUND_COLOR, text_style, BUTTON_STYLE, NORMAL_BUTTON_COLOR}, components::{MainMenuButton, QuitButton}}};

pub fn spawn_pause_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
  mut commands: Commands,
  pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
  if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
    commands.entity(pause_menu_entity).despawn_recursive();
  }
}

// System Piping Example
pub fn build_pause_menu(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>
) -> Entity {
  commands.
    spawn((
      NodeBundle {
        style: PAUSE_MENU_STYLE,
        z_index: ZIndex::Local(1), // See Ref. 1
        ..default()
      },
      PauseMenu,
    )).
    with_children(|parent| {
      parent.
        spawn(NodeBundle {
          style: PAUSE_MENU_CONTAINER_STYLE,
          background_color: BACKGROUND_COLOR.into(),
          ..default()
        }).
        with_children(|parent| {
          // Title
          parent.
            spawn(TextBundle {
              text: Text {
                sections: vec![TextSection::new(
                  "Pause Menu",
                  text_style(64.0, &asset_server),
                )],
                alignment: TextAlignment::Center,
                ..default()
              },
              ..default()
            });
          // Resume Button
          parent.
            spawn((
              ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
              },
              ResumeButton,
            )).
            with_children(|parent| {
              parent.
                spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new(
                      "Resume",
                      text_style(32.0, &asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
            });
          // Main Menu Button
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
          // Quit Button
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

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs