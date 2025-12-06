//! Windows thread metrics collection (stub implementation)
use super::ThreadMetrics;

/// Collect per-thread CPU usage metrics for the current process on Windows (stub)
pub(crate) fn collect_thread_metrics() -> Result<Vec<ThreadMetrics>, String> {
    Ok(Vec::new())
}

/// Get the RSS (Resident Set Size) of the current process in bytes (stub)
pub(crate) fn get_rss_bytes() -> Option<u64> {
    None
}
