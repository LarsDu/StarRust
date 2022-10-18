use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum AudioClipEnum{
    NoSound,
    LaserShotSilenced,
    LightPow,
    LightExplosion,
    Collection1,
    PointCounter,
    SaltExplosion,
    SputterRocket,
    CoinLarry,
    EventSlam
}

pub fn play_audio_clip(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
    clips_enum: AudioClipEnum
){
    let audio_clip = match clips_enum {
        AudioClipEnum::NoSound => asset_server.load(""),
        AudioClipEnum::LaserShotSilenced => asset_server.load("audio/clips/laser_shot_silenced.ogg"),
        AudioClipEnum::LightPow => asset_server.load("audio/clips/light_pow.ogg"),
        AudioClipEnum::LightExplosion => asset_server.load("audio/clips/light_explosion.ogg"),
        AudioClipEnum::Collection1 => asset_server.load("audio/clips/collection1.ogg"),
        AudioClipEnum::PointCounter => asset_server.load("audio/clips/point_counter.ogg"),
        AudioClipEnum::SaltExplosion => asset_server.load("audio/clips/salt_explosion.ogg"),
        AudioClipEnum::SputterRocket => asset_server.load("audio/clips/sputter_rocket.ogg"),
        AudioClipEnum::CoinLarry => asset_server.load("audio/clips/coin_larry.ogg"),
        AudioClipEnum::EventSlam => asset_server.load("audio/clips/event_slam.ogg"),
    };

    audio.play(audio_clip);
    return;
}