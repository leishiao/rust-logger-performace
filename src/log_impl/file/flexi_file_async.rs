use flexi_logger;
use flexi_logger::FileSpec;
use flexi_logger::LoggerHandle;
use flexi_logger::WriteMode;

pub(crate) fn init() -> LoggerHandle {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::Async)
        .format(flexi_logger::opt_format)
        .log_to_file(FileSpec::try_from("logs/flexi_file_async.log").unwrap())
        .start()
        .unwrap()
}
