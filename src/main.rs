pub mod events;

mod systems;

mod enemy;
mod player;
mod score;
mod star;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

fn main() {
  App::new().
    add_plugins(DefaultPlugins).
    add_event::<GameOver>().
    add_plugin(EnemyPlugin).
    add_plugin(PlayerPlugin).
    add_plugin(ScorePlugin).
    add_plugin(StarPlugin).
    // NOTE: Systems are executed in the order specified;
    //       their order here matters!
    add_startup_system(spawn_camera).
    add_system(detect_collision).
    add_system(detect_pickup).
    add_system(exit_game).
    add_system(handle_game_over).
    run();
}