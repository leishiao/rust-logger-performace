use flexi_logger;

pub fn init() {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::opt_format)
        .log_to_stdout()
        .start()
        .unwrap();
}
