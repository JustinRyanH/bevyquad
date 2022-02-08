pub mod mq;

mod prelude {
    pub(crate) use bevy_app::prelude::*;

    pub use crate::mq::MiniquadPlugin;
}

use crate::prelude::*;

fn main() {
    App::default().add_plugin(MiniquadPlugin::default()).run();
}
