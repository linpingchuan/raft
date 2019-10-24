extern crate slog_term;
#[macro_use]
extern crate slog;
extern crate slog_async;
fn main() {
    // env_logger::init().unwrap();

    let dorcorator=slog_term::TermDecorator::new().build();
    let drain=slog_term::FullFormat::new(dorcorator).build().fuse();
    let drain=slog_async::Async::new(drain).build().fuse();
    let log=slog::Logger::root(drain, o!("version"=>"0.5"));

    debug!(log,"starting";"what"=>"the_thing");
}
