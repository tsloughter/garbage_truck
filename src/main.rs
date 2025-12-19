use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_hanabi::prelude::*;

use garbage_bevy::assets::AssetsPlugin;
use garbage_bevy::particles::ParticlesPlugin;
use garbage_bevy::truck::TruckPlugin;
use garbage_bevy::environment::EnvironmentPlugin;
use garbage_bevy::ui::UiPlugin;
use garbage_bevy::game::GamePlugin;

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
            AssetsPlugin,
        ))
        .add_plugins(HanabiPlugin)
        .add_plugins((
            ParticlesPlugin,
            TruckPlugin,
            EnvironmentPlugin,
            UiPlugin,
            GamePlugin,
        ))
        .run();
}
