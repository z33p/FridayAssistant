#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth_provider::{OAuthProvider, OAuthProviderFactory};

    #[test]
    fn test_provider_creation() {
        let google_provider = OAuthProviderFactory::create_provider(
            &OAuthProvider::Google,
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost/".to_string(),
        );

        let microsoft_provider = OAuthProviderFactory::create_provider(
            &OAuthProvider::Microsoft,
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost/".to_string(),
        );

        // Test that providers are created successfully
        assert!(google_provider.create_client().is_ok());
        assert!(microsoft_provider.create_client().is_ok());
    }

    #[test]
    fn test_provider_scopes() {
        let google_provider = OAuthProviderFactory::create_provider(
            &OAuthProvider::Google,
            "test".to_string(),
            "test".to_string(),
            "http://localhost/".to_string(),
        );

        let microsoft_provider = OAuthProviderFactory::create_provider(
            &OAuthProvider::Microsoft,
            "test".to_string(),
            "test".to_string(),
            "http://localhost/".to_string(),
        );

        // Test that different providers have different scopes
        let google_scopes = google_provider.get_auth_scopes();
        let microsoft_scopes = microsoft_provider.get_auth_scopes();

        assert_eq!(google_scopes.len(), 1);
        assert_eq!(microsoft_scopes.len(), 2); // offline_access + Mail.ReadWrite

        // Verify specific scopes
        assert!(google_scopes[0].to_string().contains("mail.google.com"));
        assert!(microsoft_scopes.iter().any(|scope| scope.to_string().contains("offline_access")));
        assert!(microsoft_scopes.iter().any(|scope| scope.to_string().contains("Mail.ReadWrite")));
    }

    #[test]
    fn test_oauth_provider_display() {
        assert_eq!(OAuthProvider::Google.to_string(), "google");
        assert_eq!(OAuthProvider::Microsoft.to_string(), "microsoft");
    }

    #[test]
    fn test_oauth_provider_serialization() {
        let google = OAuthProvider::Google;
        let microsoft = OAuthProvider::Microsoft;

        // Test that serialization works (important for JSON APIs)
        let google_json = serde_json::to_string(&google).unwrap();
        let microsoft_json = serde_json::to_string(&microsoft).unwrap();

        assert_eq!(google_json, "\"GOOGLE\"");
        assert_eq!(microsoft_json, "\"MICROSOFT\"");

        // Test deserialization
        let google_deserialized: OAuthProvider = serde_json::from_str(&google_json).unwrap();
        let microsoft_deserialized: OAuthProvider = serde_json::from_str(&microsoft_json).unwrap();

        assert!(matches!(google_deserialized, OAuthProvider::Google));
        assert!(matches!(microsoft_deserialized, OAuthProvider::Microsoft));
    }
}
