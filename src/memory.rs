//! Memory system for Oxyde agents
//!
//! This module provides memory storage and retrieval capabilities for agents.
//! Memories are used to provide context for NPC responses and actions.
//! 
//! The memory system supports both keyword-based and vector-based retrieval,
//! with features for short-term and long-term memory management.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[cfg(feature = "vector-memory")]
use hnswlib::Hnsw;

use crate::config::MemoryConfig;

#[cfg(feature = "vector-memory")]
use crate::config::EmbeddingModelType;
use crate::{OxydeError, Result};

/// Embedding model for vector representations of text
#[cfg(feature = "vector-memory")]
pub trait EmbeddingModel {
    /// Generate embedding vector for text
    fn embed(&self, text: &str) -> Result<Vec<f32>>;
    
    /// Get the dimension of the embedding vectors
    fn dimension(&self) -> usize;
}

/// Simple embedding model implementation using MiniLM
#[cfg(feature = "vector-memory")]
pub struct MiniLMEmbedding {
    /// The model used for generating embeddings
    model: rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel,
    
    /// Dimension of the embedding vectors
    dimension: usize,
}

#[cfg(feature = "vector-memory")]
impl MiniLMEmbedding {
    /// Create a new MiniLM embedding model
    pub fn new() -> Result<Self> {
        use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
        
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .map_err(|e| OxydeError::MemoryError(format!("Failed to load embedding model: {}", e)))?;
        
        Ok(Self {
            dimension: 384, // MiniLM L12 V2 has 384-dimensional embeddings
            model,
        })
    }
}

#[cfg(feature = "vector-memory")]
impl EmbeddingModel for MiniLMEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.encode(&[text])
            .map_err(|e| OxydeError::MemoryError(format!("Failed to generate embedding: {}", e)))?;
        
        if embeddings.is_empty() {
            return Err(OxydeError::MemoryError("Empty embedding generated".to_string()));
        }
        
        // Convert from Vec<f64> to Vec<f32>
        let embedding: Vec<f32> = embeddings[0].iter().map(|&x| x as f32).collect();
        
        Ok(embedding)
    }
    
    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Memory category for different types of memories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryCategory {
    /// Episodic memories (events, experiences)
    Episodic,
    /// Semantic memories (facts, knowledge)
    Semantic,
    /// Procedural memories (how to do things)
    Procedural,
    /// Emotional memories (feelings, reactions)
    Emotional,
}

impl MemoryCategory {
    /// Convert from string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "episodic" => Some(Self::Episodic),
            "semantic" => Some(Self::Semantic),
            "procedural" => Some(Self::Procedural),
            "emotional" => Some(Self::Emotional),
            _ => None,
        }
    }
    
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
            Self::Procedural => "procedural",
            Self::Emotional => "emotional",
        }
    }
}

/// Memory represents a single piece of information that an agent remembers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier for the memory
    pub id: String,
    
    /// Memory category (episodic, semantic, etc.)
    pub category: MemoryCategory,
    
    /// Memory subcategory/tag for more specific classification
    pub tags: Vec<String>,
    
    /// Content of the memory
    pub content: String,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last accessed timestamp
    pub last_accessed: u64,
    
    /// Access count - how many times this memory has been recalled
    pub access_count: u32,
    
    /// Importance score (0.0 - 1.0)
    pub importance: f64,
    
    /// Emotional valence (-1.0 to 1.0, negative to positive)
    pub emotional_valence: f64,
    
    /// Emotional intensity (0.0 to 1.0)
    pub emotional_intensity: f64,
    
    /// Whether the memory is permanent (won't be forgotten)
    pub permanent: bool,
    
    /// Vector embedding of the memory content (for semantic search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
}

impl Memory {
    /// Create a new memory
    ///
    /// # Arguments
    ///
    /// * `category` - Category of the memory (episodic, semantic, etc.)
    /// * `content` - Content of the memory
    /// * `importance` - Importance score (0.0 - 1.0)
    /// * `tags` - Optional tags for the memory
    ///
    /// # Returns
    ///
    /// A new Memory instance
    pub fn new(category: MemoryCategory, content: &str, importance: f64, tags: Option<Vec<String>>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let permanent = importance >= 1.0;
        
        Self {
            id: Uuid::new_v4().to_string(),
            category,
            tags: tags.unwrap_or_default(),
            content: content.to_string(),
            created_at: now,
            last_accessed: now,
            access_count: 0,
            importance: importance.clamp(0.0, 1.0),
            emotional_valence: 0.0,
            emotional_intensity: 0.0,
            permanent,
            embedding: None,
        }
    }
    
