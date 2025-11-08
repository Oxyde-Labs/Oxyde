//! Utility functions for the Oxyde SDK
//!
//! This module provides various utility functions used across the SDK.

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, Ordering};

// Counter to ensure uniqueness even when called rapidly
#[allow(dead_code)]
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique ID using the current timestamp
///
/// # Returns
///
/// A string containing a unique ID
#[allow(dead_code)]
pub fn generate_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    
    format!("oxid-{}-{}", timestamp, counter)
}

/// Returns the current timestamp in milliseconds
///
/// # Returns
///
/// The current time in milliseconds since the Unix epoch
#[allow(dead_code)]
pub fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

/// Calculate the relevance score for a memory based on its content and a query
///
/// # Arguments
///
/// * `memory_content` - The content of the memory
/// * `query` - The query to calculate relevance for
///
/// # Returns
///
/// A relevance score between 0.0 and 1.0
#[allow(dead_code)]
pub fn calculate_relevance(memory_content: &str, query: &str) -> f64 {
    // This is a simple implementation for demonstration purposes
    // In a real implementation, this would use a more sophisticated algorithm
    
    // Convert to lowercase for case-insensitive matching
    let memory_lower = memory_content.to_lowercase();
    let query_lower = query.to_lowercase();
    
    // Split into words
    let memory_words: Vec<&str> = memory_lower.split_whitespace().collect();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    
    // Count how many query words appear in the memory
    let mut matches = 0;
    for query_word in &query_words {
        if memory_words.contains(query_word) {
            matches += 1;
        }
    }
    
    // Calculate score based on percentage of matching words
    if query_words.is_empty() {
        0.0
    } else {
        matches as f64 / query_words.len() as f64
    }
}

/// Truncate a string to a maximum length, ending with ellipsis if truncated
///
/// # Arguments
///
/// * `s` - The string to truncate
/// * `max_len` - The maximum length
///
/// # Returns
///
/// The truncated string
#[allow(dead_code)]
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find a good breaking point (ideally at a word boundary)
        let mut truncate_at = max_len - 3; // Leave room for ellipsis
        while truncate_at > 0 && !s.is_char_boundary(truncate_at) {
            truncate_at -= 1;
        }
        
        let mut result = s[0..truncate_at].to_string();
        result.push_str("...");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        
        assert!(id1.starts_with("oxid-"));
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }
    
    #[test]
    fn test_calculate_relevance() {
        let memory = "The player character found a rusty sword in the cave";
        
        // High relevance
        let query1 = "find sword cave";
        let score1 = calculate_relevance(memory, query1);
        
        // Low relevance
        let query2 = "craft potion magic";
        let score2 = calculate_relevance(memory, query2);
        
        assert!(score1 > 0.6, "Score should be reasonably high for relevant query");
        assert!(score2 < 0.1, "Score should be low for irrelevant query");
        assert!(score1 > score2, "Relevant query should score higher");
    }
    
    #[test]
    fn test_truncate_string() {
        let orig = "This is a very long string that needs to be truncated";
        let truncated = truncate_string(orig, 20);
        
        assert_eq!(truncated.len(), 20);
        assert!(truncated.ends_with("..."));
    }
}