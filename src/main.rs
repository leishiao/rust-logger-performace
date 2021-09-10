mod log_impl;
mod slog_impl;
mod tracing_impl;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use log::info;
use std::thread;

pub(crate) const PLAIN_TEXT: &str = "plain text.";

fn main() {
    let matchs = set_clap();
    if !matchs.is_present("impl") && !matchs.is_present("count") && !matchs.is_present("threads") {
        println!("{}", matchs.usage());
        return;
    }
    let log_impl = matchs.value_of("impl").unwrap();

    let count = matchs.value_of("count").unwrap();
    let count = i64::from_str_radix(count, 10).expect(format!("can't parse {}", count).as_str());

    let threads = matchs.value_of("threads").unwrap();
    let threads =
        i64::from_str_radix(threads, 10).expect(format!("can't parse {}", count).as_str());
    println!("impl:{},count:{},threads:{}", log_impl, count, threads);

    match log_impl {
        "env_term" => crate::log_impl::term::env_term::init(),

        "flexi_term" => crate::log_impl::term::flexi_term::init(),
        "flexi_file" => crate::log_impl::file::flexi_file::init(),
        "flexi_file_async" => {
            let _handle = crate::log_impl::file::flexi_file_async::init();
            log_multi_thread(threads, count);
            return;
        }
        "flexi_roll" => crate::log_impl::file::flexi_roll::init(),
        "flexi_roll_async" => {
            let _handle = crate::log_impl::file::flexi_roll_async::init();
            log_multi_thread(threads, count);
            return;
        }
        "log4rs_term" => crate::log_impl::term::log4rs_term::init(),
        "log4rs_file" => crate::log_impl::file::log4rs_file::init(),
        "log4rs_roll" => crate::log_impl::file::log4rs_roll::init(),
        "tracing_term" => {
            let _guard = crate::tracing_impl::term::tracing_term::init();
            log_multi_thread(threads, count);
            return;
        }
        "tracing_file_nb" => {
            let _guard = crate::tracing_impl::file::tracing_file_nb::init();
            log_multi_thread(threads, count);
            return;
        }
        "tracing_roll_nb" => {
            let _guard = crate::tracing_impl::file::tracing_roll_nb::init();
            log_multi_thread(threads, count);
            return;
        }

        _ => {
            panic!("unknown impl:{}", log_impl);
        }
    }
    log_multi_thread(threads, count);
}

pub(crate) fn log_multi_thread(thread: i64, count: i64) {
    let mut handles = Vec::new();
    for _ in 0..thread {
        let handle = thread::spawn(move || log_n(count));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

pub(crate) fn log_n(count: i64) {
    for _ in 0..count {
        info!("{}", PLAIN_TEXT);
    }
}

fn set_clap() -> ArgMatches<'static> {
    App::new("Log Compare")
    .about("compare rust log implementation performance.")
    .author("leishiao")
    .arg(
        Arg::with_name("impl")
            .long("impl")
            .short("i")
            .help("one of slog_term,env_term,flexi_term,flexi_file,flexi_roll,log4rs_term,log4rs_file,log4rs_roll,tracing_term")
            .takes_value(true)
        )
    .arg(
        Arg::with_name("count")
        .long("count")
        .short("c")
        .help("log line count.")
        .takes_value(true)
    )
    .arg(
        Arg::with_name("threads")
        .long("threads")
        .short("t")
        .help("log thread count.")
        .takes_value(true)
    ).get_matches()
}
