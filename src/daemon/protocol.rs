//! Daemon protocol — request/response types for CLI-daemon communication.

use serde::{Deserialize, Serialize};

/// Request from CLI to daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Request {
    /// Search for symbols/files
    #[serde(rename = "search")]
    Search { query: String, depth: usize },

    /// Get full context for a symbol
    #[serde(rename = "context")]
    Context { query: String, intent: String },

    /// Get dependencies for a symbol
    #[serde(rename = "deps")]
    Deps { symbol: String },

    /// Get graph statistics
    #[serde(rename = "stats")]
    Stats,

    /// Get codebase overview
    #[serde(rename = "overview")]
    Overview,

    /// Force rebuild the graph
    #[serde(rename = "rebuild")]
    Rebuild,

    /// Check if daemon is alive
    #[serde(rename = "ping")]
    Ping,

    /// Shutdown the daemon
    #[serde(rename = "shutdown")]
    Shutdown,
}

/// Response from daemon to CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Response {
    /// Successful response with JSON data
    #[serde(rename = "ok")]
    Ok { data: serde_json::Value },

    /// Error response
    #[serde(rename = "error")]
    Error { message: String },

    /// Pong response (daemon is alive)
    #[serde(rename = "pong")]
    Pong,

    /// Shutdown acknowledgment
    #[serde(rename = "goodbye")]
    Goodbye,
}

impl Response {
    pub fn ok<T: Serialize>(data: T) -> Self {
        Response::Ok {
            data: serde_json::to_value(data).unwrap_or(serde_json::Value::Null),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Response::Error { message: msg.into() }
    }
}
