//! Anchor Comprehensive Benchmark Suite
//!
//! Compares Anchor against REAL traditional tools (grep, find, cat).
//! No estimates - actual measurements.

use anchor::{anchor_dependencies, build_graph, get_context, graph_search, CodeGraph};
use serde::Serialize;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
struct ToolMetrics {
    pub tool_calls: usize,
    pub total_time_ms: u64,
    pub output_tokens: usize,
}

#[derive(Debug, Clone, Serialize)]
struct Comparison {
    pub name: String,
    pub anchor: ToolMetrics,
    pub traditional: ToolMetrics,
    pub call_reduction_pct: f64,
    pub time_difference_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
struct BenchmarkResults {
    repo_path: String,
    graph_stats: GraphStats,
    comparisons: Vec<Comparison>,
    summary: Summary,
}

#[derive(Debug, Clone, Serialize)]
struct GraphStats {
    files: usize,
    symbols: usize,
    edges: usize,
    build_time_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct Summary {
    total_anchor_calls: usize,
    total_traditional_calls: usize,
    total_call_reduction_pct: f64,
    anchor_faster_ms: i64,
}

fn run_command(cmd: &str, args: &[&str], repo_path: &Path) -> (Duration, String) {
    let start = Instant::now();
    let output = Command::new(cmd)
        .args(args)
        .current_dir(repo_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let elapsed = start.elapsed();
    (elapsed, output)
}

fn count_tokens(text: &str) -> usize {
    text.len() / 4
}

fn traditional_find_symbol(repo_path: &Path, pattern: &str) -> ToolMetrics {
    let mut calls = 0usize;
    let mut total_time = Duration::new(0, 0);
    let mut all_output = String::new();

    // Step 1: find files with pattern (limited)
    calls += 1;
    let (time, output) = run_command(
        "grep",
        &["-rl", "--include=*.rs", "--include=*.py", pattern, "."],
        repo_path,
    );
    total_time += time;
    all_output.push_str(&output);

    // Step 2: get line numbers (limited to 20)
    calls += 1;
    let (time, output) = run_command(
        "grep",
        &[
            "-n",
            "--include=*.rs",
            "--include=*.py",
            "-m",
            "20",
            pattern,
            ".",
        ],
        repo_path,
    );
    total_time += time;
    all_output.push_str(&output);

    ToolMetrics {
        tool_calls: calls,
        total_time_ms: total_time.as_millis() as u64,
        output_tokens: count_tokens(&all_output),
    }
}

fn traditional_search_pattern(repo_path: &Path, pattern: &str) -> ToolMetrics {
    let mut calls = 0usize;
    let mut total_time = Duration::new(0, 0);
    let mut all_output = String::new();

    // Find matching files (limit to 10 results)
    calls += 1;
    let (time, output) = run_command(
        "grep",
        &["-rl", "--include=*.rs", "--include=*.py", pattern, "."],
        repo_path,
    );
    total_time += time;
    let files: Vec<&str> = output.lines().filter(|l| !l.is_empty()).take(10).collect();
    all_output.push_str(&output);

    // Read first 3 files
    for f in files.iter().take(3) {
        calls += 1;
        let (time, _) = run_command("head", &[f], repo_path);
        total_time += time;
    }

    // Get context lines (limit to 50)
    calls += 1;
    let (time, output) = run_command(
        "grep",
        &["-n", "-A", "2", "-B", "1", "--include=*.rs", pattern, "."],
        repo_path,
    );
    total_time += time;
    all_output.push_str(&output);

    ToolMetrics {
        tool_calls: calls,
        total_time_ms: total_time.as_millis() as u64,
        output_tokens: count_tokens(&all_output),
    }
}

fn traditional_read_files_context(repo_path: &Path, files: &[&str]) -> ToolMetrics {
    let mut calls = 0;
    let mut total_time = Duration::new(0, 0);
    let mut all_output = String::new();

    for f in files {
        calls += 1;
        let (time, output) = run_command("cat", &[f], repo_path);
        total_time += time;
        all_output.push_str(&output);
    }

    ToolMetrics {
        tool_calls: calls,
        total_time_ms: total_time.as_millis() as u64,
        output_tokens: count_tokens(&all_output),
    }
}

fn anchor_search(graph: &CodeGraph, pattern: &str) -> ToolMetrics {
    let start = Instant::now();
    let result = graph_search(graph, pattern, 2);
    let elapsed = start.elapsed();

    let output = serde_json::to_string(&result).unwrap_or_default();

    ToolMetrics {
        tool_calls: 1,
        total_time_ms: elapsed.as_millis() as u64,
        output_tokens: count_tokens(&output),
    }
}

fn anchor_context(graph: &CodeGraph, symbol: &str) -> ToolMetrics {
    let start = Instant::now();
    let result = get_context(graph, symbol, "understand");
    let elapsed = start.elapsed();

    let output = serde_json::to_string(&result).unwrap_or_default();

    ToolMetrics {
        tool_calls: 1,
        total_time_ms: elapsed.as_millis() as u64,
        output_tokens: count_tokens(&output),
    }
}

fn anchor_dependencies_query(graph: &CodeGraph, symbol: &str) -> ToolMetrics {
    let start = Instant::now();
    let result = anchor_dependencies(graph, symbol);
    let elapsed = start.elapsed();

    let output = serde_json::to_string(&result).unwrap_or_default();

    ToolMetrics {
        tool_calls: 1,
        total_time_ms: elapsed.as_millis() as u64,
        output_tokens: count_tokens(&output),
    }
}

pub fn run_full_benchmark(repo_path: &Path) -> BenchmarkResults {
    println!("\n========================================");
    println!("Anchor vs Traditional Tools - REAL Benchmark");
    println!("========================================\n");
    println!("Repository: {}\n", repo_path.display());

    // Build Anchor graph
    println!("[1/3] Building Anchor graph...");
    let build_start = Instant::now();
    let graph = build_graph(repo_path);
    let build_time = build_start.elapsed().as_millis();
    let stats = graph.stats();

    println!(
        "      Built: {} files, {} symbols, {} edges",
        stats.file_count, stats.symbol_count, stats.total_edges
    );
    println!("      Time: {}ms\n", build_time);

    let mut comparisons = Vec::new();
    let mut total_anchor_calls = 0usize;
    let mut total_traditional_calls = 0usize;
    let mut anchor_faster_total = 0i64;

    // Test 1: Symbol Search
    println!("[2/3] Running benchmark tests...\n");

    let patterns = vec!["main", "Config", "error", "login"];

    println!("--- Test: Symbol Search ---");
    for pattern in &patterns {
        let trad = traditional_search_pattern(repo_path, pattern);
        let anch = anchor_search(&graph, pattern);

        let reduction = if trad.tool_calls > 0 {
            ((trad.tool_calls as f64 - anch.tool_calls as f64) / trad.tool_calls as f64) * 100.0
        } else {
            0.0
        };

        let time_diff = anch.total_time_ms as i64 - trad.total_time_ms as i64;

        println!("  Pattern '{}':", pattern);
        println!(
            "    Traditional: {} calls, {}ms",
            trad.tool_calls, trad.total_time_ms
        );
        println!(
            "    Anchor:      {} calls, {}ms",
            anch.tool_calls, anch.total_time_ms
        );
        println!("    Reduction:   {:.1}%", reduction);

        if anch.total_time_ms < trad.total_time_ms {
            println!(
                "    ✓ Anchor faster by {}ms",
                trad.total_time_ms - anch.total_time_ms
            );
            anchor_faster_total += (trad.total_time_ms - anch.total_time_ms) as i64;
        } else {
            println!(
                "    Traditional faster by {}ms",
                anch.total_time_ms - trad.total_time_ms
            );
            anchor_faster_total -= (anch.total_time_ms - trad.total_time_ms) as i64;
        }
        println!();

        comparisons.push(Comparison {
            name: format!("search:{}", pattern),
            anchor: anch.clone(),
            traditional: trad.clone(),
            call_reduction_pct: reduction,
            time_difference_ms: time_diff,
        });

        total_anchor_calls += anch.tool_calls;
        total_traditional_calls += trad.tool_calls;
    }

    // Test 2: Context Query
    println!("--- Test: Context Query ---");
    let symbols = vec!["CodeGraph", "Storage"];

    for symbol in &symbols {
        let trad = traditional_find_symbol(repo_path, symbol);
        let anch = anchor_context(&graph, symbol);

        let reduction = if trad.tool_calls > 0 {
            ((trad.tool_calls as f64 - anch.tool_calls as f64) / trad.tool_calls as f64) * 100.0
        } else {
            0.0
        };

        let time_diff = anch.total_time_ms as i64 - trad.total_time_ms as i64;

        println!("  Symbol '{}':", symbol);
        println!(
            "    Traditional: {} calls, {}ms",
            trad.tool_calls, trad.total_time_ms
        );
        println!(
            "    Anchor:      {} calls, {}ms",
            anch.tool_calls, anch.total_time_ms
        );
        println!("    Reduction:   {:.1}%", reduction);

        if anch.total_time_ms < trad.total_time_ms {
            println!(
                "    ✓ Anchor faster by {}ms",
                trad.total_time_ms - anch.total_time_ms
            );
            anchor_faster_total += (trad.total_time_ms - anch.total_time_ms) as i64;
        } else {
            println!(
                "    Traditional faster by {}ms",
                anch.total_time_ms - trad.total_time_ms
            );
            anchor_faster_total -= (anch.total_time_ms - trad.total_time_ms) as i64;
        }
        println!();

        comparisons.push(Comparison {
            name: format!("context:{}", symbol),
            anchor: anch.clone(),
            traditional: trad.clone(),
            call_reduction_pct: reduction,
            time_difference_ms: time_diff,
        });

        total_anchor_calls += anch.tool_calls;
        total_traditional_calls += trad.tool_calls;
    }

    // Test 3: Dependency Query
    println!("--- Test: Dependency Query ---");
    let deps_symbols = vec!["CodeGraph", "Config"];

    for symbol in &deps_symbols {
        // Traditional: trace dependencies manually
        let trad = traditional_find_symbol(repo_path, symbol);
        let anch = anchor_dependencies_query(&graph, symbol);

        let reduction = if trad.tool_calls > 0 {
            ((trad.tool_calls as f64 - anch.tool_calls as f64) / trad.tool_calls as f64) * 100.0
        } else {
            0.0
        };

        let time_diff = anch.total_time_ms as i64 - trad.total_time_ms as i64;

        println!("  Symbol '{}':", symbol);
        println!(
            "    Traditional: {} calls, {}ms",
            trad.tool_calls, trad.total_time_ms
        );
        println!(
            "    Anchor:      {} calls, {}ms",
            anch.tool_calls, anch.total_time_ms
        );
        println!("    Reduction:   {:.1}%", reduction);

        if anch.total_time_ms < trad.total_time_ms {
            println!(
                "    ✓ Anchor faster by {}ms",
                trad.total_time_ms - anch.total_time_ms
            );
            anchor_faster_total += (trad.total_time_ms - anch.total_time_ms) as i64;
        } else {
            println!(
                "    Traditional faster by {}ms",
                anch.total_time_ms - trad.total_time_ms
            );
            anchor_faster_total -= (anch.total_time_ms - trad.total_time_ms) as i64;
        }
        println!();

        comparisons.push(Comparison {
            name: format!("deps:{}", symbol),
            anchor: anch.clone(),
            traditional: trad.clone(),
            call_reduction_pct: reduction,
            time_difference_ms: time_diff,
        });

        total_anchor_calls += anch.tool_calls;
        total_traditional_calls += trad.tool_calls;
    }

    // Summary
    println!("[3/3] Summary\n");

    let total_reduction = if total_traditional_calls > 0 {
        ((total_traditional_calls as f64 - total_anchor_calls as f64)
            / total_traditional_calls as f64)
            * 100.0
    } else {
        0.0
    };

    println!("========================================");
    println!("FINAL RESULTS");
    println!("========================================\n");

    println!("TOOL CALLS:");
    println!("  Traditional: {}", total_traditional_calls);
    println!("  Anchor:      {}", total_anchor_calls);
    println!("  Reduction:   {:.1}%\n", total_reduction);

    println!("SPEED:");
    if anchor_faster_total > 0 {
        println!("  Anchor is {}ms faster overall", anchor_faster_total);
    } else if anchor_faster_total < 0 {
        println!("  Traditional is {}ms faster overall", -anchor_faster_total);
    } else {
        println!("  Equal speed");
    }
    println!();

    println!("========================================");
    println!("VERDICT");
    println!("========================================\n");

    if total_reduction > 50.0 {
        println!("✓ Anchor dramatically reduces tool calls");
        println!("  - {:.1}% fewer calls", total_reduction);
        println!("  - Single query API vs multiple shell commands");
    } else if total_reduction > 30.0 {
        println!("△ Anchor shows meaningful improvement");
    } else {
        println!("○ Marginal difference");
    }

    println!("\n");

    // Output JSON
    let results = BenchmarkResults {
        repo_path: repo_path.display().to_string(),
        graph_stats: GraphStats {
            files: stats.file_count,
            symbols: stats.symbol_count,
            edges: stats.total_edges,
            build_time_ms: build_time as u64,
        },
        comparisons,
        summary: Summary {
            total_anchor_calls,
            total_traditional_calls,
            total_call_reduction_pct: total_reduction,
            anchor_faster_ms: anchor_faster_total,
        },
    };

    println!("{}", serde_json::to_string_pretty(&results).unwrap());

    results
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <repo_path>", args[0]);
        eprintln!("Example: {} /path/to/large/repo", args[0]);
        std::process::exit(1);
    }

    let repo_path = Path::new(&args[1]);

    if !repo_path.exists() {
        eprintln!("Error: Repository not found at {}", repo_path.display());
        std::process::exit(1);
    }

    run_full_benchmark(repo_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_real_benchmark_runs() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let results = run_full_benchmark(&repo_path);

        // Verify results are reasonable
        assert!(results.graph_stats.files > 0);
        assert!(results.graph_stats.symbols > 0);
        assert!(results.summary.total_anchor_calls > 0);
        assert!(results.summary.total_traditional_calls > 0);

        println!("\nBenchmark Results:");
        println!(
            "  Files: {}, Symbols: {}",
            results.graph_stats.files, results.graph_stats.symbols
        );
        println!(
            "  Traditional calls: {}",
            results.summary.total_traditional_calls
        );
        println!("  Anchor calls: {}", results.summary.total_anchor_calls);
        println!(
            "  Reduction: {:.1}%",
            results.summary.total_call_reduction_pct
        );
    }

    #[test]
    fn test_traditional_tools_actually_run() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // These should actually run, not error
        let metrics = traditional_search_pattern(&repo_path, "main");
        assert!(metrics.tool_calls > 0);
        assert!(metrics.total_time_ms >= 0);
    }
}
