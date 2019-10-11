#[cfg(test)]
mod tests {
    #[test]
    fn test_key_values() {
        // checks if the built-in macros are correctly resolved.

        let _ = slog::kv!("a" => "A");
        let _ = slog::kv!("a" => %"A");
        let _ = slog::kv!("a" => ?"A");

        let _ = slog::slog_kv!("a" => "A");
        let _ = slog::slog_kv!("a" => %"A");
        let _ = slog::slog_kv!("a" => ?"A");

        // checks if `local_inner_macros` works correctly.
        let _ = slog::o!("a" => "A");
        let _ = slog::b!("a" => "A");
        let _ = slog::slog_o!("a" => "A");
        let _ = slog::slog_b!("a" => "A");
    }

    #[test]
    fn test_log() {
        let logger = slog::Logger::root(slog::Discard, slog::o!());

        // checks if the built-in macros are correctly resolved.
        slog::log!(logger, slog::Level::Info, "", "logger message");
        slog::log!(logger, slog::Level::Info, "", "{}", 42);
        slog::log!(logger, slog::Level::Info, "", "{}{}", a = "A", b = "B");
        slog::log!(logger, slog::Level::Info, "", "{}", a="A"; "id" => 42);

        slog::slog_log!(logger, slog::Level::Info, "", "logger message");
        slog::slog_log!(logger, slog::Level::Info, "", "{}", 42);
        slog::slog_log!(
            logger,
            slog::Level::Info,
            "",
            "{}{}",
            a = "A",
            b = "B"
        );
        slog::slog_log!(logger, slog::Level::Info, "", "{}", a="A"; "id" => 42);

        // checks if `local_inner_macros` works correctly.

        slog::trace!(logger, "message");
        slog::debug!(logger, "message");
        slog::info!(logger, "message");
        slog::warn!(logger, "message");
        slog::error!(logger, "message");
        slog::crit!(logger, "message");

        slog::slog_trace!(logger, "message");
        slog::slog_debug!(logger, "message");
        slog::slog_info!(logger, "message");
        slog::slog_warn!(logger, "message");
        slog::slog_error!(logger, "message");
        slog::slog_crit!(logger, "message");
    }
}
