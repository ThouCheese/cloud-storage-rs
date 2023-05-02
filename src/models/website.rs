/// Contains configuration about how to visit the website linked to this Bucket.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Website {
    /// If the requested object path is missing, the service will ensure the path has a trailing
    /// '/', append this suffix, and attempt to retrieve the resulting object. This allows the
    /// creation of index.html objects to represent directory pages.
    pub main_page_suffix: String,
    /// If the requested object path is missing, and any mainPageSuffix object is missing, if
    /// applicable, the service will return the named object from this bucket as the content for a
    /// 404 Not Found result.
    pub not_found_page: String,
}