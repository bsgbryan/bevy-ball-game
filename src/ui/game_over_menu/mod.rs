use bevy::prelude::*;

use crate::AppState;

use systems::interactions::*;
use systems::layout::*;
use systems::updates::*;

mod components;
mod styles;
mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
  fn build(&self, app: &mut App) {
    app.
      add_system(spawn_game_over_menu.
        in_schedule(OnEnter(AppState::GameOver))
      ).
      add_system(despawn_game_over_menu.
        in_schedule(OnExit(AppState::GameOver))
      ).
      add_systems((
        interact_with_restart_button,
        interact_with_main_menu_button,
        interact_with_quit_button,
        update_final_score_text,
      )
        .in_set(OnUpdate(AppState::GameOver))
      );
  }
}