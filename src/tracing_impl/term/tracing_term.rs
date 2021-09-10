use log4rs::encode::writer;
use tracing::Level;
use tracing_appender::non_blocking::{WorkerGuard, DEFAULT_BUFFERED_LINES_LIMIT};
use tracing_log;
use tracing_subscriber::fmt;

pub(crate) fn init() -> WorkerGuard {
    tracing_log::LogTracer::init().unwrap();
    let (writer, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
        .buffered_lines_limit(DEFAULT_BUFFERED_LINES_LIMIT)
        .lossy(false)
        .finish(std::io::stdout());
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_timer(fmt::time::SystemTime)
        .with_level(true)
        .with_target(true)
        .event_format(fmt::format::Format::default())
        .with_writer(writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    return guard;
}
