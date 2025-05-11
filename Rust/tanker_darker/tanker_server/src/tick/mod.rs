use std::time::Duration;

use bevy::prelude::*;

use bevy_replicon::{prelude::*, renet::ServerEvent};
use tanker_common::{
    events::{MoveDirection, SpawnBomb},
    *,
};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Self::tick_bomb_control_timers_system)
            .add_systems(FixedUpdate, Self::tick_bomb_fuse_timers_system)
            .add_systems(FixedUpdate, Self::blow_bombs_system)
            .add_systems(FixedUpdate, Self::movement_system)
            .add_systems(FixedUpdate, Self::server_event_system)
            .add_systems(FixedUpdate, Self::spawn_bombs_system);
    }
}

impl TickPlugin {
    fn tick_bomb_control_timers_system(mut query: Query<&mut BombControl>, time: Res<Time>) {
        for mut timer in &mut query {
            timer.0.tick(time.delta());
        }
    }

    fn tick_bomb_fuse_timers_system(mut query: Query<&mut BombFuse>, time: Res<Time>) {
        for mut timer in &mut query {
            timer.0.tick(time.delta());
        }
    }

    /// Logs server events and spawns a new player whenever a client connects.
    fn server_event_system(
        mut commands: Commands,
        mut server_event: EventReader<ServerEvent>,
        query: Query<(Entity, &Player)>,
    ) {
        for event in server_event.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("Player: {client_id} Connected");
                    // Generate pseudo random color from client id.
                    let r = ((client_id.raw() % 23) as f32) / 23.0;
                    let g = ((client_id.raw() % 27) as f32) / 27.0;
                    let b = ((client_id.raw() % 39) as f32) / 39.0;

                    let color = Color::rgb(r, g, b);

                    let player = PlayerBundle::new(*client_id, Vec3::new(0., 0.5, 0.), color);

                    let timer = Timer::new(Duration::from_secs(0), TimerMode::Once);

                    commands.spawn((player, BombControl(timer)));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("Player: {client_id} disconnected: {reason}");

                    for (entity, player) in &query {
                        if *client_id == player.0 {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }

    /// Mutates [`PlayerPosition`] based on [`MoveDirection`].
    ///
    /// Fast-paced games usually you don't want to wait until server send a position back because of the latency.
    /// But this example just demonstrates simple replication concept.
    fn movement_system(
        time: Res<Time>,
        mut events: EventReader<FromClient<MoveDirection>>,
        mut query: Query<(&Player, &mut PlayerPosition)>,
    ) {
        const MOVE_SPEED: f32 = 3.0;

        for FromClient { client_id, event } in events.read() {
            let (_, mut position) = query.iter_mut().find(|(p, _)| p.0 == *client_id).unwrap();

            **position += event.0 * time.delta_seconds() * MOVE_SPEED;
        }
    }

    fn blow_bombs_system(mut commands: Commands, query: Query<(Entity, &BombFuse, &BombPosition)>) {
        const BOMB_RADIUS: f32 = 3.0;

        for (entity, _, position) in query.iter().filter(|(_, f, _)| f.0.finished()) {
            commands.entity(entity).despawn();

            commands.spawn(BombExplosionBundle::new(position.0, BOMB_RADIUS));
        }
    }

    fn spawn_bombs_system(
        mut commands: Commands,
        mut events: EventReader<FromClient<SpawnBomb>>,
        mut query: Query<(&Player, &PlayerPosition, &PlayerColor, &mut BombControl)>,
    ) {
        for FromClient {
            client_id,
            event: _,
        } in events.read()
        {
            let (player, position, color, mut control) = query
                .iter_mut()
                .find(|(p, _, _, _)| p.0 == *client_id)
                .unwrap();

            if !control.0.finished() {
                return;
            }

            const BOMB_DELAY: u64 = 3000;
            const BOMB_FUSE_DELAY: f32 = 3.0;

            control.0.set_duration(Duration::from_millis(BOMB_DELAY));
            control.0.reset();

            let bomb = BombBundle::new(position.0, color.0);

            let timer = BombFuse(Timer::from_seconds(BOMB_FUSE_DELAY, TimerMode::Once));

            commands.spawn((bomb, timer));

            info!("Player: {} Spawned bomb at {}", player.0, position.0);
        }
    }
}
