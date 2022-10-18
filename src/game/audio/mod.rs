
use super::super::AppState;
use super::collisions::check_collisions;
use super::events::*;
use super::constants::*;

//use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*};
pub mod clip;
use clip::*;


pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame))
            .add_event::<AudioEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(on_audio_event)//.after(check_collisions)
            );
    }
}

fn on_audio_event(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut audio_events: EventReader<AudioEvent>
){
    if audio_events.is_empty(){
        return;
    }
    for event in audio_events.iter() {
        play_audio_clip(&audio, &asset_server, event.clip);
    }
}
