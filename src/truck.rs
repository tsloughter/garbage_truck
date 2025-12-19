use bevy::prelude::*;
use crate::assets::BackupAlarm;

pub const TRUCK_SPEED: f32 = 200.0;
pub const ROTATION_SPEED: f32 = 3.0;

#[derive(Component)]
pub struct Truck {
    pub direction: f32, // Angle in radians (0 = Up, increasing clockwise)
}

#[derive(Component)]
pub struct BackupAudioController;

pub struct TruckPlugin;

impl Plugin for TruckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_truck)
           .add_systems(Update, truck_movement);
    }
}

fn spawn_truck(mut commands: Commands, asset_server: Res<AssetServer>) {
    let truck_texture = asset_server.load("embedded://garbage_bevy/../assets/top_down_garbage_truck.png");
    
    commands.spawn((
        Sprite::from_image(truck_texture),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(0.25), // Scale down to 25%
            ..default()
        },
        Truck { direction: 0.0 },
    ));
}

fn truck_movement(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Truck)>,
    mut exit: MessageWriter<AppExit>,
    backup_alarm: Res<BackupAlarm>,
    audio_controller: Query<Entity, With<BackupAudioController>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    for (mut transform, mut truck) in query.iter_mut() {
        let mut rotation = 0.0;
        let mut acceleration = 0.0;

        // Rotation
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            rotation -= ROTATION_SPEED;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            rotation += ROTATION_SPEED;
        }

        // Forward/backward
        let mut reversing = false;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            acceleration += TRUCK_SPEED;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            acceleration -= TRUCK_SPEED * 0.5; // Slower in reverse
            reversing = true;
        }

        // Backup alarm logic
        if reversing {
            if audio_controller.is_empty() {
                commands.spawn((
                    AudioPlayer(backup_alarm.0.clone()),
                    PlaybackSettings::LOOP,
                    BackupAudioController,
                ));
            }
        } else {
            for entity in audio_controller.iter() {
                commands.entity(entity).despawn();
            }
        }

        // Update direction
        truck.direction += rotation * time.delta_secs();

        // Calculate movement based on direction
        let direction_vec = Vec2::new(truck.direction.sin(), truck.direction.cos());
        let movement = direction_vec * acceleration * time.delta_secs();

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // Apply rotation to sprite
        transform.rotation = Quat::from_rotation_z(-truck.direction);
    }
}
