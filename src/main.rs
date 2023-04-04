use bevy::prelude::*;

use systems::*;

use ui::UIPlugin;

use game::{
  GamePlugin,
  SimulationState,
};

use states::AppState;

pub mod events;

mod game;
mod states;
mod systems;
mod ui;

fn main() {
  App::new().
    add_plugins(DefaultPlugins).
    add_state::<AppState>().
    add_plugin(GamePlugin).
    add_plugin(UIPlugin).
    add_startup_system(spawn_camera).
    add_system(transition_to_game_state).
    add_system(transition_to_main_menu_state).
    add_system(exit_game).
    add_system(handle_game_over).
    // NOTE: Systems are executed in the order specified;
    //       their order here matters!
    add_systems((
      detect_collision,
      detect_pickup,
    ).
      in_set(OnUpdate(AppState::Game)).
      in_set(OnUpdate(SimulationState::Running))
    ).
    run();
}
