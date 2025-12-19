use bevy::prelude::*;

#[derive(Resource)]
pub struct BackupAlarm(pub Handle<AudioSource>);

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "../assets/top_down_garbage_truck.png");
        bevy::asset::embedded_asset!(app, "../assets/parking_lot.png");
        bevy::asset::embedded_asset!(app, "../assets/full_garbage_bin.png");
        bevy::asset::embedded_asset!(app, "../assets/backup_alarm.wav");
    }
}
