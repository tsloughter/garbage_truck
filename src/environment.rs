use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Background {
    pub grid_x: i32,
    pub grid_y: i32,
}

#[derive(Component)]
pub struct GarbageCan;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment)
           .add_systems(Update, update_background);
    }
}

fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bg_texture = asset_server.load("embedded://garbage_bevy/../assets/parking_lot.png");
    let can_texture = asset_server.load("embedded://garbage_bevy/../assets/full_garbage_bin.png");

    // Spawn background grid (3x3)
    let initial_size = 1000.0; // Approximate size, will be corrected in update
    for x in -1..=1 {
        for y in -1..=1 {
            commands
                .spawn((
                    Sprite::from_image(bg_texture.clone()),
                    Transform::from_xyz(
                        x as f32 * initial_size,
                        y as f32 * initial_size,
                        -10.0, // Behind everything
                    ),
                    Background {
                        grid_x: x,
                        grid_y: y,
                    },
                ))
                .with_children(|parent| {
                    // Spawn random garbage cans
                    let mut rng = rand::rng();
                    let num_cans = rng.random_range(3..8);
                    for _ in 0..num_cans {
                        let cx = rng.random_range(-400.0..400.0);
                        let cy = rng.random_range(-400.0..400.0);
                        parent.spawn((
                            Sprite::from_image(can_texture.clone()),
                            Transform {
                                translation: Vec3::new(cx, cy, 1.0), // Above background
                                scale: Vec3::splat(0.15),            // Scale down cans
                                ..default()
                            },
                            GarbageCan,
                        ));
                    }
                });
        }
    }
}

fn update_background(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    mut background_query: Query<
        (
            Entity,
            &mut Transform,
            &Sprite,
            &mut Background,
            Option<&Children>,
        ),
        Without<Camera>,
    >,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    let Some(camera_transform) = camera_query.iter().next() else {
        return;
    };

    let can_texture = asset_server.load("embedded://garbage_bevy/../assets/full_garbage_bin.png");

    for (entity, mut bg_transform, sprite, mut bg_info, children) in background_query.iter_mut() {
        let Some(image) = images.get(&sprite.image) else {
            continue;
        };

        let width = image.width() as f32;
        let height = image.height() as f32;

        let current_grid_x = (bg_transform.translation.x / width).round();
        let current_grid_y = (bg_transform.translation.y / height).round();

        // Snap to grid
        bg_transform.translation.x = current_grid_x * width;
        bg_transform.translation.y = current_grid_y * height;

        // Wrap logic
        let diff_x = bg_transform.translation.x - camera_transform.translation.x;
        let diff_y = bg_transform.translation.y - camera_transform.translation.y;

        let mut wrapped = false;

        if diff_x < -1.5 * width {
            bg_transform.translation.x += 3.0 * width;
            bg_info.grid_x += 3;
            wrapped = true;
        } else if diff_x > 1.5 * width {
            bg_transform.translation.x -= 3.0 * width;
            bg_info.grid_x -= 3;
            wrapped = true;
        }

        if diff_y < -1.5 * height {
            bg_transform.translation.y += 3.0 * height;
            bg_info.grid_y += 3;
            wrapped = true;
        } else if diff_y > 1.5 * height {
            bg_transform.translation.y -= 3.0 * height;
            bg_info.grid_y -= 3;
            wrapped = true;
        }

        if wrapped {
            // Despawn old children
            if let Some(children) = children {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }

            // Spawn new cans
            let mut rng = rand::rng();
            let num_cans = rng.random_range(3..8);

            commands.entity(entity).with_children(|parent| {
                for _ in 0..num_cans {
                    let cx = rng.random_range(-width / 2.0 + 50.0..width / 2.0 - 50.0);
                    let cy = rng.random_range(-height / 2.0 + 50.0..height / 2.0 - 50.0);
                    parent.spawn((
                        Sprite::from_image(can_texture.clone()),
                        Transform {
                            translation: Vec3::new(cx, cy, 1.0),
                            scale: Vec3::splat(0.15),
                            ..default()
                        },
                        GarbageCan,
                    ));
                }
            });
        }
    }
}
