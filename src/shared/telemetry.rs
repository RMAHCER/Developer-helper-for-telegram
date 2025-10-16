// Telemetry - logging and monitoring setup
use tracing_subscriber::EnvFilter;

/// Initialize logging system
pub fn init_telemetry() -> Result<(), String> {
    // Create filter from RUST_LOG environment variable
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Failed to initialize logging filter");

    // Initialize subscriber with pretty format
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .try_init()
        .map_err(|e| format!("Failed to initialize telemetry: {}", e))?;

    tracing::info!("✅ Telemetry initialized");
    Ok(())
}
