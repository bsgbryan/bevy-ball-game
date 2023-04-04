use bevy::prelude::*;

use crate::ui::{
  components::{
    MainMenuButton,
    QuitButton,
  },
  elements::{
    btn,
    centered_text,
  },
  pause_menu::{
    components::{
      PauseMenu,
      ResumeButton,
    },
    styles::{
      PAUSE_MENU_STYLE,
      PAUSE_MENU_CONTAINER_STYLE
    }
  },
  styles::BACKGROUND_COLOR,
};

pub fn spawn_pause_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
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
          parent.spawn(centered_text("Pause Menu", 64.0, &asset_server));
          
          parent.
            spawn(btn::<ResumeButton>()).
            with_children(|parent| {
              parent.spawn(centered_text("Resume", 32.0, &asset_server));
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

pub fn despawn_pause_menu(
  mut commands: Commands,
  pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
  if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
    commands.entity(pause_menu_entity).despawn_recursive();
  }
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs