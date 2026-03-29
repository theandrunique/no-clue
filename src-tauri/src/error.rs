pub fn log_err<E: std::fmt::Display>(e: E, context: &str) -> String {
    tracing::error!(error = %e, context);
    e.to_string()
}
