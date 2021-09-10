use log4rs;

pub fn init() {
    log4rs::init_file("config/log4rs_roll.yaml", Default::default()).unwrap();
}
