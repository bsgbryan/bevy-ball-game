use bevy::prelude::*;

use systems::*;

use game::{GamePlugin, SimulationState};
use main_menu::MainMenuPlugin;

pub mod events;

mod systems;
mod game;
mod main_menu;

fn main() {
  App::new().
    add_plugins(DefaultPlugins).
    add_state::<AppState>().
    add_plugin(GamePlugin).
    add_plugin(MainMenuPlugin).
    // NOTE: Systems are executed in the order specified;
    //       their order here matters!
    add_startup_system(spawn_camera).
    // add_system(detect_collision.
    //   run_if(in_state(AppState::Game)).
    //   run_if(in_state(SimulationState::Running))
    // ).
    // add_system(detect_pickup.
    //   run_if(in_state(AppState::Game)).
    //   run_if(in_state(SimulationState::Running))
    // ).
    add_system(transition_to_game_state).
    add_system(transition_to_main_menu_state).
    add_system(exit_game).
    add_system(handle_game_over).
    add_systems((
      detect_collision,
      detect_pickup,
    ).
      in_set(OnUpdate(AppState::Game)).
      in_set(OnUpdate(SimulationState::Running))
    ).
    run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
  #[default]
  MainMenu,
  Game,
  GameOver,
}
