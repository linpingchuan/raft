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



