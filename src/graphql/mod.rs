//! GraphQL API for Anchor.
//!
//! Provides a flexible query interface where clients specify exactly what they need.
//!
//! ## Example
//!
//! ```graphql
//! # Minimal - just location
//! { symbol(name: "Config") { file line } }
//!
//! # With code
//! { symbol(name: "Config") { file line code } }
//!
//! # With relationships
//! { symbol(name: "Config") { file line callers { name file } } }
//! ```

pub mod mutation;
pub mod query;
pub mod schema;

use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;

use crate::graph::CodeGraph;
use mutation::Mutation;
use query::Query;

/// The Anchor GraphQL schema type
pub type AnchorSchema = Schema<Query, Mutation, EmptySubscription>;

/// Build the GraphQL schema with the code graph as context
pub fn build_schema(graph: Arc<CodeGraph>) -> AnchorSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(graph)
        .limit_depth(5) // Prevent infinite nesting
        .limit_complexity(100) // Prevent overly complex queries
        .finish()
}

/// Execute a GraphQL query and return JSON result
pub async fn execute(schema: &AnchorSchema, query: &str) -> String {
    let result = schema.execute(query).await;
    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::CodeGraph;

    #[tokio::test]
    async fn test_stats_query() {
        let graph = Arc::new(CodeGraph::new());
        let schema = build_schema(graph);

        let result = execute(&schema, "{ stats { files symbols edges } }").await;

        assert!(result.contains("files"));
        assert!(result.contains("symbols"));
        assert!(result.contains("edges"));
    }

    #[tokio::test]
    async fn test_symbol_query_empty() {
        let graph = Arc::new(CodeGraph::new());
        let schema = build_schema(graph);

        let result = execute(&schema, r#"{ symbol(name: "nonexistent") { name file } }"#).await;

        // Should return empty array, no errors
        assert!(result.contains("symbol"));
        assert!(!result.contains("error"));
    }
}
