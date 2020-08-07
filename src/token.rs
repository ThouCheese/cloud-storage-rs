use crate::error::Error;
use serde::{Deserialize, Serialize};

/// This struct contains contains a token, an expiry, and an access scope.
pub struct Token {
    // this field contains the JWT and the expiry thereof. They are in the same Option because if
    // one of them is `Some`, we require that the other be `Some` as well.
    token: Option<(String, u64)>,
    // store the access scope for later use if we need to refresh the token
    access_scope: String,
}

#[derive(Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: usize,
    token_type: String,
}

impl Token {
    pub fn new(scope: &str) -> Self {
        Self {
            token: None,
            access_scope: scope.to_string(),
        }
    }

    pub async fn get(&mut self) -> crate::Result<String> {
        match self.token {
            Some((ref token, exp)) if exp > now() => Ok(token.clone()),
            _ => self.retrieve().await,
        }
    }

    async fn retrieve(&mut self) -> crate::Result<String> {
        self.token = Some(Self::get_token(&self.access_scope).await?);
        match self.token {
            Some(ref token) => Ok(token.0.clone()),
            None => unreachable!(),
        }
    }

    async fn get_token(scope: &str) -> Result<(String, u64), Error> {
        let now = now();
        let exp = now + 3600;

        let claims = Claims {
            iss: crate::SERVICE_ACCOUNT.client_email.clone(),
            scope: scope.into(),
            aud: "https://www.googleapis.com/oauth2/v4/token".to_string(),
            exp,
            iat: now,
        };
        let mut header = jsonwebtoken::Header::default();
        header.alg = jsonwebtoken::Algorithm::RS256;
        let private_key_bytes = crate::SERVICE_ACCOUNT.private_key.as_bytes();
        let private_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key_bytes)?;
        let jwt = jsonwebtoken::encode(&header, &claims, &private_key)?;
        let body = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];
        let response: TokenResponse = reqwest::Client::new()
            .post("https://www.googleapis.com/oauth2/v4/token")
            .form(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok((response.access_token, exp))
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
