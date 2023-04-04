use bevy::prelude::*;

use bevy::app::AppExit;

use crate::AppState;
use crate::game::SimulationState;
use crate::ui::components::{
  MainMenuButton,
  QuitButton
};
use crate::ui::pause_menu::components::*;
use crate::ui::styles::{
  PRESSED_BUTTON_COLOR,
  HOVERED_BUTTON_COLOR,
  NORMAL_BUTTON_COLOR
};

pub fn interact_with_resume_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ResumeButton>),
  >,
  mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
  for (interaction, mut color) in button_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON_COLOR.into();
        simulation_state_next_state.set(SimulationState::Running);
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
  mut app_state_next_state: ResMut<NextState<AppState>>,
) {
  for (interaction, mut color) in button_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::MainMenu);
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