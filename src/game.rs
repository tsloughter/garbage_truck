use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::truck::Truck;
use crate::environment::GarbageCan;
use crate::particles::SparkleEffect;
use crate::ui::Score;
use crate::assets::BackupAlarm;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game)
           .add_systems(Update, (camera_follow, check_collisions));
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // Load backup alarm resource
    let backup_alarm = asset_server.load("embedded://garbage_bevy/../assets/backup_alarm.wav");
    commands.insert_resource(BackupAlarm(backup_alarm));
}

fn camera_follow(
    truck_query: Query<&Transform, With<Truck>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Truck>)>,
) {
    if let Some(truck_transform) = truck_query.iter().next() {
        if let Some(mut camera_transform) = camera_query.iter_mut().next() {
            camera_transform.translation.x = truck_transform.translation.x;
            camera_transform.translation.y = truck_transform.translation.y;
        }
    }
}

pub fn check_collisions(
    mut commands: Commands,
    truck_query: Query<&Transform, With<Truck>>,
    can_query: Query<(Entity, &GlobalTransform), With<GarbageCan>>,
    sparkle_effect: Res<SparkleEffect>,
    mut score: ResMut<Score>,
) {
    let Some(truck_transform) = truck_query.iter().next() else {
        return;
    };

    for (can_entity, can_transform) in can_query.iter() {
        let distance = truck_transform
            .translation
            .truncate()
            .distance(can_transform.translation().truncate());

        if distance < 50.0 {
            // Collision threshold
            // Despawn can
            commands.entity(can_entity).despawn();

            // Increment score
            score.0 += 1;

            // Spawn sparkle effect
            commands.spawn((
                ParticleEffect::new(sparkle_effect.0.clone()),
                Transform::from_translation(can_transform.translation()),
            ));
        }
    }
}
