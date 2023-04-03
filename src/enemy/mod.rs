use bevy::prelude::*;

pub mod constants;
pub mod components;
pub mod resources;

mod systems;

use resources::EnemySpawnTimer;

use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.
      init_resource::<EnemySpawnTimer>().
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      add_startup_system(spawn_enemies).
      add_system(move_enemy).
      add_system(bounce_enemy_off_screen_border).
      add_system(constrain_enemy_movement).
      add_system(update_enemy_spawn_timer).
      add_system(spawn_enemies_over_time);
  }
}
