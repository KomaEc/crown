use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[ctor::ctor]
fn init_logger() {
    let logfile = std::fs::File::create("../../test.log")
        .unwrap()
        .with_max_level(tracing::Level::DEBUG);
    let stdout = std::io::stdout.with_max_level(tracing::Level::DEBUG);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .without_time()
                .with_target(false)
                .with_level(false)
                .with_writer(stdout.and(logfile)),
        )
        .with(
            EnvFilter::from_default_env()
                .add_directive("analyses::output_params=debug".parse().unwrap()),
        )
        .init();
}
