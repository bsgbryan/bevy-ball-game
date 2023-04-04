use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::main_menu::styles::{NORMAL_BUTTON_COLOR, BUTTON_STYLE, TITLE_IMAGE_STYLE, text_style};

pub fn spawn_main_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let _e = build_main_menu(&mut commands, &asset_server);
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
  commands.spawn((
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
          // === LEFT IMAGE ===
          parent.spawn(ImageBundle {
            style: TITLE_IMAGE_STYLE,
            image: asset_server.load("sprites/ball_blue_large.png").into(),
            ..default()
          });

          // === TEXT ===
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Bevy Ball Game",
                text_style(64.0, &asset_server)
              )],
              ..default()
            },
            ..default()
          });

          // === RIGHT IMAGE ===
          parent.spawn(ImageBundle {
            style: TITLE_IMAGE_STYLE,
            image: asset_server.load("sprites/ball_red_large.png").into(),
            ..default()
          });
        });

      // === PLAY BUTTON ===
      parent.
        spawn((
          ButtonBundle {
            style: BUTTON_STYLE,
            background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
          },
          PlayButton
        )).
        with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Play",
                text_style(32.0, &asset_server)
              )],
              ..default()
            },
            ..default()
          });
        });

      // === QUIT BUTTON ===
      parent.
        spawn((
          ButtonBundle {
            style: BUTTON_STYLE,
            background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
          },
          QuitButton
        )).
        with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Quit",
                text_style(32.0, &asset_server)
              )],
              ..default()
            },
            ..default()
          });
        });
    }).
    id() 
}
