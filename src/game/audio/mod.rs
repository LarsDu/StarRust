use super::super::AppState;
use super::events::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioClipAssets {
    pub no_sound: Handle<AudioSource>,
    pub laser_shot: Handle<AudioSource>,
    pub laser_shot_silenced: Handle<AudioSource>,
    pub light_pow: Handle<AudioSource>,
    pub heavy_pow: Handle<AudioSource>,
    pub light_explosion: Handle<AudioSource>,
    pub collection1: Handle<AudioSource>,
    pub point_counter: Handle<AudioSource>,
    pub salt_explosion: Handle<AudioSource>,
    pub sputter_rocket: Handle<AudioSource>,
    pub coin_larry: Handle<AudioSource>,
    pub event_slam: Handle<AudioSource>,
}
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .add_event::<AudioEvent>()
            .add_system(on_audio_event);
    }
}
pub fn setup_resources(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let audio_clip_assets = AudioClipAssets {
        no_sound: asset_server.load(""),
        laser_shot: asset_server.load("audio/clips/laser_shot.ogg"),
        laser_shot_silenced: asset_server.load("audio/clips/laser_shot_silenced.ogg"),
        light_pow: asset_server.load("audio/clips/light_pow.ogg"),
        heavy_pow: asset_server.load("audio/clips/heavy_pow.ogg"),
        light_explosion: asset_server.load("audio/clips/light_explosion.ogg"),
        collection1: asset_server.load("audio/clips/collection1.ogg"),
        point_counter: asset_server.load("audio/clips/point_counter.ogg"),
        salt_explosion: asset_server.load("audio/clips/salt_explosion.ogg"),
        sputter_rocket: asset_server.load("audio/clips/sputter_rocket.ogg"),
        coin_larry: asset_server.load("audio/clips/coin_larry.ogg"),
        event_slam: asset_server.load("audio/clips/event_slam.ogg"),
    };
    commands.insert_resource(audio_clip_assets);
}
fn on_audio_event(mut commands: Commands, asset_server: Res<AssetServer>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }
    //for event in audio_events.iter() {
    //    audio.play(event.clip.clone());
    //}
    for event in audio_events.iter(){
        commands.spawn((AudioBundle{
            source: asset_server.load(event.audioFile),
            settings: PlaybackSettings { mode: PlaybackSettings::Once, ..default()}
        }))
    }
}
