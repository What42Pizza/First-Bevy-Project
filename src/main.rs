// Started 03/09/23
// Last updated 03/19/23



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// clippy
#![allow(clippy::too_many_arguments)]

// nightly
#![feature(duration_constants)]



pub mod player_mod;
pub mod room_generation_mod;
pub mod misc;
pub mod custom_events;
pub mod fns;
pub mod settings;
pub mod prelude;



use crate::prelude::*;



pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(player::MainPlugin)
        .add_plugin(room_generation::MainPlugin)
        .add_plugin(misc::MainPlugin)
        .run();
}
