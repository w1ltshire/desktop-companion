use std::ops::Deref;

use ggez::ContextBuilder;
use ggez::event;
use log::info;

use crate::companion::load_config;
use crate::core::CompanionApp;
use crate::errors::unwrap_or_exit;

mod companion;
mod core;
mod errors;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("wgpu", log::LevelFilter::Error)
        .level_for("wgpu_core", log::LevelFilter::Error)
        .level_for("naga", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
 
    let config = unwrap_or_exit(load_config(), 1);
    info!("{:#?}", config);

    config.companion.iter().for_each(|c| {
        let (mut ctx, event_loop) = ContextBuilder::new("desktop-companion", "w1ltshire")
            .window_mode(
                ggez::conf::WindowMode::default()
                .transparent(true)
                .borderless(true),
                )
            .build()
            .expect("Could not create ggez context");
        let app = CompanionApp::new(&mut ctx, c.clone());
        event::run(ctx, event_loop, app);
    });

}
