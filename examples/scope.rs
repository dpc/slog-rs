#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;

use slog::DrainExt;

fn main() {
	slog_scope::set_global_logger(slog::Logger::root(slog_term::streamer().build().fuse(), o![]));

	info!["Info message using the global logger"];

	slog_scope::scope(slog_scope::logger().new(o!["where" => "A RAII scope"]), || {
		warn!["You are sending this to a composed logger"];
		info!["You can also call any functions in this thread, and the logger will be passed via the thread-local"];
	});

	debug!["This is sent to the global logger, the scoped logger was popped"];
}
