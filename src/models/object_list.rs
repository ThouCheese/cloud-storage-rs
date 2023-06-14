use super::Object;

/// Response from `Object::list`.
#[derive(Debug, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectList {
    /// The kind of item this is. For lists of objects, this is always `storage#objects`.
    pub kind: String,

    /// The list of objects, ordered lexicographically by name.
    #[serde(default = "Vec::new")]
    pub items: Vec<Object>,

    /// Object name prefixes for objects that matched the listing request but were excluded
    /// from `items` because of a delimiter. Values in this list are object names up to and
    /// including the requested delimiter. Duplicate entries are omitted from this list.
    #[serde(default = "Vec::new")]
    pub prefixes: Vec<String>,

    /// The continuation token, included only if there are more items to return. Provide
    /// this value as the `page_token` of a subsequent request in order to return the next
    /// page of results.
    pub next_page_token: Option<String>,
}