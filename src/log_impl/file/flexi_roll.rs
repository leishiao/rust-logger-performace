use flexi_logger;
use flexi_logger::Cleanup;
use flexi_logger::Criterion;
use flexi_logger::FileSpec;
use flexi_logger::Naming;

pub(crate) fn init() {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::opt_format)
        .log_to_file(FileSpec::try_from("logs/flexi_roll.log").unwrap())
        .rotate(
            Criterion::Size(50 * 1024 * 1024),
            Naming::Numbers,
            Cleanup::KeepLogFiles(1),
        )
        .start()
        .unwrap();
}
