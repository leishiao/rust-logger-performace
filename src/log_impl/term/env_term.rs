use env_logger;

pub fn init() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .default_format()
        .try_init()
        .unwrap();
}
