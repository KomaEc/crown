use once_cell::sync::OnceCell;

static SET: OnceCell<()> = OnceCell::new();

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger() {
    let filter = EnvFilter::new("analysis=debug,alias=debug");
    SET.get_or_init(|| {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::default())
            .init()
    });
}
