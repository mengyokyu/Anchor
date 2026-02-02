//! Daemon server — Unix socket server that handles CLI requests.

use anyhow::Result;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use tracing::{debug, error, info, warn};

use crate::graph::engine::CodeGraph;
use crate::watcher::{start_watching, WatcherHandle};
use crate::{anchor_dependencies, anchor_stats, build_graph, get_context, graph_search};

use super::protocol::{Request, Response};

/// Default socket path (in project's .anchor directory)
pub fn socket_path(root: &Path) -> PathBuf {
    root.join(".anchor").join("anchor.sock")
}

/// PID file path
pub fn pid_path(root: &Path) -> PathBuf {
    root.join(".anchor").join("daemon.pid")
}

/// Start the daemon server.
pub fn start_daemon(root: &Path) -> Result<()> {
    let root = root.canonicalize()?;
    let sock_path = socket_path(&root);
    let pid_file = pid_path(&root);

    // Ensure .anchor directory exists
    std::fs::create_dir_all(sock_path.parent().unwrap())?;

    // Remove stale socket if exists
    if sock_path.exists() {
        std::fs::remove_file(&sock_path)?;
    }

    // Write PID file
    std::fs::write(&pid_file, std::process::id().to_string())?;

    // Build initial graph
    info!(root = %root.display(), "building initial graph");
    let graph = build_graph(&root);
    let graph = Arc::new(RwLock::new(graph));

    // Start file watcher
    let _watcher: Option<WatcherHandle> = match start_watching(&root, Arc::clone(&graph), 200) {
        Ok(handle) => {
            info!("file watcher started");
            Some(handle)
        }
        Err(e) => {
            warn!(error = %e, "file watcher failed to start");
            None
        }
    };

    // Bind socket
    let listener = UnixListener::bind(&sock_path)?;
    info!(socket = %sock_path.display(), "daemon listening");

    // Shutdown flag
    let shutdown = Arc::new(AtomicBool::new(false));

    // Accept connections
    for stream in listener.incoming() {
        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        match stream {
            Ok(stream) => {
                let graph = Arc::clone(&graph);
                let shutdown = Arc::clone(&shutdown);
                let root = root.clone();

                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, &graph, &shutdown, &root) {
                        debug!(error = %e, "client handler error");
                    }
                });
            }
            Err(e) => {
                error!(error = %e, "accept error");
            }
        }
    }

    // Cleanup
    info!("daemon shutting down");
    let _ = std::fs::remove_file(&sock_path);
    let _ = std::fs::remove_file(&pid_file);

    Ok(())
}

/// Handle a single client connection.
fn handle_client(
    stream: UnixStream,
    graph: &Arc<RwLock<CodeGraph>>,
    shutdown: &Arc<AtomicBool>,
    root: &Path,
) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let request: Request = serde_json::from_str(&line)?;
    debug!(?request, "received request");

    let response = process_request(request, graph, shutdown, root);

    let response_json = serde_json::to_string(&response)?;
    writeln!(writer, "{}", response_json)?;

    Ok(())
}

/// Process a request and return a response.
fn process_request(
    request: Request,
    graph: &Arc<RwLock<CodeGraph>>,
    shutdown: &Arc<AtomicBool>,
    root: &Path,
) -> Response {
    match request {
        Request::Ping => Response::Pong,

        Request::Shutdown => {
            shutdown.store(true, Ordering::Relaxed);
            Response::Goodbye
        }

        Request::Stats => {
            let g = match graph.read() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            let result = anchor_stats(&g);
            Response::ok(result)
        }

        Request::Search { query, depth } => {
            let g = match graph.read() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            let result = graph_search(&g, &query, depth);
            Response::ok(result)
        }

        Request::Context { query, intent } => {
            let g = match graph.read() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            let result = get_context(&g, &query, &intent);
            Response::ok(result)
        }

        Request::Deps { symbol } => {
            let g = match graph.read() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            let result = anchor_dependencies(&g, &symbol);
            Response::ok(result)
        }

        Request::Overview => {
            let g = match graph.read() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            let stats = g.stats();
            let files = graph_search(&g, "src/", 0);
            let mains = graph_search(&g, "main", 0);
            Response::ok(serde_json::json!({
                "stats": stats,
                "files": files.matched_files,
                "entry_points": mains.symbols.iter()
                    .filter(|s| s.name == "main")
                    .collect::<Vec<_>>()
            }))
        }

        Request::Rebuild => {
            let new_graph = build_graph(root);
            let mut g = match graph.write() {
                Ok(g) => g,
                Err(e) => return Response::error(format!("lock error: {}", e)),
            };
            *g = new_graph;
            let stats = g.stats();
            Response::ok(serde_json::json!({
                "message": "graph rebuilt",
                "stats": stats
            }))
        }
    }
}

/// Check if daemon is running by checking PID file and process.
pub fn is_daemon_running(root: &Path) -> bool {
    let pid_file = pid_path(root);

    if !pid_file.exists() {
        return false;
    }

    // Read PID and check if process is alive
    if let Ok(pid_str) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            // Check if process exists (signal 0 = check existence)
            unsafe {
                return libc::kill(pid, 0) == 0;
            }
        }
    }

    false
}

/// Send a request to the daemon and get a response.
pub fn send_request(root: &Path, request: Request) -> Result<Response> {
    let sock_path = socket_path(root);
    let mut stream = UnixStream::connect(&sock_path)?;

    let request_json = serde_json::to_string(&request)?;
    writeln!(stream, "{}", request_json)?;

    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line)?;

    let response: Response = serde_json::from_str(&response_line)?;
    Ok(response)
}
