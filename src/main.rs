use ggez::ContextBuilder;
use ggez::event;
use log::debug;
use log::info;

use crate::companion::load_companion_config;
use crate::companion::load_config;
use crate::core::CompanionApp;
use crate::errors::unwrap_or_exit;

mod animation;
mod behavior;
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

    // `ggez` is synchronous, so at this moment we can spawn only one companion.
    // TODO: Make a command line interface so we can spawn new process for every companion instead
    // of threads.
    config.companion.iter().for_each(|c| {
        debug!("Loading {}", c.name);
        let (mut ctx, event_loop) = ContextBuilder::new("desktop-companion", "w1ltshire")
            .window_mode(
                ggez::conf::WindowMode::default()
                    .transparent(true)
                    .borderless(true)
                    .dimensions(c.width, c.height),
            )
            .build()
            .expect("Could not create ggez context");

        debug!(
            "{}/config/{}/companion.toml",
            std::env::current_dir().unwrap().to_str().unwrap(),
            &c.path
        );

        let companion_config = unwrap_or_exit(
            load_companion_config(&format!(
                "{}/config/{}/companion.toml",
                std::env::current_dir().unwrap().to_str().unwrap(),
                &c.path
            )),
            1,
        );

        let app = CompanionApp::new(&mut ctx, c.clone(), companion_config);
        event::run(ctx, event_loop, app);
    });
}