    /// Create a new memory with emotional content
    ///
    /// # Arguments
    ///
    /// * `category` - Category of the memory (episodic, semantic, etc.)
    /// * `content` - Content of the memory
    /// * `importance` - Importance score (0.0 - 1.0)
    /// * `valence` - Emotional valence (-1.0 to 1.0)
    /// * `intensity` - Emotional intensity (0.0 to 1.0)
    /// * `tags` - Optional tags for the memory
    ///
    /// # Returns
    ///
    /// A new Memory instance with emotional data
    pub fn new_emotional(
        category: MemoryCategory, 
        content: &str, 
        importance: f64,
        valence: f64,
        intensity: f64,
        tags: Option<Vec<String>>
    ) -> Self {
        let mut memory = Self::new(category, content, importance, tags);
        memory.emotional_valence = valence.clamp(-1.0, 1.0);
        memory.emotional_intensity = intensity.clamp(0.0, 1.0);
        memory
    }
    
    /// Update the last accessed time and increment access count
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        self.access_count += 1;
    }
    
    /// Calculate the relevance of this memory to a query
    ///
    /// # Arguments
    ///
    /// * `query` - Query text to check relevance against
    /// * `query_embedding` - Optional vector embedding of the query for semantic search
    ///
    /// # Returns
    ///
    /// Relevance score (0.0 - 1.0)
    pub fn relevance(&self, query: &str, query_embedding: Option<&[f32]>) -> f64 {
        // If we have embeddings for both the query and the memory, use vector similarity
        if let (Some(query_vec), Some(memory_vec)) = (query_embedding, &self.embedding) {
            if query_vec.len() == memory_vec.len() {
                // Cosine similarity calculation
                let mut dot_product = 0.0;
                let mut query_magnitude = 0.0;
                let mut memory_magnitude = 0.0;
                
                for i in 0..query_vec.len() {
                    dot_product += query_vec[i] as f64 * memory_vec[i] as f64;
                    query_magnitude += (query_vec[i] as f64).powi(2);
                    memory_magnitude += (memory_vec[i] as f64).powi(2);
                }
                
                query_magnitude = query_magnitude.sqrt();
                memory_magnitude = memory_magnitude.sqrt();
                
                if query_magnitude > 0.0 && memory_magnitude > 0.0 {
                    let cosine_similarity = dot_product / (query_magnitude * memory_magnitude);
                    // Apply importance and recency bias
                    return (cosine_similarity * 0.7 + self.importance * 0.3)
                        .clamp(0.0, 1.0);
                }
            }
        }
        
        // Fallback to keyword-based relevance if embeddings aren't available
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        
        let content_lower = self.content.to_lowercase();
        let content_words: Vec<&str> = content_lower.split_whitespace().collect();
        
        // Check for tag matches to improve relevance
        let tag_match_bonus = self.tags.iter()
            .filter(|tag| query_lower.contains(&tag.to_lowercase()))
            .count() as f64 * 0.1;
        
        // Check for content word matches
        let mut matches = 0;
        for qw in &query_words {
            if content_words.iter().any(|cw| cw.contains(qw)) {
                matches += 1;
            }
        }
        
        if query_words.is_empty() {
            tag_match_bonus.min(1.0) // Just use tag bonus if query is empty
        } else {
            let word_match_score = matches as f64 / query_words.len() as f64;
            
            // Combine word matching with tag bonus and importance
            let relevance_score = (word_match_score * 0.6 + self.importance * 0.3 + tag_match_bonus)
                .clamp(0.0, 1.0);
            
            // Apply a small emotional intensity bonus for emotional memories
            if self.category == MemoryCategory::Emotional && self.emotional_intensity > 0.5 {
                (relevance_score * 1.2).min(1.0)
            } else {
                relevance_score
            }
        }
    }
    
    /// Set the vector embedding for this memory
    ///
    /// # Arguments
    ///
    /// * `embedding` - Vector embedding to set
    pub fn set_embedding(&mut self, embedding: Vec<f32>) {
        self.embedding = Some(embedding);
    }
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Memory {}

