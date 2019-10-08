#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum SignatureResponse {
    Success(SuccessResponse),
    Failure(FailureResponse),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    pub key_id: String,
    pub signature: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct FailureResponse {
    pub error: SignatureError,
}

#[derive(serde::Deserialize, Debug)]
pub struct SignatureError {
    pub code: u16,
    pub message: String,
    pub status: String,
    pub details: Option<Vec<Details>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "@type")]
    pub _type: String,
    pub links: Vec<Link>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Link {
    pub description: String,
    pub url: String,
}
