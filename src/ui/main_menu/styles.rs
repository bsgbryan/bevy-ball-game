use bevy::prelude::*;

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
