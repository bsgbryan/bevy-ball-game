use bevy::{
  prelude::Mut,
  ui::{
    Interaction,
    BackgroundColor
  },
};

use super::styles::{
  HOVERED_BUTTON_COLOR,
  NORMAL_BUTTON_COLOR,
  PRESSED_BUTTON_COLOR,
};

pub fn handle<F>(
  interaction: &Interaction,
  mut color: Mut<BackgroundColor>,
  delegate: F
) where F: FnOnce() {
  match *interaction {
    Interaction::Clicked => {
      *color = PRESSED_BUTTON_COLOR.into();

      delegate();
    }
    Interaction::Hovered => {
      *color = HOVERED_BUTTON_COLOR.into();
    }
    Interaction::None => {
      *color = NORMAL_BUTTON_COLOR.into();
    }
  }
}