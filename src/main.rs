mod camera;
mod color;
mod input;
pub mod mq;

mod prelude {
    pub(crate) use bevy_app::prelude::*;
    pub(crate) use bevy_math::prelude::*;

    pub use crate::camera::Camera2D;
    pub use crate::color::*;
    pub use crate::input::*;
    pub use crate::mq::{DebugShape2D, DebugText, MiniquadPlugin};
}

use crate::prelude::*;

fn main() {
    App::default().add_plugin(MiniquadPlugin::default()).run();
}
