use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use garbage_bevy::truck::Truck;
use garbage_bevy::environment::GarbageCan;
use garbage_bevy::ui::Score;
use garbage_bevy::particles::SparkleEffect;
use garbage_bevy::game::check_collisions;

#[test]
fn test_truck_can_collision() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<EffectAsset>();

    // Add required resources
    app.insert_resource(Score(0));
    
    let mut effects = app.world_mut().resource_mut::<Assets<EffectAsset>>();
    let effect_handle = effects.add(EffectAsset::default());
    app.insert_resource(SparkleEffect(effect_handle));

    // Add the system under test
    app.add_systems(Update, check_collisions);

    // Spawn truck at origin
    app.world_mut().spawn((
        Truck { direction: 0.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn garbage can within collision range (distance < 50)
    let can_id = app.world_mut().spawn((
        GarbageCan,
        Transform::from_xyz(30.0, 0.0, 0.0),
        GlobalTransform::from_xyz(30.0, 0.0, 0.0),
    )).id();

    // Run systems
    app.update();

    // Verify score incremented
    let score = app.world().resource::<Score>();
    assert_eq!(score.0, 1);

    // Verify garbage can despawned
    assert!(app.world().get_entity(can_id).is_err());

    // Verify particle effect spawned (check for ParticleEffect component)
    let mut particle_query = app.world_mut().query::<&ParticleEffect>();
    assert_eq!(particle_query.iter(app.world()).count(), 1);
}

#[test]
fn test_no_collision_when_far() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<EffectAsset>();

    app.insert_resource(Score(0));
    
    let mut effects = app.world_mut().resource_mut::<Assets<EffectAsset>>();
    let effect_handle = effects.add(EffectAsset::default());
    app.insert_resource(SparkleEffect(effect_handle));

    app.add_systems(Update, check_collisions);

    app.world_mut().spawn((
        Truck { direction: 0.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn garbage can outside collision range (distance > 50)
    let can_id = app.world_mut().spawn((
        GarbageCan,
        Transform::from_xyz(60.0, 0.0, 0.0),
        GlobalTransform::from_xyz(60.0, 0.0, 0.0),
    )).id();

    app.update();

    let score = app.world().resource::<Score>();
    assert_eq!(score.0, 0);

    assert!(app.world().get_entity(can_id).is_ok());
}
