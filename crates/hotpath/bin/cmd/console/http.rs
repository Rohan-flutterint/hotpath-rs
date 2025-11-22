use eyre::Result;
use hotpath::channels::ChannelLogs;
use hotpath::streams::{StreamLogs, StreamsJson};
use hotpath::{FunctionLogsJson, FunctionsJson};

/// Fetches timing metrics from the hotpath HTTP server
pub(crate) fn fetch_functions_timing(agent: &ureq::Agent, port: u16) -> Result<FunctionsJson> {
    let url = format!("http://localhost:{}/functions_timing", port);
    let metrics: FunctionsJson = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(metrics)
}

/// Fetches allocation metrics from the hotpath HTTP server
/// Returns None if hotpath-alloc feature is not enabled (404 response)
pub(crate) fn fetch_functions_alloc(
    agent: &ureq::Agent,
    port: u16,
) -> Result<Option<FunctionsJson>> {
    let url = format!("http://localhost:{}/functions_alloc", port);
    let response = agent.get(&url).call();

    match response {
        Ok(mut resp) => {
            let metrics: FunctionsJson = resp
                .body_mut()
                .read_json()
                .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
            Ok(Some(metrics))
        }
        Err(e) => {
            // Check if it's a 404 response (feature not enabled)
            let err_str = e.to_string();
            if err_str.contains("404") {
                return Ok(None);
            }
            Err(eyre::eyre!("HTTP request failed: {}", e))
        }
    }
}

/// Fetches channels from the hotpath HTTP server
pub(crate) fn fetch_channels(
    agent: &ureq::Agent,
    port: u16,
) -> Result<hotpath::channels::ChannelsJson> {
    let url = format!("http://localhost:{}/channels", port);
    let channels: hotpath::channels::ChannelsJson = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(channels)
}

/// Fetches recent logs for a specific function
pub(crate) fn fetch_function_logs(
    agent: &ureq::Agent,
    port: u16,
    function_name: &str,
) -> Result<FunctionLogsJson> {
    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(function_name.as_bytes());

    let url = format!("http://localhost:{}/functions/{}/logs", port, encoded);
    let function_logs: FunctionLogsJson = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(function_logs)
}

/// Fetches logs for a specific channel from the HTTP server
pub(crate) fn fetch_channel_logs(
    agent: &ureq::Agent,
    port: u16,
    channel_id: u64,
) -> Result<ChannelLogs> {
    let url = format!("http://localhost:{}/channels/{}/logs", port, channel_id);
    let logs: ChannelLogs = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(logs)
}

/// Fetches streams from the hotpath HTTP server
pub(crate) fn fetch_streams(agent: &ureq::Agent, port: u16) -> Result<StreamsJson> {
    let url = format!("http://localhost:{}/streams", port);
    let streams: StreamsJson = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(streams)
}

/// Fetches logs for a specific stream from the HTTP server
pub(crate) fn fetch_stream_logs(
    agent: &ureq::Agent,
    port: u16,
    stream_id: u64,
) -> Result<StreamLogs> {
    let url = format!("http://localhost:{}/streams/{}/logs", port, stream_id);
    let logs: StreamLogs = agent
        .get(&url)
        .call()
        .map_err(|e| eyre::eyre!("HTTP request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| eyre::eyre!("JSON deserialization failed: {}", e))?;
    Ok(logs)
}
