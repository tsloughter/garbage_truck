use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource)]
pub struct SparkleEffect(pub Handle<EffectAsset>);

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_particles);
    }
}

fn setup_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient = bevy_hanabi::Gradient::new();
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
            gradient: bevy_hanabi::Gradient::constant(Vec3::splat(5.0)),
            screen_space_size: false,
        });

    let effect_handle = effects.add(effect);
    commands.insert_resource(SparkleEffect(effect_handle));
}
