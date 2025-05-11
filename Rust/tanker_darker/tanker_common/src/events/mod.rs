use bevy::{
    app::App,
    prelude::{Event, Vec3},
};
use bevy_replicon::{
    network_event::{client_event::ClientEventAppExt, EventType},
    replicon_core::replication_rules::AppReplicationExt,
};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec3);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct SpawnBomb;

pub trait Register {
    fn register_events(&mut self) -> &mut Self;
    fn register_replications(&mut self) -> &mut Self;
}

impl Register for App {
    fn register_events(&mut self) -> &mut Self {
        self.add_client_event::<MoveDirection>(EventType::Ordered)
            .add_client_event::<SpawnBomb>(EventType::Ordered)
    }

    fn register_replications(&mut self) -> &mut Self {
        self.replicate::<PlayerPosition>()
            .replicate::<PlayerColor>()
            .replicate::<BombPosition>()
            .replicate::<BombExplosion>()
    }
}
