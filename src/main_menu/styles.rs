use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 1.0, 0.35);

pub const BUTTON_STYLE: Style = Style {
  size: Size::new(Val::Px(200.0), Val::Px(80.0)),
  justify_content: JustifyContent::Center,
  align_items: AlignItems::Center,
  ..Style::DEFAULT
};

pub const TITLE_IMAGE_STYLE: Style = Style {
  size: Size::new(
    Val::Px(64.0),
    Val::Px(64.0)
  ),
  margin: UiRect::new(
    Val::Px(8.0),
    Val::Px(8.0),
    Val::Px(8.0),
    Val::Px(8.0)
  ),
  ..Style::DEFAULT
};

pub fn text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size,
    color: Color::WHITE,
  }
}