use once_cell::sync::OnceCell;

static SET: OnceCell<()> = OnceCell::new();

use tracing_subscriber::{filter, fmt, prelude::*};

pub fn init_logger() {
    let filter = filter::LevelFilter::DEBUG;
    SET.get_or_init(|| {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::default())
            .init()
    });
}
