use std::fmt::{Display, Formatter};

/// Trait that refreshes a token when it is expired
#[async_trait::async_trait]
pub trait TokenCache: Sync {
    /// Returns the token that is currently held within the instance of `TokenCache`, together with
    /// the expiry of that token as a u64 in seconds sine the Unix Epoch (1 Jan 1970).
    async fn token_and_exp(&self) -> Option<(String, u64)>;

    /// Updates the token to the value `token`.
    async fn set_token(&self, token: String, exp: u64) -> crate::Result<()>;

    /// Returns the intended scope for the current token.
    async fn scope(&self) -> String;

    /// Returns a valid, unexpired token. If the contained token is expired, it updates and returns
    /// the token.
    async fn get(&self, client: &reqwest::Client) -> crate::Result<String> {
        match self.token_and_exp().await {
            Some((token, exp)) if now() > exp => Ok(token),
            _ => {
                let (token, exp) = self.fetch_token(client).await?;
                self.set_token(token, exp).await?;

                self.token_and_exp()
                    .await
                    .map(|(t, _)| t)
                    .ok_or(crate::Error::Other("Token is not set".to_string()))
            }
        }
    }

    /// Fetches and returns the token using the service account
    async fn fetch_token(&self, client: &reqwest::Client) -> crate::Result<(String, u64)>;
}

#[derive(serde::Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(serde::Deserialize, Debug)]
// #[allow(dead_code)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    // token_type: String,
}

/// This struct contains a token, an expiry, and an access scope.
pub struct Token {
    // this field contains the JWT and the expiry thereof. They are in the same Option because if
    // one of them is `Some`, we require that the other be `Some` as well.
    token: tokio::sync::RwLock<Option<DefaultTokenData>>,
    // store the access scope for later use if we need to refresh the token
    access_scope: String,
}

#[derive(Debug, Clone)]
pub struct DefaultTokenData(String, u64);

impl Display for DefaultTokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::new("https://www.googleapis.com/auth/devstorage.full_control")
    }
}

impl Token {
    pub(crate) fn new(scope: &str) -> Self {
        Self {
            token: tokio::sync::RwLock::new(None),
            access_scope: scope.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TokenCache for Token {
    async fn scope(&self) -> String {
        self.access_scope.clone()
    }

    async fn token_and_exp(&self) -> Option<(String, u64)> {
        self.token.read().await.as_ref().map(|d| (d.0.clone(), d.1))
    }

    async fn set_token(&self, token: String, exp: u64) -> crate::Result<()> {
        *self.token.write().await = Some(DefaultTokenData(token, exp));
        Ok(())
    }

    async fn fetch_token(&self, client: &reqwest::Client) -> crate::Result<(String, u64)> {
        let now = now();
        let exp = now + 3600;

        let claims = Claims {
            iss: crate::SERVICE_ACCOUNT.client_email.clone(),
            scope: self.scope().await.into(),
            aud: "https://www.googleapis.com/oauth2/v4/token".to_string(),
            exp,
            iat: now,
        };
        let header = jsonwebtoken::Header {
            alg: jsonwebtoken::Algorithm::RS256,
            ..Default::default()
        };
        let private_key_bytes = crate::SERVICE_ACCOUNT.private_key.as_bytes();
        let private_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key_bytes)?;
        let jwt = jsonwebtoken::encode(&header, &claims, &private_key)?;
        let body = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];
        let response: TokenResponse = client
            .post("https://www.googleapis.com/oauth2/v4/token")
            .form(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok((response.access_token, now + response.expires_in))
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
