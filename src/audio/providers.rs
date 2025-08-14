use thiserror::Error;

/// Errors that can occur during TTS audio processing and provider interaction.
#[derive(Debug, Error)]
pub enum TTSError {
    /// Missing API key for the specified provider.
    #[error("Missing API key for provider: {0}")]
    MissingApiKey(&'static str),
    /// Network error from the reqwest library.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    /// Error during audio processing.
    #[error("Audio processing error: {0}")]
    AudioProcessingError(String),
    /// Error returned by the provider.
    #[error("Provider error: {0}")]
    Provider(String),
    /// Invalid or unsupported audio format.
    #[error("Invalid audio format: {0}")]
    InvalidFormat(String),
    /// Error related to caching audio data.
    #[error("Cache error: {0}")]
    Cache(String),
    /// Error in configuration.
    #[error("Configuration error: {0}")]
    Config(String),
    /// Error returned by the API.
    #[error("API error: {0}")]
    ApiError(String),
}

pub use TTSError as Error;