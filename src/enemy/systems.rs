use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;

use super::constants::COUNT;
use super::constants::SIZE;
use super::constants::SPEED;

use super::components::Enemy;
use super::resources::EnemySpawnTimer;

pub fn spawn_enemies(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok(window) = window_query.get_single() {
    let half_enemy_size = SIZE * 0.5;

    for _ in 0..COUNT {
      // TODO: Instead of this, constrain for grid coors,
      //       so enemies don't overlap each other,
      //       or the player
      let mut x = random::<f32>() * window.width();
      let mut y = random::<f32>() * window.height();

      if x < half_enemy_size {
        x += half_enemy_size;
      }
      else if x > window.width() - half_enemy_size {
        x -= half_enemy_size;
      }

      if y < half_enemy_size {
        y += half_enemy_size;
      }
      else if y > window.height() - half_enemy_size {
        y -= half_enemy_size;
      }

      commands.spawn((
        SpriteBundle {
          transform: Transform::from_xyz(x, y, 0.0),
          texture: asset_server.load("sprites/ball_red_large.png"),
          ..default()
        },
        Enemy {
          direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
      ));
    }
  }
}

pub fn move_enemy(
  mut enemy_query: Query<(&mut Transform, &Enemy)>,
  time: Res<Time>,
) {
  for (mut transform, enemy) in enemy_query.iter_mut() {
    let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

    transform.translation += direction * SPEED * time.delta_seconds();
  }
}

pub fn bounce_enemy_off_screen_border(
  mut enemy_query: Query<(&Transform, &mut Enemy)>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>,
) {
  if let Ok(window) = window_query.get_single() {
    let half_enemy_size = SIZE * 0.5;
      
    let min_x = 0.0 + half_enemy_size;
    let min_y = 0.0 + half_enemy_size;
    let max_x = window.width()  - half_enemy_size;
    let max_y = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
      let mut bounced = false;
      
      let translation = transform.translation;
  
      if translation.x < min_x || translation.x > max_x {
        enemy.direction.x *= -1.0;

        bounced = true;
      }

      if translation.y < min_y || translation.y > max_y {
        enemy.direction.y *= -1.0;

        bounced = true;
      }

      if bounced {
        let effect =  if random::<f32>() > 0.5 {
          asset_server.load("audio/pluck_001.ogg")
        }
        else {
          asset_server.load("audio/pluck_002.ogg")
        };

        audio.play_with_settings(
          effect,
          PlaybackSettings {
            volume: 0.5,
            ..default()
          }
        );
      }
    }
  }
}

pub fn constrain_enemy_movement(
  mut enemy_query: Query<&mut Transform, With<Enemy>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(window) = window_query.get_single() {
    let half_enemy_size = SIZE * 0.5;
    
    let min_x = 0.0 + half_enemy_size;
    let min_y = 0.0 + half_enemy_size;
    let max_x = window.width()  - half_enemy_size;
    let max_y = window.height() - half_enemy_size;
    
    for mut enemy_transform in enemy_query.iter_mut() {
      let mut translation = enemy_transform.translation;

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

      enemy_transform.translation = translation;
    }
  }
}

pub fn update_enemy_spawn_timer(
  mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
  time: Res<Time>,
) {
  enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
  if enemy_spawn_timer.timer.finished() {
    if let Ok(window) = window_query.get_single() {
      let half_enemy_size = SIZE * 0.5;

      let mut x = random::<f32>() * window.width();
      let mut y = random::<f32>() * window.height();

      if x < half_enemy_size {
        x += half_enemy_size;
      }
      else if x > window.width() - half_enemy_size {
        x -= half_enemy_size;
      }

      if y < half_enemy_size {
        y += half_enemy_size;
      }
      else if y > window.height() - half_enemy_size {
        y -= half_enemy_size;
      }

      commands.spawn((
        SpriteBundle {
          transform: Transform::from_xyz(x, y, 0.0),
          texture: asset_server.load("sprites/ball_red_large.png"),
          ..default()
        },
        Enemy {
          direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
      ));
    }
  }
}
