use prometheus::register_gauge_vec;
use std::sync::Arc;

use crate::libs::metrics::Metrics;

pub async fn init() -> Result<Metrics, Box<dyn std::error::Error>> {
    let temperatures = register_gauge_vec!(
        "cpu_temperature",
        "Metric for CPU temperatures",
        &["unit", "temp_type"]
    )?;

    let metrics = Metrics { temperatures };

    // Wrap in Arc to allow shared ownership across async tasks or Axum app state
    // Arc allows you to clone and share the metrics instance safely across threads or tasks
    // (like multiple HTTP handlers or background workers).
    // Without Arc, you’d only be able to use it in one place.
    let metrics = Arc::new(metrics);

    Ok(metrics.as_ref().clone())
}
