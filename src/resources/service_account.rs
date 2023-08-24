use dotenvy::dotenv;

/// A deserialized `service-account-********.json`-file.
#[derive(serde::Deserialize, Debug)]
pub struct ServiceAccount {
    /// The type of authentication, this should always be `service_account`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The name of the current project.
    pub project_id: String,
    /// A unqiue identifier for the private key.
    pub private_key_id: String,
    /// The private key in RSA format.
    pub private_key: String,
    /// The email address associated with the service account.
    pub client_email: String,
    /// The unique identifier for this client.
    pub client_id: String,
    /// The endpoint where authentication happens.
    pub auth_uri: String,
    /// The endpoint where OAuth2 tokens are issued.
    pub token_uri: String,
    /// The url of the cert provider.
    pub auth_provider_x509_cert_url: String,
    /// The url of a static file containing metadata for this certificate.
    pub client_x509_cert_url: String,
}

impl ServiceAccount {
    pub(crate) fn get() -> Self {
        dotenv().ok();
        let credentials_json = std::env::var("SERVICE_ACCOUNT")
            .or_else(|_| std::env::var("GOOGLE_APPLICATION_CREDENTIALS"))
            .map(|path| std::fs::read_to_string(path).expect("SERVICE_ACCOUNT file not found"))
            .or_else(|_| std::env::var("SERVICE_ACCOUNT_JSON"))
            .or_else(|_| std::env::var("GOOGLE_APPLICATION_CREDENTIALS_JSON"))
            .expect(
                "SERVICE_ACCOUNT(_JSON) or GOOGLE_APPLICATION_CREDENTIALS(_JSON) environment parameter required",
            );
        let account: Self =
            serde_json::from_str(&credentials_json).expect("SERVICE_ACCOUNT file not valid");
        assert_eq!(
            account.r#type, "service_account",
            "`type` parameter of `SERVICE_ACCOUNT` variable is not 'service_account'"
        );
        account
    }
}
