use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::constants::SIZE;
use super::constants::SPEED;

use super::components::Player;

pub fn spawn_player(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok(window) = window_query.get_single() {
    commands.spawn((
      SpriteBundle {
        transform: Transform::from_xyz(
          window.width() * 0.5,
          window.height() * 0.5,
          0.0
        ),
        texture: asset_server.load("sprites/ball_blue_large.png"),
        ..default()
      },
      Player { },
    ));
  }
}

pub fn move_player(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<&mut Transform, With<Player>>,
  time: Res<Time>,
) {
  if let Ok(mut transform) = player_query.get_single_mut() {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
      direction += Vec3::new(-1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
      direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
      direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
      direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
      direction = direction.normalize();
    }

    transform.translation += direction * SPEED * time.delta_seconds();
  }
}

pub fn constrain_player_movement(
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(mut player_transform) = player_query.get_single_mut() {
    if let Ok(window) = window_query.get_single() {
      let half_player_size = SIZE * 0.5;
      
      let min_x = 0.0 + half_player_size;
      let min_y = 0.0 + half_player_size;
      let max_x = window.width()  - half_player_size;
      let max_y = window.height() - half_player_size;

      let mut translation = player_transform.translation;

      if translation.x < min_x {
        translation.x = min_x;
      }
      else if translation.x > max_x {
        translation.x = max_x;
      }

      if translation.y < min_y {
        translation.y = min_y;
      }
      else if translation.y > max_y {
        translation.y = max_y;
      }

      player_transform.translation = translation;
    }
  }
}
