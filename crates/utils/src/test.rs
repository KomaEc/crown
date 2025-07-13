#[ctor::ctor]
fn init_logger() {
    tracing_subscriber::fmt::init();
}
