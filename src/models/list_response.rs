#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListResponse<T> {
    #[serde(default = "Vec::new")]
    pub items: Vec<T>,
}