impl PartialOrd for Memory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Memory {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by importance for the binary heap
        self.importance.partial_cmp(&other.importance)
            .unwrap_or(Ordering::Equal)
            .reverse() // Use reverse for a max-heap
    }
}

/// Memory system for storing and retrieving agent memories
#[derive(Debug)]
pub struct MemorySystem {
    /// Configuration for the memory system
    config: MemoryConfig,
    
    /// Stored memories - includes both short-term and long-term
    memories: RwLock<Vec<Memory>>,
    
    /// Embedding model for vector-based memory retrieval
    #[cfg(feature = "vector-memory")]
    embedding_model: Option<Arc<RwLock<dyn EmbeddingModel + Send + Sync>>>,
}

impl MemorySystem {
    /// Create a new memory system with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Memory system configuration
    ///
    /// # Returns
    ///
    /// A new MemorySystem instance
    pub fn new(config: MemoryConfig) -> Self {
        #[cfg(feature = "vector-memory")]
        let embedding_model = if config.use_embeddings {
            None // Will be initialized lazily when needed
        } else {
            None
        };
        
        #[cfg(feature = "vector-memory")]
        return Self {
            config,
            memories: RwLock::new(Vec::new()),
            embedding_model,
        };
        
        #[cfg(not(feature = "vector-memory"))]
        return Self {
            config,
            memories: RwLock::new(Vec::new()),
        };
    }
    
    /// Initialize the embedding model for vector memory
    ///
    /// This is called lazily the first time vector embeddings are needed.
    #[cfg(feature = "vector-memory")]
    async fn ensure_embedding_model(&self) -> Result<()> {
        if self.config.use_embeddings {
            let mut model_opt = None;
            
            if let Some(model) = &self.embedding_model {
                // Model already initialized
                return Ok(());
            }
            
            // Initialize the appropriate model based on configuration
            match self.config.embedding_model {
                EmbeddingModelType::MiniBert => {
                    let model = MiniLMEmbedding::new()?;
                    model_opt = Some(Arc::new(RwLock::new(model)) as Arc<RwLock<dyn EmbeddingModel + Send + Sync>>);
                },
                EmbeddingModelType::DistilBert => {
                    // Could implement other models here
                    return Err(OxydeError::MemoryError("DistilBert model not yet implemented".to_string()));
                },
                EmbeddingModelType::Custom => {
                    if let Some(path) = &self.config.custom_model_path {
                        // Custom model loading would go here
                        return Err(OxydeError::MemoryError("Custom models not yet supported".to_string()));
                    } else {
                        return Err(OxydeError::MemoryError("Custom model path not specified".to_string()));
                    }
                }
            }
            
            // Update the model
            if let Some(model) = model_opt {
                let mut embed_model = unsafe {
                    // This is safe because we're replacing Option<T> with Some(T)
                    &mut *(&self.embedding_model as *const _ as *mut Option<Arc<RwLock<dyn EmbeddingModel + Send + Sync>>>)
                };
                *embed_model = Some(model);
            }
        }
        
        Ok(())
    }
    
    #[cfg(feature = "vector-memory")]
    async fn generate_embedding(&self, text: &str) -> Result<Option<Vec<f32>>> {
        if !self.config.use_embeddings {
            return Ok(None);
        }
        
        // Ensure model is initialized
        self.ensure_embedding_model().await?;
        
        if let Some(model) = &self.embedding_model {
            let model = model.read().await;
            let embedding = model.embed(text)?;
            Ok(Some(embedding))
        } else {
            Ok(None)
        }
    }
    
