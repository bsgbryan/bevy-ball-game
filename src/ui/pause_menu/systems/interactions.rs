use bevy::prelude::*;

use bevy::app::AppExit;

use crate::AppState;
use crate::game::SimulationState;
use crate::ui::interactions::handle;
use crate::ui::{
  components::{
    MainMenuButton,
    QuitButton
  },
  pause_menu::components::ResumeButton
};

pub fn interact_with_resume_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ResumeButton>),
  >,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  for (interaction, color) in button_query.iter_mut() {
    handle(interaction, color, || {
      next_sim_state.set(SimulationState::Running);
    });
  }
}

pub fn interact_with_main_menu_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<MainMenuButton>),
  >,
  mut next_app_state: ResMut<NextState<AppState>>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  for (interaction, color) in button_query.iter_mut() {
    handle(interaction, color, || {
      next_app_state.set(AppState::MainMenu);
      next_sim_state.set(SimulationState::Unloaded);
    });
  }
}

pub fn interact_with_quit_button(
  mut app_exit_event_writer: EventWriter<AppExit>,
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>),
  >,
) {
  for (interaction, color) in button_query.iter_mut() {
    handle(interaction, color, || {
      app_exit_event_writer.send(AppExit);
    });
  }
}