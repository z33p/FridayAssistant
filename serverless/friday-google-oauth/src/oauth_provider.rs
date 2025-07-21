use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OAuthProvider {
    Google,
    Microsoft,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::Microsoft => write!(f, "microsoft"),
        }
    }
}

pub trait OAuthClientProvider {
    fn create_client(&self) -> Result<BasicClient, Box<dyn Error>>;
    fn get_auth_scopes(&self) -> Vec<Scope>;
    fn get_additional_auth_params(&self) -> Vec<(&'static str, &'static str)>;
    fn get_additional_token_params(&self) -> Vec<(&'static str, &'static str)>;

    #[allow(dead_code)]
    fn provider_name(&self) -> &'static str;
}

pub struct GoogleOAuthProvider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

pub struct MicrosoftOAuthProvider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

impl OAuthClientProvider for GoogleOAuthProvider {
    fn create_client(&self) -> Result<BasicClient, Box<dyn Error>> {
        let client_id = ClientId::new(self.client_id.clone());
        let client_secret = ClientSecret::new(self.client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
        let redirect_url = RedirectUrl::new(self.redirect_url.clone())?;

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Ok(client)
    }

    fn get_auth_scopes(&self) -> Vec<Scope> {
        vec![Scope::new("https://mail.google.com/".to_string())]
    }

    fn get_additional_auth_params(&self) -> Vec<(&'static str, &'static str)> {
        vec![("access_type", "offline")]
    }

    fn get_additional_token_params(&self) -> Vec<(&'static str, &'static str)> {
        vec![("access_type", "offline")]
    }

    fn provider_name(&self) -> &'static str {
        "google"
    }
}

impl OAuthClientProvider for MicrosoftOAuthProvider {
    fn create_client(&self) -> Result<BasicClient, Box<dyn Error>> {
        let client_id = ClientId::new(self.client_id.clone());
        let client_secret = ClientSecret::new(self.client_secret.clone());
        let auth_url = AuthUrl::new(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        )?;
        let token_url = TokenUrl::new(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        )?;
        let redirect_url = RedirectUrl::new(self.redirect_url.clone())?;

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Ok(client)
    }

    fn get_auth_scopes(&self) -> Vec<Scope> {
        vec![
            Scope::new("offline_access".to_string()),
            Scope::new("https://graph.microsoft.com/Mail.ReadWrite".to_string()),
        ]
    }

    fn get_additional_auth_params(&self) -> Vec<(&'static str, &'static str)> {
        vec![] // Microsoft doesn't need additional auth params
    }

    fn get_additional_token_params(&self) -> Vec<(&'static str, &'static str)> {
        vec![] // Microsoft doesn't need additional token params
    }

    fn provider_name(&self) -> &'static str {
        "microsoft"
    }
}

pub struct OAuthProviderFactory;

impl OAuthProviderFactory {
    pub fn create_provider(
        provider: &OAuthProvider,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Box<dyn OAuthClientProvider> {
        match provider {
            OAuthProvider::Google => Box::new(GoogleOAuthProvider {
                client_id,
                client_secret,
                redirect_url,
            }),
            OAuthProvider::Microsoft => Box::new(MicrosoftOAuthProvider {
                client_id,
                client_secret,
                redirect_url,
            }),
        }
    }
}
