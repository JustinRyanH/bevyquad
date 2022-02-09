mod color;
mod input;
pub mod mq;

mod prelude {
    pub(crate) use bevy_app::prelude::*;
    pub(crate) use bevy_math::prelude::*;
    pub(crate) use bevy_transform::prelude::*;

    pub use crate::color::*;
    pub use crate::input::*;
    pub use crate::mq::{DebugShape2D, DebugText, MiniquadPlugin};
}

use crate::prelude::*;

fn main() {
    App::default()
        .add_plugin(TransformPlugin)
        .add_plugin(MiniquadPlugin::default())
        .run();
}
