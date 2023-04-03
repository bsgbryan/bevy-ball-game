use bevy::prelude::*;

use crate::events::GameOver;

use super::resources::Score;
use super::resources::HighScores;

pub fn init_score(
  mut commands: Commands
) {
  commands.insert_resource(Score::default());
}

pub fn remove_score(
  mut commands: Commands
) {
  commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
  if score.is_changed() {
    println!("Great job! New score is {}", score.value.to_string());
  }
}

pub fn update_high_scores(
  mut game_over_event_reader: EventReader<GameOver>,
  mut high_scores: ResMut<HighScores>,
) {
  for event in game_over_event_reader.iter() {
    high_scores.scores.push(("ME".to_string(), event.score));
  }
}

pub fn check_high_scores(
  high_scores: Res<HighScores>
) {
  if high_scores.is_changed() {
    println!("High scores: {:?}", high_scores);
  }
}