    /// Add a memory to the system
    ///
    /// # Arguments
    ///
    /// * `memory` - Memory to add
    ///
    /// # Returns
    ///
    /// Success or error
    pub async fn add(&self, memory: Memory) -> Result<()> {
        // Generate embedding for the memory if vector embeddings are enabled
        #[cfg(feature = "vector-memory")]
        if self.config.use_embeddings && memory.embedding.is_none() {
            if let Some(embedding) = self.generate_embedding(&memory.content).await? {
                memory.embedding = Some(embedding);
            }
        }
        
        let mut memories = self.memories.write().await;
        
        // Check if we need to remove a memory to stay under capacity
        if !memory.permanent && memories.len() >= self.config.capacity {
            // First try to remove a memory with the same category if we have too many
            let category_count = memories.iter()
                .filter(|m| m.category == memory.category && !m.permanent)
                .count();
                
            if category_count > self.config.capacity / 4 {  // Don't let one category take up more than 25% of memory
                // Find the least important non-permanent memory of the same category
                if let Some(index) = memories.iter()
                    .enumerate()
                    .filter(|(_, m)| !m.permanent && m.category == memory.category)
                    .min_by(|(_, a), (_, b)| {
                        // Consider both importance and access frequency
                        let a_score = a.importance * (1.0 + a.access_count as f64 / 10.0);
                        let b_score = b.importance * (1.0 + b.access_count as f64 / 10.0);
                        a_score.partial_cmp(&b_score).unwrap_or(Ordering::Equal)
                    })
                    .map(|(i, _)| i)
                {
                    memories.remove(index);
                    memories.push(memory);
                    return Ok(());
                }
            }
            
            // Otherwise find the least important non-permanent memory overall
            if let Some(index) = memories.iter()
                .enumerate()
                .filter(|(_, m)| !m.permanent)
                .min_by(|(_, a), (_, b)| {
                    // Consider both importance and access frequency
                    let a_score = a.importance * (1.0 + a.access_count as f64 / 10.0);
                    let b_score = b.importance * (1.0 + b.access_count as f64 / 10.0);
                    a_score.partial_cmp(&b_score).unwrap_or(Ordering::Equal)
                })
                .map(|(i, _)| i)
            {
                memories.remove(index);
            } else {
                return Err(OxydeError::MemoryError(
                    "Memory capacity reached and all memories are permanent".to_string()
                ));
            }
        }
        
        memories.push(memory);
        Ok(())
    }
    
    /// Retrieve a memory by ID
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the memory to retrieve
    ///
    /// # Returns
    ///
    /// The memory if found, or None
    pub async fn get(&self, id: &str) -> Option<Memory> {
        let mut memories = self.memories.write().await;
        
        if let Some(index) = memories.iter().position(|m| m.id == id) {
            let mut memory = memories[index].clone();
            memory.touch();
            memories[index] = memory.clone();
            Some(memory)
        } else {
            None
        }
    }
    
    /// Retrieve memories by category
    ///
    /// # Arguments
    ///
    /// * `category` - Category of memories to retrieve
    ///
    /// # Returns
    ///
    /// Vector of matching memories
    pub async fn get_by_category(&self, category: MemoryCategory) -> Vec<Memory> {
        let mut memories = self.memories.write().await;
        
        let result: Vec<Memory> = memories.iter()
            .filter(|m| m.category == category)
            .cloned()
            .collect();
        
        // Update last_accessed for retrieved memories
        for memory in &result {
            if let Some(index) = memories.iter().position(|m| m.id == memory.id) {
                let mut updated = memories[index].clone();
                updated.touch();
                memories[index] = updated;
            }
        }
        
        result
    }
    
    /// Retrieve memories by tag
    ///
    /// # Arguments
    ///
    /// * `tag` - Tag to search for
    ///
    /// # Returns
    ///
    /// Vector of matching memories
    pub async fn get_by_tag(&self, tag: &str) -> Vec<Memory> {
        let mut memories = self.memories.write().await;
        
        let result: Vec<Memory> = memories.iter()
            .filter(|m| m.tags.iter().any(|t| t == tag))
            .cloned()
            .collect();
        
        // Update last_accessed for retrieved memories
        for memory in &result {
            if let Some(index) = memories.iter().position(|m| m.id == memory.id) {
                let mut updated = memories[index].clone();
                updated.touch();
                memories[index] = updated;
            }
        }
        
        result
    }
    
