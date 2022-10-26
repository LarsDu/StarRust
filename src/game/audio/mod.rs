use super::super::AppState;
use super::events::*;
use bevy::prelude::*;


pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioEvent>().add_system(on_audio_event);
    }
}

fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play(event.clip.clone());
    }
}
