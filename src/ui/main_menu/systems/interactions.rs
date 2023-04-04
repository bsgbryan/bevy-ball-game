use bevy::{
  prelude::*,
  app::AppExit,
};

use crate::{
  AppState,
  ui::{
    styles::{
      NORMAL_BUTTON_COLOR,
      PRESSED_BUTTON_COLOR,
      HOVERED_BUTTON_COLOR,
    },
    main_menu::{
      components::{
        PlayButton
      }
    }, components::QuitButton
  }
};

pub fn interact_with_play_button(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
  >,
  mut next_app_state: ResMut<NextState<AppState>>,
) {
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *background_color = PRESSED_BUTTON_COLOR.into();

        next_app_state.set(AppState::Game);
      }
      Interaction::Hovered => {
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *background_color = NORMAL_BUTTON_COLOR.into();

      }
    }
  }
}

pub fn interact_with_quit_button(
  mut app_exit_event_writer: EventWriter<AppExit>,
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
  >
) {
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *background_color = PRESSED_BUTTON_COLOR.into();
      
        app_exit_event_writer.send(AppExit);
      }
      Interaction::Hovered => {
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *background_color = NORMAL_BUTTON_COLOR.into();

      }
    }
  }
}