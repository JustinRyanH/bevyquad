use crate::prelude::*;
use miniquad::*;

pub fn miniquad_runner(mut app: App) {
    let config = conf::Conf {
        window_title: "FlappyBird".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    };
    miniquad::start(config, |ctx| {
        app.insert_resource(ctx);
        UserData::Free(Box::new(Stage { app }))
    });
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner);
    }
}

struct Stage {
    app: App,
}

impl EventHandlerFree for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let ctx = self
            .app
            .world
            .get_resource_mut::<miniquad::Context>()
            .unwrap();
        ctx.clear(Some((0.13, 0.137, 0.137, 1.0)), None, None);
    }
}
