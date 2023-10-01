use once_cell::sync::OnceCell;

static TRACING_SUB_FMT_SET: OnceCell<()> = OnceCell::new();

pub fn init_logger() {
    TRACING_SUB_FMT_SET.get_or_init(tracing_subscriber::fmt::init);
}
