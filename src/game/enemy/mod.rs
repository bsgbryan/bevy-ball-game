use bevy::prelude::*;

use crate::AppState;

pub mod constants;
pub mod components;
pub mod resources;

mod systems;

use resources::EnemySpawnTimer;

use systems::*;

use super::SimulationState;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.
      init_resource::<EnemySpawnTimer>().
      // add_startup_system(spawn_enemies).
      add_system(spawn_enemies.
        in_schedule(OnEnter(AppState::Game))
      ).
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      add_systems((
        move_enemy,
        bounce_enemy_off_screen_border,
        constrain_enemy_movement,
        update_enemy_spawn_timer,
        spawn_enemies_over_time,
      ).
        in_set(OnUpdate(AppState::Game)).
        in_set(OnUpdate(SimulationState::Running))
      ).
      // add_system(move_enemy.
      //   run_if(in_state(AppState::Game)).
      //   run_if(in_state(SimulationState::Running)) 
      // ).
      // add_system(bounce_enemy_off_screen_border).
      // add_system(constrain_enemy_movement).
      // add_system(update_enemy_spawn_timer).
      // add_system(spawn_enemies_over_time);
      add_system(despawn_enemies.
        in_schedule(OnExit(AppState::Game))
      );
  }
}
