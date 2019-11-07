extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::*;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = Logger::root(
        slog_term::FullFormat::new(plain)
        .build().fuse(), o!() 
    );

    info!(logger, "Logging ready!");

    info!(logger,"Logging exit!");
}
