use bevy::prelude::*;

use crate::ui::hud::systems::updates::{update_enemy_text, update_score_text};
use crate::AppState;

use systems::layout::*;

mod components;
mod styles;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut App) {
    app.
      add_system(spawn_hud.
        in_schedule(OnEnter(AppState::Game))
      ).
      add_system(despawn_hud.
        in_schedule(OnExit(AppState::Game))
      ).
      add_systems((
        update_score_text,
        update_enemy_text
      ).
        in_set(OnUpdate(AppState::Game))
      );
  }
}