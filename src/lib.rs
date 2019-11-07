#[cfg(feature = "failpoints")]
#[macro_use]
extern crate fail;

#[macro_use]
extern crate slog;
#[macro_use]
extern crate quick_error;

mod util;

macro_rules! fatal {
    
    ($logger:expr, $msg:expr) => {{
        let owned_kv = ($logger).list();
        let s = crate::util::format_kv_list(&owned_kv);
        if s.is_empty() {
            panic!("{}", $msg)
        } else {
            panic!("{}, {}", $msg, s)
        }
    }};
    ($logger:expr, $fmt:expr, $($arg:tt)+) => {{
        fatal!($logger, format_args!($fmt, $($arg)+))
    }};
}

mod raft_log;

mod storage;

mod protos;

mod errors;

pub mod raft;

mod log_unstable;

/// The default logger we fall back to when passed `None` in external facing constructors.
///
/// Currently, this is a `log` adaptor behind a `Once` to ensure there is no clobbering.
#[cfg(any(test, feature = "default-logger"))]
pub fn default_logger() -> slog::Logger {
    use slog::Drain;
    use std::sync::{Mutex, Once};

    static LOGGER_INITIALIZED: Once = Once::new();
    static mut LOGGER: Option<slog::Logger> = None;

    let logger = unsafe {
        LOGGER_INITIALIZED.call_once(|| {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::CompactFormat::new(decorator).build();
            let drain = slog_envlogger::new(drain);
            LOGGER = Some(slog::Logger::root(Mutex::new(drain).fuse(), o!()));
        });
        LOGGER.as_ref().unwrap()
    };
    let case = std::thread::current()
        .name()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .to_string();
    logger.new(o!("case" => case))
}



