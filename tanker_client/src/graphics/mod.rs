use bevy::{prelude::*, render::mesh::shape::UVSphere};
use bevy_replicon::renet::transport::NetcodeClientTransport;
use tanker_common::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::init_ui_system)
            .add_systems(Startup, Self::init_scenery_system)
            .add_systems(Update, Self::spawn_bomb_explosions_system)
            .add_systems(Update, Self::update_players_system)
            .add_systems(Update, Self::spawn_players_system)
            .add_systems(Update, Self::spawn_bombs_system);
    }
}

impl GraphicsPlugin {
    fn init_ui_system(mut commands: Commands, transport: Res<NetcodeClientTransport>) {
        commands.spawn(TextBundle::from_section(
            format!("Client: {}", transport.client_id()),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // camera
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }

    fn init_scenery_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // circular base
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Circle::new(4.0).into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        });

        // light
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
    }

    fn update_players_system(mut query: Query<(&mut Transform, &PlayerPosition)>) {
        for (mut transform, position) in &mut query {
            *transform = Transform::from_translation(position.0)
        }
    }

    fn spawn_players_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<(Entity, &PlayerColor, &PlayerPosition), Added<PlayerColor>>,
    ) {
        for (entity, color, position) in &query {
            commands.entity(entity).insert(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(color.0.into()),
                transform: Transform::from_translation(position.0),
                ..default()
            });
        }
    }

    fn spawn_bomb_explosions_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<&BombExplosion, Added<BombExplosion>>,
    ) {
        for explosion in &query {
            let mesh = meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: explosion.radius.0,
                    subdivisions: 7,
                })
                .unwrap(),
            );

            let material = materials.add(StandardMaterial {
                base_color: Color::rgba(0.9, 0.2, 0.3, 1.0),
                alpha_mode: AlphaMode::Add,
                ..default()
            });

            commands.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(explosion.position.0),
                ..default()
            });
        }
    }

    fn spawn_bombs_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<(&PlayerColor, &BombPosition), Added<BombPosition>>,
    ) {
        for (color, position) in &query {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(UVSphere {
                    stacks: 64,
                    sectors: 128,
                    radius: 0.5,
                })),
                material: materials.add(color.0.into()),
                transform: Transform::from_translation(position.0),
                ..default()
            });
        }
    }
}
