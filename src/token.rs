use std::fmt::{Display, Formatter};

/// Trait that refreshes a token when it is expired
#[async_trait::async_trait]
pub trait TokenCache: Sync {
    type TokenData: Sync + Send + Clone + Display;
    /// Getter for the token
    async fn get_token(&self) -> Option<Self::TokenData>;

    /// Updates the token
    async fn set_token(&self, token: Self::TokenData) -> crate::Result<()>;

    /// Getter for the scope
    fn get_scope(&self) -> &str;

    /// Returns whether the token is expired
    fn is_expired(token: &Self::TokenData) -> bool;

    /// Returns a valid, unexpired token
    /// Otherwise updates and returns the token
    async fn get(&self, client: &reqwest::Client) -> crate::Result<Self::TokenData> {
        match self.get_token().await {
            Some(token) if !Self::is_expired(&token) => Ok(token),
            _ => {
                let scope = self.get_scope();
                let token = Self::fetch_token(client, scope).await?;
                self.set_token(token).await?;

                self.get_token()
                    .await
                    .ok_or(crate::Error::Other("Token is not set".to_string()))
            }
        }
    }

    /// Fetches and returns the token using the service account
    async fn fetch_token(client: &reqwest::Client, scope: &str) -> crate::Result<Self::TokenData>;
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
struct TokenResponse {
    access_token: String,
    expires_in: usize,
    token_type: String,
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
pub struct DefaultTokenData(pub String, u64);

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
    pub fn new(scope: &str) -> Self {
        Self {
            token: tokio::sync::RwLock::new(None),
            access_scope: scope.to_string(),
        }
    }
}
#[async_trait::async_trait]
impl TokenCache for Token {
    type TokenData = DefaultTokenData;

    fn get_scope(&self) -> &str {
        self.access_scope.as_ref()
    }

    fn is_expired(token: &Self::TokenData) -> bool {
        token.1 > now()
    }

    async fn get_token(&self) -> Option<Self::TokenData> {
        self.token.read().await.clone()
    }
    async fn set_token(&self, token: Self::TokenData) -> crate::Result<()> {
        *self.token.write().await = Some(token);
        Ok(())
    }

    async fn fetch_token(client: &reqwest::Client, scope: &str) -> crate::Result<Self::TokenData> {
        let now = now();
        let exp = now + 3600;

        let claims = Claims {
            iss: crate::SERVICE_ACCOUNT.client_email.clone(),
            scope: scope.into(),
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
        Ok(DefaultTokenData(response.access_token, exp))
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
