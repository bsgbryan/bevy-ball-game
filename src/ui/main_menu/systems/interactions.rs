use bevy::{
  prelude::*,
  app::AppExit,
};

use crate::{
  AppState,
  game::SimulationState,
  ui::{
    main_menu::{
      components::{
        PlayButton
      }
    },
    components::QuitButton, interactions::handle
  }
};

pub fn interact_with_play_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
  >,
  mut next_app_state: ResMut<NextState<AppState>>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  if let Ok((interaction, color)) = button_query.get_single_mut() {
    handle(interaction, color, || {
      next_app_state.set(AppState::Game);
      next_sim_state.set(SimulationState::Running);
    });
  }
}

pub fn interact_with_quit_button(
  mut app_exit_event_writer: EventWriter<AppExit>,
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
  >
) {
  if let Ok((interaction, color)) = button_query.get_single_mut() {
    handle(interaction, color, || {
      app_exit_event_writer.send(AppExit);
    });
  }
}