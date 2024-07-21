#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use wicked_moon::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
