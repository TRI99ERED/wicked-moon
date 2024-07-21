use bevy::prelude::*;

mod credits;
mod loading;
mod playing;
mod splash;
mod title;

use credits::CreditsPlugin;
use loading::LoadingPlugin;
use playing::PlayingPlugin;
use splash::SplashPlugin;
use title::TitlePlugin;

pub(super) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CreditsPlugin, LoadingPlugin, PlayingPlugin, SplashPlugin, TitlePlugin))
            .init_state::<Screen>()
            .enable_state_scoped_entities::<Screen>();
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
}
