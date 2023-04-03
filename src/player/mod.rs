use bevy::prelude::*;

pub mod constants;
pub mod components;
mod systems;

use systems::*;

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
      add_startup_system(spawn_player).
      add_system(move_player.in_set(PlayerMovement::Execute)).
      add_system(constrain_player_movement.in_set(PlayerMovement::Constrain));
      // add_systems((
      //   move_player,
      //   contrain_player_movement
      // ).chain());
  }
}