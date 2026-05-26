/// Application configuration loaded from environment variables.
/// All required fields validated at startup — fail fast on missing config.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server
    pub server_host: String,
    pub server_port: u16,

    // Database
    pub scylla_nodes: Vec<String>,
    pub scylla_keyspace: String,
    pub scylla_username: Option<String>,
    pub scylla_password: Option<String>,

    // Redis
    pub redis_url: String,

    // Security
    pub jwt_secret: String,
    pub argon2_memory_kb: u32,
    pub argon2_iterations: u32,
    pub argon2_parallelism: u32,

    // Media
    pub storage_backend: String,
    pub s3_bucket: String,
    pub s3_region: String,

    // NATS
    pub nats_url: String,

    // Observability
    pub log_level: String,

    // Limits
    pub max_message_size_bytes: usize,
    pub max_media_size_bytes: usize,
    pub max_group_members: usize,
    pub rate_limit_auth_per_min: u32,
    pub rate_limit_api_per_min: u32,
}

impl AppConfig {
    /// Load configuration from environment variables.
    /// Panics on missing required fields (fail fast).
    pub fn from_env() -> Self {
        Self {
            server_host: env_or("SERVER_HOST", "0.0.0.0"),
            server_port: env_or("SERVER_PORT", "8080").parse().expect("Invalid SERVER_PORT"),

            scylla_nodes: env_or("SCYLLA_NODES", "localhost:9042")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            scylla_keyspace: env_or("SCYLLA_KEYSPACE", "ghostlink"),
            scylla_username: std::env::var("SCYLLA_USERNAME").ok(),
            scylla_password: std::env::var("SCYLLA_PASSWORD").ok(),

            redis_url: env_or("REDIS_URL", "redis://localhost:6379"),

            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET is required"),
            argon2_memory_kb: env_or("ARGON2_MEMORY_KB", "65536").parse().unwrap(),
            argon2_iterations: env_or("ARGON2_ITERATIONS", "3").parse().unwrap(),
            argon2_parallelism: env_or("ARGON2_PARALLELISM", "4").parse().unwrap(),

            storage_backend: env_or("STORAGE_BACKEND", "minio"),
            s3_bucket: env_or("S3_BUCKET", "ghostlink-media"),
            s3_region: env_or("S3_REGION", "us-east-1"),

            nats_url: env_or("NATS_URL", "nats://localhost:4222"),

            log_level: env_or("LOG_LEVEL", "info"),

            max_message_size_bytes: env_or("MAX_MESSAGE_SIZE_BYTES", "65536").parse().unwrap(),
            max_media_size_bytes: env_or("MAX_MEDIA_SIZE_BYTES", "52428800").parse().unwrap(),
            max_group_members: env_or("MAX_GROUP_MEMBERS", "256").parse().unwrap(),
            rate_limit_auth_per_min: env_or("RATE_LIMIT_AUTH_PER_MIN", "10").parse().unwrap(),
            rate_limit_api_per_min: env_or("RATE_LIMIT_API_PER_MIN", "300").parse().unwrap(),
        }
    }
}

/// Helper: get env var or return default.
fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
