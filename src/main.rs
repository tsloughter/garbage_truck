use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_hanabi::prelude::*;
use bevy_hanabi::{Gradient, SpawnerSettings};
use rand::Rng;

const TRUCK_SPEED: f32 = 200.0;
const ROTATION_SPEED: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Garbage Truck Game".into(),
                    resolution: WindowResolution::new(1024, 768),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetsPlugin,
        ))
        .add_plugins(HanabiPlugin)
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                truck_movement,
                camera_follow,
                update_background,
                check_collisions,
                update_score_ui,
            ),
        )
        .run();
}

#[derive(Component)]
struct Truck {
    direction: f32, // Angle in radians (0 = Up, increasing clockwise)
}

#[derive(Component)]
struct Background {
    grid_x: i32,
    grid_y: i32,
}

#[derive(Component)]
struct GarbageCan;

#[derive(Resource)]
struct SparkleEffect(Handle<EffectAsset>);

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Component)]
struct ScoreText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // Load assets
    let truck_texture =
        asset_server.load("embedded://garbage_bevy/../assets/top_down_garbage_truck.png");
    let bg_texture = asset_server.load("embedded://garbage_bevy/../assets/parking_lot.png");
    let can_texture = asset_server.load("embedded://garbage_bevy/../assets/full_garbage_bin.png");

    // Define Sparkle Effect
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.0, 1.0)); // Yellow
    color_gradient.add_key(0.5, Vec4::new(1.0, 1.0, 1.0, 1.0)); // White
    color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 0.0, 0.0)); // Transparent

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.5).uniform(writer.lit(1.0)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(20.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(100.).uniform(writer.lit(200.)).expr(),
    };

    let spawner = SpawnerSettings::once(30.0.into());
    let effect = EffectAsset::new(32768, spawner, writer.finish())
        .with_name("sparkle")
        .init(init_age)
        .init(init_lifetime)
        .init(init_pos)
        .init(init_vel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec3::splat(5.0)),
            screen_space_size: false,
        });

    let effect_handle = effects.add(effect);
    commands.insert_resource(SparkleEffect(effect_handle));

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

    // Spawn the truck
    commands.spawn((
        Sprite::from_image(truck_texture),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(0.25), // Scale down to 25%
            ..default()
        },
        Truck { direction: 0.0 },
    ));

    // Instructions text
    commands.spawn((
        Text::new("Arrow Keys or WASD to drive\nESC to quit"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        bevy::ui::Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // Score text
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        bevy::ui::Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));
}

fn check_collisions(
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

fn truck_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Truck)>,
    mut exit: MessageWriter<AppExit>,
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
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            acceleration += TRUCK_SPEED;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            acceleration -= TRUCK_SPEED * 0.5; // Slower in reverse
        }

        // Update direction
        truck.direction += rotation * time.delta_secs();

        // Calculate movement based on direction
        // Direction 0 is UP (0, 1)
        // Rotation is clockwise from North
        // sin(theta) gives X component (Right), cos(theta) gives Y component (Up)
        let direction_vec = Vec2::new(truck.direction.sin(), truck.direction.cos());
        let movement = direction_vec * acceleration * time.delta_secs();

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // Apply rotation to sprite
        // Bevy uses counter-clockwise rotation for Z-axis
        // Our direction is clockwise from North, so we negate it
        transform.rotation = Quat::from_rotation_z(-truck.direction);
    }
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

fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.0 = format!("Score: {}", score.0);
    }
}

pub struct EmbeddedAssetsPlugin;

impl Plugin for EmbeddedAssetsPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "../assets/top_down_garbage_truck.png");
        bevy::asset::embedded_asset!(app, "../assets/parking_lot.png");
        bevy::asset::embedded_asset!(app, "../assets/full_garbage_bin.png");
    }
}
