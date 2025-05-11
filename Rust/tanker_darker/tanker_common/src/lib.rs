use bevy::{
    prelude::{Bundle, Color, Component, Deref, DerefMut, Vec3},
    time::Timer,
};
use bevy_replicon::{prelude::Replication, renet::ClientId};
use serde::{Deserialize, Serialize};

pub mod events;

pub const PORT: u16 = 42069;
pub const PROTOCOL_ID: u64 = 0;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: PlayerPosition,
    color: PlayerColor,
    replication: Replication,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, position: Vec3, color: Color) -> Self {
        Self {
            player: Player(client_id),
            position: PlayerPosition(position),
            color: PlayerColor(color),
            replication: Replication,
        }
    }
}

/// Contains the client ID of the player.
#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub ClientId);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct PlayerPosition(pub Vec3);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);

#[derive(Component)]
pub struct BombControl(pub Timer);

#[derive(Component, Deserialize, Serialize)]
pub struct BombPosition(pub Vec3);

#[derive(Component)]
pub struct BombFuse(pub Timer);

#[derive(Component, Serialize, Deserialize)]
pub struct BombRadius(pub f32);

#[derive(Component, Serialize, Deserialize)]
pub struct BombExplosion {
    pub position: BombPosition,
    pub radius: BombRadius,
}

#[derive(Bundle)]
pub struct BombExplosionBundle {
    pub explosion: BombExplosion,
    replication: Replication,
}

impl BombExplosionBundle {
    pub fn new(position: Vec3, radius: f32) -> Self {
        Self {
            explosion: BombExplosion {
                position: BombPosition(position),
                radius: BombRadius(radius),
            },
            replication: Replication,
        }
    }
}

#[derive(Bundle)]
pub struct BombBundle {
    position: BombPosition,
    color: PlayerColor,
    replication: Replication,
}

impl BombBundle {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            color: PlayerColor(color),
            position: BombPosition(position),
            replication: Replication,
        }
    }
}