    /// Retrieve memories most relevant to a query
    ///
    /// # Arguments
    ///
    /// * `query` - Query to find relevant memories for
    /// * `limit` - Maximum number of memories to return
    /// * `query_embedding` - Optional vector embedding of the query for semantic search
    ///
    /// # Returns
    ///
    /// Vector of relevant memories, sorted by relevance
    pub async fn retrieve_relevant(&self, query: &str, limit: usize, query_embedding: Option<&[f32]>) -> Result<Vec<Memory>> {
        let mut memories = self.memories.write().await;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        // Check if we should prioritize certain categories of memories
        let has_priority_categories = !self.config.priority_categories.is_empty();
        
        // Define a custom struct for scored memories that implements Ord
        #[derive(Debug, Clone, PartialEq)]
        struct ScoredMemory {
            score: f64,
            memory: Memory,
            category_priority_bonus: f64,
        }
        
        // Implement comparison traits for ScoredMemory
        impl PartialOrd for ScoredMemory {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Include the category priority bonus in the comparison
                let self_score = self.score + self.category_priority_bonus;
                let other_score = other.score + other.category_priority_bonus;
                self_score.partial_cmp(&other_score)
            }
        }
        
        impl Eq for ScoredMemory {}
        
        impl Ord for ScoredMemory {
            fn cmp(&self, other: &Self) -> Ordering {
                // Guaranteed to work since f64 always implements partial_cmp
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }
        
        // Calculate relevance scores and apply time decay
        let mut scored_memories: BinaryHeap<ScoredMemory> = BinaryHeap::new();
        
        for memory in memories.iter() {
            // Apply recency bias based on access count and last access time
            let recency_factor = if memory.access_count > 0 {
                // Frequently accessed memories are more relevant
                let access_frequency = (memory.access_count as f64).min(10.0) / 10.0;
                // Recently accessed memories are more relevant
                let last_access_age = now.saturating_sub(memory.last_accessed) as f64;
                let last_access_factor = (-self.config.decay_rate * (last_access_age / 86400.0)).exp();
                
                0.7 + (0.3 * access_frequency * last_access_factor)
            } else {
                1.0 // No recency bias for memories that haven't been accessed
            };
            
            // Calculate time decay factor (1.0 for new memories, approaches 0 for old ones)
            let age_seconds = now.saturating_sub(memory.created_at);
            let decay_factor = if memory.permanent {
                1.0
            } else {
                (-self.config.decay_rate * (age_seconds as f64 / 86400.0)).exp() // 86400 seconds in a day
            };
            
            // Calculate relevance using the enhanced relevance function with embeddings
            let relevance = memory.relevance(query, query_embedding) * decay_factor * recency_factor;
            
            // Calculate category priority bonus
            let category_priority_bonus = if has_priority_categories {
                if self.config.priority_categories.iter().any(|c| {
                    MemoryCategory::from_str(c).map_or(false, |cat| cat == memory.category)
                }) {
                    0.2 // Boost priority categories
                } else {
                    0.0
                }
            } else {
                0.0
            };
            
            // Add to heap if above threshold
            if relevance >= self.config.importance_threshold {
                scored_memories.push(ScoredMemory {
                    score: relevance,
                    memory: memory.clone(),
                    category_priority_bonus,
                });
            }
        }
        
        // Extract top memories
        let mut result = Vec::with_capacity(limit);
        
        // Keep track of short-term and long-term memories
        let mut short_term_count = 0;
        
