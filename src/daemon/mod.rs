//! Daemon module — background process for real-time graph updates.
//!
//! The daemon keeps the code graph in memory, watches for file changes,
//! and serves queries over a Unix socket. This enables instant queries
//! without loading the graph from disk on every CLI command.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │           anchor daemon                  │
//! │  - graph in memory                      │
//! │  - file watcher (incremental updates)   │
//! │  - Unix socket server                   │
//! └─────────────────────────────────────────┘
//!           ▲
//!           │ .anchor/anchor.sock
//!           ▼
//! ┌─────────────────────────────────────────┐
//! │           anchor CLI                     │
//! │  - connects to daemon                   │
//! │  - sends JSON requests                  │
//! │  - receives JSON responses              │
//! └─────────────────────────────────────────┘
//! ```

pub mod protocol;
pub mod server;

pub use protocol::{Request, Response};
pub use server::{is_daemon_running, send_request, socket_path, start_daemon};
