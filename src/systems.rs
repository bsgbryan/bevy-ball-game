use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;

use crate::player::constants::SIZE as PLAYER_SIZE;
use crate::enemy::constants::SIZE  as ENEMY_SIZE;
use crate::star::constants::SIZE   as STAR_SIZE;

use crate::player::components::Player;
use crate::enemy::components::Enemy;
use crate::score::resources::Score;
use crate::star::components::Star;

pub fn spawn_camera(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(window) = window_query.get_single() {
    commands.spawn(Camera2dBundle {
      transform: Transform::from_xyz(
        window.width() * 0.5,
        window.height() * 0.5,
        0.0
      ),
      ..default()
    });
  }
}

pub fn detect_collision(
  mut commands: Commands,
  mut game_over_event_writer: EventWriter<GameOver>,
  mut player_query: Query<(Entity, &Transform), With<Player>>,
  enemy_query: Query<&Transform, With<Enemy>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>,
  score: Res<Score>,
) {
  if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
    for enemy_transform in enemy_query.iter() {
      let distance = player_transform.
      translation.
      distance(enemy_transform.translation);

      let player_radius = PLAYER_SIZE * 0.5;
      let enemy_radius  = ENEMY_SIZE  * 0.5;

      if distance < player_radius + enemy_radius {
        let effect = asset_server.
          load("audio/explosionCrunch_000.ogg");

        audio.play_with_settings(
          effect,
          PlaybackSettings {
            volume: 0.5,
            ..default()
          }
        );

        commands.
          entity(player_entity).
          despawn();

        game_over_event_writer.send(GameOver { score: score.value });
      }
    }
  }
}

pub fn detect_pickup(
  mut commands: Commands,
  player_query: Query<&Transform, With<Player>>,
  star_query: Query<(Entity, &Transform), With<Star>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>,
  mut score: ResMut<Score>,
) {
  if let Ok(player_transform) = player_query.get_single() {
    for (star_entity, star_transform) in star_query.iter() {
      let distance = player_transform.
      translation.
      distance(star_transform.translation);

      let player_radius = PLAYER_SIZE * 0.5;
      let star_radius   = STAR_SIZE   * 0.5;

      if distance < player_radius + star_radius {
        let effect = asset_server.
          load("audio/impactGeneric_light_004.ogg");

        audio.play_with_settings(
          effect,
          PlaybackSettings {
            volume: 0.5,
            ..default()
          }
        );

        commands.
          entity(star_entity).
          despawn();

        score.value += 50;
      }
    }
  }
}

pub fn exit_game(
  keyboard_input: Res<Input<KeyCode>>,
  mut app_exit_event_writer: EventWriter<AppExit>,
) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    app_exit_event_writer.send(AppExit);
  }
}

pub fn handle_game_over(
  mut game_over_event_reader: EventReader<GameOver>
) {
  for event in game_over_event_reader.iter() {
    println!("Game over! Final score: {}", event.score.to_string());
  }
}
