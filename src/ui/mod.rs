use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;

pub mod components;
pub mod elements;
pub mod styles;
pub mod interactions;

mod game_over_menu;
mod hud;
mod main_menu;
mod pause_menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
  fn build(&self, app: &mut App) {
    app.
      add_plugin(MainMenuPlugin).
      add_plugin(HudPlugin).
      add_plugin(PauseMenuPlugin).
      add_plugin(GameOverMenuPlugin);
  }
}