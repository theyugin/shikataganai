use crate::ecs::resources::chunk_map::{ChunkMap, ChunkMapSize};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ChunkMapSize { x: 5, y: 5 })
      .init_resource::<ChunkMap>();
  }
}
