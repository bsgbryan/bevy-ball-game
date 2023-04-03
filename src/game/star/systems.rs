use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;

use super::constants::COUNT;
use super::constants::SIZE;

use super::components::Star;
use super::resources::StarSpawnTimer;

pub fn spawn_stars(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok(window) = window_query.get_single() {
    let half_star_size = SIZE * 0.5;

    for _ in 0..COUNT {
      // TODO: Instead of this, constrain for grid coors,
      //       so enemies don't overlap each other,
      //       or the player
      let mut x = random::<f32>() * window.width();
      let mut y = random::<f32>() * window.height();

      if x < half_star_size {
        x += half_star_size;
      }
      else if x > window.width() - half_star_size {
        x -= half_star_size;
      }

      if y < half_star_size {
        y += half_star_size;
      }
      else if y > window.height() - half_star_size {
        y -= half_star_size;
      }

      commands.spawn((
        SpriteBundle {
          transform: Transform::from_xyz(x, y, 0.0),
          texture: asset_server.load("sprites/star.png"),
          ..default()
        },
        Star { },
      ));
    }
  }
}

pub fn despawn_stars(
  mut commands: Commands,
  stars_query: Query<Entity, With<Star>>,
) {
  for e in stars_query.iter() {
    commands.entity(e).despawn();
  }
}

pub fn update_star_spawn_timer(
  mut star_spawn_timer: ResMut<StarSpawnTimer>,
  time: Res<Time>,
) {
  star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  star_spawn_timer: Res<StarSpawnTimer>,
) {
  if star_spawn_timer.timer.finished() {
    if let Ok(window) = window_query.get_single() {
      let half_star_size = SIZE * 0.5;

      let mut x = random::<f32>() * window.width();
      let mut y = random::<f32>() * window.height();

      if x < half_star_size {
        x += half_star_size;
      }
      else if x > window.width() - half_star_size {
        x -= half_star_size;
      }

      if y < half_star_size {
        y += half_star_size;
      }
      else if y > window.height() - half_star_size {
        y -= half_star_size;
      }

      commands.spawn((
        SpriteBundle {
          transform: Transform::from_xyz(x, y, 0.0),
          texture: asset_server.load("sprites/star.png"),
          ..default()
        },
        Star { },
      ));
    }
  }
}
