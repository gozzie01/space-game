use crate::{Position, Tracking, Identifier};
use bevy::prelude::*;
use bevy::core_pipeline::{
        bloom::*,
        tonemapping::Tonemapping,
    };

pub fn update_camera_system(
    tracker: Res<Tracking>,
    mut positions: Query<(&Position, &Identifier)>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {

        if tracker.0 != 0 {
            //check if the entity is in the query 1 by 1
            for (position, id) in positions.iter() {
                if id.0 == tracker.0 {
                    transform.translation = Vec3::new((position.0.x / 1e9) as f32, (position.0.y / 1e9) as f32, 0.0);
                }
            }
        }
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
        /*BloomSettings {
            intensity: 0.015,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.0,
                threshold_softness: 0.0,
            },
            composite_mode: BloomCompositeMode::EnergyConserving,
        }*/
    ));
}