        for _ in 0..limit {
            if let Some(scored_memory) = scored_memories.pop() {
                // Check if we've already reached the short-term memory limit
                let is_short_term = now.saturating_sub(scored_memory.memory.created_at) < 3600; // Less than 1 hour old
                
                if is_short_term && short_term_count >= self.config.short_term_capacity {
                    // Skip this short-term memory if we've reached the limit, unless it's very important
                    if scored_memory.memory.importance < 0.8 {
                        continue;
                    }
                }
                
                if is_short_term {
                    short_term_count += 1;
                }
                
                // Update last_accessed for this memory
                if let Some(index) = memories.iter().position(|m| m.id == scored_memory.memory.id) {
                    let mut updated = memories[index].clone();
                    updated.touch();
                    memories[index] = updated;
                }
                
                result.push(scored_memory.memory);
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    /// Forget a memory
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the memory to forget
    ///
    /// # Returns
    ///
    /// Success or error
    pub async fn forget(&self, id: &str) -> Result<()> {
        let mut memories = self.memories.write().await;
        
        if let Some(index) = memories.iter().position(|m| m.id == id) {
            if memories[index].permanent {
                return Err(OxydeError::MemoryError(
                    "Cannot forget a permanent memory".to_string()
                ));
            }
            
            memories.remove(index);
            Ok(())
        } else {
            Err(OxydeError::MemoryError(
                format!("Memory with ID {} not found", id)
            ))
        }
    }
    
    /// Forget memories of a certain category
    ///
    /// # Arguments
    ///
    /// * `category` - Category of memories to forget
    ///
    /// # Returns
    ///
    /// Number of memories forgotten
    pub async fn forget_by_category(&self, category: MemoryCategory) -> usize {
        let mut memories = self.memories.write().await;
        
        let initial_len = memories.len();
        memories.retain(|m| m.category != category || m.permanent);
        
        initial_len - memories.len()
    }
    
    /// Forget memories with a specific tag
    ///
    /// # Arguments
    ///
    /// * `tag` - Tag of memories to forget
    ///
    /// # Returns
    ///
    /// Number of memories forgotten
    pub async fn forget_by_tag(&self, tag: &str) -> usize {
        let mut memories = self.memories.write().await;
        
        let initial_len = memories.len();
        memories.retain(|m| !m.tags.contains(&tag.to_string()) || m.permanent);
        
        initial_len - memories.len()
    }
    
    /// Clear all non-permanent memories
    ///
    /// # Returns
    ///
    /// Number of memories cleared
    pub async fn clear(&self) -> usize {
        let mut memories = self.memories.write().await;
        
        let initial_len = memories.len();
        memories.retain(|m| m.permanent);
        
        initial_len - memories.len()
    }
    
    /// Get the total number of memories
    ///
    /// # Returns
    ///
    /// Total number of memories
    pub async fn count(&self) -> usize {
        self.memories.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_memory_creation() {
        let memory = Memory::new(MemoryCategory::Semantic, "Test content", 0.5, None);
        assert_eq!(memory.category, MemoryCategory::Semantic);
        assert_eq!(memory.content, "Test content");
        assert_eq!(memory.importance, 0.5);
        assert_eq!(memory.permanent, false);
        assert_eq!(memory.access_count, 0);
    }
    
    #[tokio::test]
    async fn test_memory_system() {
        let config = MemoryConfig {
            capacity: 3,
            persistence: false,
            decay_rate: 0.05,
            importance_threshold: 0.2,
            short_term_capacity: 5,
            use_embeddings: false,
            embedding_model: EmbeddingModelType::MiniBert,
            custom_model_path: None,
            embedding_dimension: 384,
            priority_categories: Vec::new(),
        };
        
        let system = MemorySystem::new(config);
        
        // Add memories
        system.add(Memory::new(MemoryCategory::Semantic, "The sky is blue", 0.5, Some(vec!["fact".to_string()]))).await.unwrap();
        system.add(Memory::new(MemoryCategory::Semantic, "Grass is green", 0.3, Some(vec!["fact".to_string()]))).await.unwrap();
        system.add(Memory::new(MemoryCategory::Semantic, "Water is wet", 0.7, Some(vec!["fact".to_string()]))).await.unwrap();
        
        // Test count
        assert_eq!(system.count().await, 3);
        
        // Test get by category
        let facts = system.get_by_category(MemoryCategory::Semantic).await;
        assert_eq!(facts.len(), 3);
        
        // Test get by tag
        let facts_by_tag = system.get_by_tag("fact").await;
        assert_eq!(facts_by_tag.len(), 3);
        
        // Test relevant retrieval
        let relevant = system.retrieve_relevant("sky color", 2, None).await.unwrap();
        assert_eq!(relevant.len(), 1);
        assert!(relevant[0].content.contains("sky"));
        
        // Test memory limit
        system.add(Memory::new(MemoryCategory::Semantic, "Fire is hot", 0.6, Some(vec!["fact".to_string()]))).await.unwrap();
        assert_eq!(system.count().await, 3); // Still 3 due to capacity limit
    }
}
