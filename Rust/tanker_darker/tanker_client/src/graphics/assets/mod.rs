use bevy::{prelude::*, render::mesh::shape::UVSphere};

#[derive(Resource)]
pub struct AssetsResources {
    pub player_mesh: Handle<Mesh>,

    pub explosion_mesh: Handle<Mesh>,
    pub explosion_material: Handle<StandardMaterial>,

    pub bomb_mesh: Handle<Mesh>,
}

pub struct AssetsPlugin;

impl AssetsPlugin {
    fn init_assets_resources_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let player_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

        let explosion_mesh = meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 7,
            })
            .unwrap(),
        );

        let explosion_material = materials.add(StandardMaterial {
            base_color: Color::rgba(0.9, 0.2, 0.3, 1.0),
            alpha_mode: AlphaMode::Add,
            ..default()
        });

        let bomb_mesh = meshes.add(Mesh::from(UVSphere {
            stacks: 64,
            sectors: 128,
            radius: 0.5,
        }));

        commands.insert_resource(AssetsResources {
            player_mesh,
            explosion_mesh,
            explosion_material,
            bomb_mesh,
        })
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::init_assets_resources_system);
    }
}
