use bevy::prelude::*;

pub mod resources;

mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app.
      init_resource::<Score>().
      init_resource::<HighScores>().
      // NOTE: Systems are executed in the order specified;
      //       their order here matters!
      add_system(update_score).
      add_system(update_high_scores).
      add_system(check_high_scores);
  }
}
