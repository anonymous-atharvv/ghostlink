use governor::middleware::NoOpMiddleware;
use std::sync::Arc;
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::PeerIpKeyExtractor,
    GovernorLayer,
};

use crate::config::AppConfig;

/// Rate limiting configuration.
/// Uses tower-governor for per-IP and per-account limiting.
pub struct RateLimitConfig;

impl RateLimitConfig {
    /// Create a GovernorLayer for auth endpoints (per-IP, strict).
    pub fn auth_layer(config: &Arc<AppConfig>) -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware> {
        let gov_config = GovernorConfigBuilder::default()
            .per_second(config.rate_limit_auth_per_min as u64 / 60)
            .burst_size(config.rate_limit_auth_per_min as u32)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .expect("Failed to build auth rate limiter");

        GovernorLayer { config: Arc::new(gov_config) }
    }

    /// Create a GovernorLayer for API endpoints (per-account).
    pub fn api_layer(config: &Arc<AppConfig>) -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware> {
        let gov_config = GovernorConfigBuilder::default()
            .per_second(config.rate_limit_api_per_min as u64 / 60)
            .burst_size(config.rate_limit_api_per_min as u32)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .expect("Failed to build API rate limiter");

        GovernorLayer { config: Arc::new(gov_config) }
    }
}
