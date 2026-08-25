use tracing::Metadata;
use tracing_subscriber::filter::{FilterExt, filter_fn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn default_filter(verbosity: u8) -> &'static str {
    match verbosity {
        0 => "info",
        1 => "codex_free=debug,rmcp=warn",
        _ => "codex_free=trace,rmcp=warn",
    }
}

pub fn init(verbosity: u8) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter(verbosity)));
    let protocol_guard = filter_fn(protocol_event_allowed);
    let layer = tracing_subscriber::fmt::layer().with_filter(env_filter.and(protocol_guard));

    tracing_subscriber::registry().with(layer).init();
}

fn protocol_event_allowed(metadata: &Metadata<'_>) -> bool {
    framework_event_allowed(metadata.target())
}

pub fn framework_event_allowed(target: &str) -> bool {
    !target.starts_with("rmcp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbosity_targets_codex_free_before_protocol_internals() {
        assert_eq!(default_filter(0), "info");
        assert_eq!(default_filter(1), "codex_free=debug,rmcp=warn");
        assert_eq!(default_filter(2), "codex_free=trace,rmcp=warn");
        assert_eq!(default_filter(u8::MAX), "codex_free=trace,rmcp=warn");
    }

    #[test]
    fn rmcp_protocol_events_cannot_be_enabled_by_rust_log() {
        assert!(!framework_event_allowed("rmcp::service"));
        assert!(!framework_event_allowed("rmcp::transport"));
        assert!(framework_event_allowed("codex_free::server"));
    }
}
