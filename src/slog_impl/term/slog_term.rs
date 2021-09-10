use slog::o;
use slog::Drain;
use slog_async;
use slog_scope;
use slog_stdlog;
use slog_term;

static mut GLOBAL: Option<slog_scope::GlobalLoggerGuard> = None;

pub fn init() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let _scope_guard = slog_scope::set_global_logger(logger);
    unsafe { GLOBAL.replace(_scope_guard) };
    let _log_guard = slog_stdlog::init_with_level(log::Level::Debug).unwrap();
}
