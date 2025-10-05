# Lockset Vault Provider

This crate provides the foundational traits and types for integrating external secret management vaults with the Lockset Vault system. It defines a common interface that allows Lockset Vault to retrieve secrets from various providers, such as HashiCorp Vault, AWS Secrets Manager, or Azure Key Vault.

## Core Concepts

The crate is built around a few key traits and structs:

-   `VaultProvider`: This trait defines the contract for a connection to an external vault. It has a single method, `get_secret`, which retrieves a secret by name.

-   `VaultProviderFactory`: This trait is responsible for creating and validating instances of `VaultProvider`. It ensures that the configuration for a provider is valid and that a connection to the external vault can be established.

-   `ProviderSecret`: This struct represents a secret retrieved from a provider. It includes the secret's value (which is wrapped in `zeroize::Zeroizing` to securely erase it from memory when it goes out of scope) and an optional version identifier.

-   `ProviderError`: This enum defines the possible errors that can occur when interacting with a vault provider.

## Usage

To create a custom vault provider, you need to implement the `VaultProvider` and `VaultProviderFactory` traits. Here is a basic example of how you might implement a simple in-memory provider:

```rust
use async_trait::async_trait;
use lockset_vault_provider::{
    ProviderError, ProviderSecret, VaultProvider, VaultProviderFactory,
};
use std::collections::HashMap;
use zeroize::Zeroizing;

// 1. Define a struct for your provider
pub struct InMemoryProvider {
    secrets: HashMap<String, String>,
}

#[async_trait]
impl VaultProvider for InMemoryProvider {
    async fn get_secret(&self, name: &str) -> Result<ProviderSecret, ProviderError> {
        match self.secrets.get(name) {
            Some(value) => Ok(ProviderSecret {
                value: Zeroizing::new(value.clone()),
                version: None,
            }),
            None => Err(ProviderError::SecretNotFound(name.to_string())),
        }
    }
}

// 2. Define a factory for your provider
pub struct InMemoryProviderFactory;

#[async_trait]
impl VaultProviderFactory for InMemoryProviderFactory {
    async fn validate(&self, _config: &Zeroizing<String>) -> Result<(), ProviderError> {
        // For this simple provider, no validation is needed.
        Ok(())
    }

    async fn create(
        &self,
        config: Zeroizing<String>,
    ) -> Result<Box<dyn VaultProvider>, ProviderError> {
        // In a real provider, you would parse the config string.
        // For this example, we'll just create a provider with a dummy secret.
        let mut secrets = HashMap::new();
        secrets.insert("my-secret".to_string(), "my-secret-value".to_string());

        Ok(Box::new(InMemoryProvider { secrets }))
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

