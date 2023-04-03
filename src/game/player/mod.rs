use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub mod constants;
pub mod components;

mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerMovement {
  Execute,
  Constrain,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.
      configure_set(PlayerMovement::Execute.before(PlayerMovement::Constrain)).
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      // add_startup_system(spawn_player).
      add_system(spawn_player.
        in_schedule(OnEnter(AppState::Game))
      ).
      add_system(despawn_player.
        in_schedule(OnExit(AppState::Game))
      ).
      add_systems((
        move_player.in_set(PlayerMovement::Execute),
        constrain_player_movement.in_set(PlayerMovement::Constrain)
      ).
        in_set(OnUpdate(AppState::Game)).
        in_set(OnUpdate(SimulationState::Running))
      );
      // add_system(move_player.
      //   in_set(PlayerMovement::Execute).
      //   run_if(in_state(AppState::Game)).
      //   run_if(in_state(SimulationState::Running))
      // ).
      // add_system(constrain_player_movement.
      //   in_set(PlayerMovement::Constrain).
      //   run_if(in_state(AppState::Game)).
      //   run_if(in_state(SimulationState::Running))
      // );
      // add_systems((
      //   move_player,
      //   contrain_player_movement
      // ).chain());
  }
}