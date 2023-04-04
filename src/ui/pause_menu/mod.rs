use bevy::prelude::*;

use crate::game::SimulationState;
use crate::states::AppState;

use systems::interactions::*;
use systems::layout::*;

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
  fn build(&self, app: &mut App) {
    app.
      add_system(spawn_pause_menu.
        in_schedule(OnEnter(SimulationState::Paused))
      ).
      add_system(despawn_pause_menu.
        in_schedule(OnExit(SimulationState::Paused))
      ).
      add_systems((
        interact_with_resume_button,
        interact_with_main_menu_button,
        interact_with_quit_button,
      ).
        in_set(OnUpdate(AppState::Game)).
        in_set(OnUpdate(SimulationState::Paused)),
      );
  }
}