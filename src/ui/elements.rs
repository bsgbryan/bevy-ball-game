use bevy::{
  prelude::{
    AssetServer,
    ButtonBundle,
    Component,
    Res,
    TextBundle, ImageBundle,
  },
  text::{
    Text,
    TextSection,
    TextAlignment,
  }, ui::{Style, Size, Val}
};

use super::styles::{
  BUTTON_STYLE,
  NORMAL_BUTTON_COLOR,
  text_style,
};

pub fn centered_text(value: &str, size: f32, asset_server: &Res<AssetServer>) -> TextBundle {
  TextBundle {
    text: Text {
      sections: vec![TextSection::new(
        value.to_string(),
        text_style(size, &asset_server),
      )],
      alignment: TextAlignment::Center,
      ..Default::default()
    },
    ..Default::default()
  }
}

pub fn btn<C: Component>() -> (ButtonBundle, C) where C: Default {(
  ButtonBundle {
    style: BUTTON_STYLE,
    background_color: NORMAL_BUTTON_COLOR.into(),
    ..Default::default()
  },
  <C as std::default::Default>::default()
)}

pub fn img(sprite: &str, size: f32, asset_server: &Res<AssetServer>) -> ImageBundle {
  ImageBundle {
    style: Style {
      size: Size::new(
        Val::Px(size),
        Val::Px(size)
      ),
      margin: bevy::ui::UiRect::new(
        Val::Px(8.0),
        Val::Px(8.0),
        Val::Px(8.0),
        Val::Px(8.0)
      ),
      ..Style::DEFAULT
    },
    image: asset_server.
      load(format!("sprites/{}.png", sprite)).
      into(),
    ..Default::default()
  }
}