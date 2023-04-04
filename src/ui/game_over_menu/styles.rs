use bevy::prelude::*;

pub const GAME_OVER_MENU_STYLE: Style = Style {
  position_type: PositionType::Absolute, // Needed to display separately from HUD.
  display: Display::Flex,                // Hidden by Default
  justify_content: JustifyContent::Center,
  align_items: AlignItems::Center,
  size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
  ..Style::DEFAULT
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = Style {
  display: Display::Flex,
  flex_direction: FlexDirection::Column,
  justify_content: JustifyContent::Center,
  align_items: AlignItems::Center,
  size: Size::new(Val::Px(400.0), Val::Px(400.0)),
  gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
  ..Style::DEFAULT
};