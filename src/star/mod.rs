use bevy::prelude::*;

pub mod constants;
pub mod components;
pub mod resources;

mod systems;

use resources::StarSpawnTimer;

use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
  fn build(&self, app: &mut App) {
    app.
      init_resource::<StarSpawnTimer>().
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      add_startup_system(spawn_stars).
      add_system(update_star_spawn_timer).
      add_system(spawn_stars_over_time);
  }
}
