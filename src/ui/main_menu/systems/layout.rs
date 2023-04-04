use bevy::prelude::*;

use crate::ui::{
  components::QuitButton,
  elements::{
    btn,
    centered_text,
    img,
  },
  main_menu::components::{
    MainMenu,
    PlayButton
  }
};

pub fn spawn_main_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
  mut commands: Commands,
  main_menu_query: Query<Entity, With<MainMenu>>,
) {
  if let Ok(e) = main_menu_query.get_single() {
    commands.entity(e).despawn_recursive();
  }
}

pub fn build_main_menu(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
) -> Entity {
  commands.
    spawn((
      NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          flex_direction: FlexDirection::Column,
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
          ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
      },
      MainMenu
    )).
    with_children(|parent| {
      // === TITLE ===
      parent.
        spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(300.0), Val::Px(120.0)),
            ..default()
          },
          ..default()
        }).
        with_children(|parent| {
          parent.spawn(img("ball_blue_large", 64.0, &asset_server));
          parent.spawn(centered_text("Bevy Ball Game", 64.0, &asset_server));
          parent.spawn(img("ball_red_large", 64.0, &asset_server));
        });

      parent.
        spawn(btn::<PlayButton>()).
        with_children(|parent| {
          parent.spawn(centered_text("Play", 32.0, &asset_server));
        });

      parent.
        spawn(btn::<QuitButton>()).
        with_children(|parent| {
          parent.spawn(centered_text("Quit", 32.0, &asset_server));
        });
    }).
    id() 
}
