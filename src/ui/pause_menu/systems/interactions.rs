use bevy::prelude::*;

use bevy::app::AppExit;

use crate::AppState;
use crate::game::SimulationState;
use crate::ui::{
  components::{
    MainMenuButton,
    QuitButton
  },
  pause_menu::components::ResumeButton,
  styles::{
    PRESSED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    NORMAL_BUTTON_COLOR
  }
};

pub fn interact_with_resume_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ResumeButton>),
  >,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  for (interaction, mut color) in button_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON_COLOR.into();

        next_sim_state.set(SimulationState::Running);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON_COLOR.into();
      }
    }
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
  for (interaction, mut color) in button_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON_COLOR.into();

        next_app_state.set(AppState::MainMenu);
        next_sim_state.set(SimulationState::Unloaded);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

pub fn interact_with_quit_button(
  mut app_exit_event_writer: EventWriter<AppExit>,
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>),
  >,
) {
  for (interaction, mut color) in button_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON_COLOR.into();

        app_exit_event_writer.send(AppExit);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}