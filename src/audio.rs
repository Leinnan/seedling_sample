use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_seedling::pool::Sampler;
use bevy_seedling::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SeedlingPlugin::default())
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_systems(
                Update,
                control_flying_sound
                    .after(set_movement_actions)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct FlyingAudio;

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        SamplePlayer::new(audio_assets.flying.clone())
            .looping()
            .with_volume(Volume::Linear(0.3)),
        FlyingAudio,
    ));
}

fn control_flying_sound(
    actions: Res<Actions>,
    mut audio_q: Query<(&mut PlaybackSettings, &Sampler), With<FlyingAudio>>,
) {
    for (mut settings, status) in audio_q.iter_mut() {
        if actions.player_movement.is_some() {
            if !status.is_playing() {
                settings.play();
            }
        }
        if actions.player_movement.is_none() && status.is_playing() {
            settings.pause();
        }
    }
}
