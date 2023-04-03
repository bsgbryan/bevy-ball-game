use bevy::prelude::*;

pub mod constants;
pub mod components;
pub mod resources;

mod systems;

use resources::StarSpawnTimer;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
  fn build(&self, app: &mut App) {
    app.
      init_resource::<StarSpawnTimer>().
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      // add_startup_system(spawn_stars).
      add_system(spawn_stars.
        in_schedule(OnEnter(AppState::Game))        
      ).
      add_system(despawn_stars.
        in_schedule(OnExit(AppState::Game))
      ).
      add_systems((
        update_star_spawn_timer,
        spawn_stars_over_time,
      ).
        in_set(OnUpdate(AppState::Game)).
        in_set(OnUpdate(SimulationState::Running))
      );
      // add_system(update_star_spawn_timer).
      // add_system(spawn_stars_over_time);
  }
}
