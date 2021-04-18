use lazy_static::__Deref;

/// This struct contains a token, an expiry, and an access scope.
pub struct Token {
    // this field contains the JWT and the expiry thereof. They are in the same Option because if
    // one of them is `Some`, we require that the other be `Some` as well.
    token: tokio::sync::RwLock<Option<(String, u64)>>,
    // store the access scope for later use if we need to refresh the token
    access_scope: String,
}

#[async_trait::async_trait]
pub trait RefreshableToken: Sync {
    async fn get_inner_token(&self) -> Option<(String, u64)>;
    async fn update_inner_token(&self, token: (String, u64)) -> crate::Result<()>;
    async fn get_scope(&self) -> &str;

    async fn get(&self, client: &reqwest::Client) -> crate::Result<String> {
        match self.get_inner_token().await {
            Some((token, exp)) if exp > now() => Ok(token),
            _ => {
                let scope = self.get_scope().await;
                let token = Self::fetch_token(client, scope).await?;
                self.update_inner_token(token).await?;

                Ok(self.get_inner_token().await.unwrap().0)
            }
        }
    }

    async fn fetch_token(client: &reqwest::Client, scope: &str) -> crate::Result<(String, u64)> {
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
        Ok((response.access_token, exp))
    }
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

impl Token {
    pub fn new(scope: &str) -> Self {
        Self {
            token: tokio::sync::RwLock::new(None),
            access_scope: scope.to_string(),
        }
    }

    // TODO: should not need to use mem::take and then place back when the token is valid
    pub async fn get(&self, client: &reqwest::Client) -> crate::Result<String> {
        match self.token.read().await.as_ref() {
            Some((token, exp)) if *exp > now() => Ok(token.to_owned()),
            _ => self.retrieve(client).await,
        }
    }

    async fn retrieve(&self, client: &reqwest::Client) -> crate::Result<String> {
        let mut token = self.token.write().await;
        *token = Some(Self::get_token(client, &self.access_scope).await?);
        match token.as_ref() {
            Some(token) => Ok(token.0.to_owned()),
            None => unreachable!(),
        }
    }

    async fn get_token(client: &reqwest::Client, scope: &str) -> crate::Result<(String, u64)> {
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
        Ok((response.access_token, exp))
    }
}
#[async_trait::async_trait]
impl RefreshableToken for Token {
    async fn get_scope(&self) -> &str {
        self.access_scope.as_ref()
    }
    async fn get_inner_token(&self) -> Option<(String, u64)> {
        self.token.read().await.deref().clone()
    }
    async fn update_inner_token(&self, token: (String, u64)) -> crate::Result<()> {
        *self.token.write().await = Some(token);
        Ok(())
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
