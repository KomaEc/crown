use once_cell::sync::OnceCell;

static ENV_LOG_SET: OnceCell<()> = OnceCell::new();

pub fn init_logger() {
    ENV_LOG_SET.get_or_init(|| env_logger::init());
}