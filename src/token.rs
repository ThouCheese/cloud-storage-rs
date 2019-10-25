use openssl::{pkey::PKey, rsa::Rsa};
use serde::{Deserialize, Serialize};

use crate::service_account::SERVICE_ACCOUNT;

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

    pub fn get(&mut self) -> String {
        match self.token {
            Some((ref token, exp)) if exp > now() => token.clone(),
            _ => self.retrieve(),
        }
    }

    fn retrieve(&mut self) -> String {
        self.token = Some(Self::get_token(&self.access_scope));
        self.token.clone().unwrap().0
    }

    fn get_token(scope: &str) -> (String, u64) {
        let now = now();
        let exp = now + 3600;

        let claims = Claims {
            iss: SERVICE_ACCOUNT.client_email.clone(),
            scope: scope.into(),
            aud: "https://www.googleapis.com/oauth2/v4/token".to_string(),
            exp,
            iat: now,
        };
        let mut header = jsonwebtoken::Header::default();
        header.alg = jsonwebtoken::Algorithm::RS256;
        let rsa = Rsa::private_key_from_pem(SERVICE_ACCOUNT.private_key.as_bytes()).unwrap();
        let private_key = PKey::from_rsa(rsa).unwrap();
        let private_key = &private_key.private_key_to_der().unwrap();
        // let private_key = include_bytes!("../../private_key2.der");
        let jwt = jsonwebtoken::encode(&header, &claims, private_key).unwrap();
        let body = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];
        let client = reqwest::Client::new();
        let mut response = client
            .post("https://www.googleapis.com/oauth2/v4/token")
            .form(&body)
            .send()
            .unwrap();
        let response: TokenResponse = response.json().unwrap();
        (response.access_token, exp)
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
