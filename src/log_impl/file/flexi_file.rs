use flexi_logger;
use flexi_logger::FileSpec;

pub(crate) fn init() {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::opt_format)
        .log_to_file(FileSpec::try_from("logs/flexi_file.log").unwrap())
        .start()
        .unwrap();
}
