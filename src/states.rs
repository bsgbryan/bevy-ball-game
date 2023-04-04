use bevy::prelude::States;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
  #[default]
  MainMenu,
  Game,
  GameOver,
}