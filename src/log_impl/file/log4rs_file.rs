use log4rs;

pub fn init() {
    log4rs::init_file("config/log4rs_file.yaml", Default::default()).unwrap();
}
