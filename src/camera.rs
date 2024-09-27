use crate::CenterOfMass;
use bevy::prelude::*;
use bevy::core_pipeline::{
        bloom::*,
        tonemapping::Tonemapping,
    };

pub fn update_camera_system(
    center_of_mass: Res<CenterOfMass>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new((center_of_mass.0.x / 1e9) as f32, (center_of_mass.0.y / 1e9) as f32, transform.translation.z);
    }
}

pub fn intialize_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings {
            intensity: 0.15,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.0,
                threshold_softness: 0.0,
            },
            composite_mode: BloomCompositeMode::EnergyConserving,
        }
    ));
}