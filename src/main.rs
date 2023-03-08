use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        render_resource::WgpuFeatures, settings::WgpuSettings,
        view::RenderLayers,
    },
};
use rand::prelude::*;

use bevy_hanabi::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = WgpuSettings::default();
    options
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::default()
        .insert_resource(options)
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::WARN,
            filter: "bevy_hanabi=warn,spawn=trace".to_string(),
        }))
        .add_system(bevy::window::close_on_esc)
        .add_plugin(HanabiPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .add_system(spawn)
        .add_system(remove)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
) {
    let mut camera = Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    };
    camera.transform.translation = Vec3::new(0.0, 0.0, 100.0);

    commands.spawn((camera, RenderLayers::layer(3)));
}


fn spawn(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("cloud.png");

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::splat(1.0));
    gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::splat(0.0));

    for _i in 0..10 {
        let effect = effects.add(
            EffectAsset {
                name: "Gradient".to_string(),
                capacity: 32768,
                spawner: Spawner::rate(1000.0.into()),
                ..Default::default()
            }
            .init(PositionSphereModifier {
                center: Vec3::ZERO,
                radius: 1.,
                dimension: ShapeDimension::Volume,
                speed: 2.0.into(),
            })
            .init(ParticleLifetimeModifier {
                lifetime: 5_f32.into(),
            })
            .render(ParticleTextureModifier {
                texture: texture_handle.clone(),
            })
            .render(ColorOverLifetimeModifier { gradient: gradient.clone() }),
        );

        let x = thread_rng().gen_range(-50.0f32..50.0);

        commands
            .spawn((
                Name::new("effect"),
                ParticleEffectBundle::new(effect),
                RenderLayers::layer(3),
            ))
            .insert(Transform::from_translation(Vec3::new(x as f32, 30., 0.)));
    }
}

fn update(
    time: Res<Time>, 
    mut query: Query<&mut Transform, With<ParticleEffect>>
) {
    for mut transform in query.iter_mut() {
        transform.translation = transform.translation - 10. * Vec3::Y * time.delta_seconds();
    }
}


fn remove(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<ParticleEffect>>
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -25. {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            } 
        }
    }
}
