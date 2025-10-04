use thiserror::Error;
use zeroize::Zeroizing;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Secret not found: {0}")]
    SecretNotFound(String),
    #[error("Provider internal error: {0}")]
    ClientError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Represents a secret retrieved from a provider.
pub struct ProviderSecret {
    pub value: Zeroizing<String>,
    pub version: Option<String>,
}

/// Defines the contract for a connection to an external vault.
#[async_trait::async_trait]
pub trait VaultProvider: Send + Sync {
    /// Retrieves a secret from the external vault.
    async fn get_secret(&self, name: &str) -> Result<ProviderSecret, ProviderError>;
}

/// Defines the contract for a factory that creates `VaultProvider` instances.
#[async_trait::async_trait]
pub trait VaultProviderFactory: Send + Sync {
    /// Validates the configuration and connectivity to the external vault.
    async fn validate(&self, config: &Zeroizing<String>) -> Result<(), ProviderError>;

    /// Creates a new `VaultProvider` instance from a configuration string.
    async fn create(&self, config: Zeroizing<String>) -> Result<Box<dyn VaultProvider>, ProviderError>;
}
