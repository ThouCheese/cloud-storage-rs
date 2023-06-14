use super::Projection;

/// The request that is supplied to perform `Object::list`.
/// See [the Google Cloud Storage API
/// reference](https://cloud.google.com/storage/docs/json_api/v1/objects/list)
/// for more details.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListRequest {
    /// When specified, allows the `list` to operate like a directory listing by splitting the
    /// object location on this delimiter.
    pub delimiter: Option<String>,

    /// Filter results to objects whose names are lexicographically before `end_offset`.
    /// If `start_offset` is also set, the objects listed have names between `start_offset`
    /// (inclusive) and `end_offset` (exclusive).
    pub end_offset: Option<String>,

    /// If true, objects that end in exactly one instance of `delimiter` have their metadata
    /// included in `items` in addition to the relevant part of the object name appearing in
    /// `prefixes`.
    pub include_trailing_delimiter: Option<bool>,

    /// Maximum combined number of entries in `items` and `prefixes` to return in a single
    /// page of responses. Because duplicate entries in `prefixes` are omitted, fewer total
    /// results may be returned than requested. The service uses this parameter or 1,000
    /// items, whichever is smaller.
    pub max_results: Option<usize>,

    /// A previously-returned page token representing part of the larger set of results to view.
    /// The `page_token` is an encoded field that marks the name and generation of the last object
    /// in the returned list. In a subsequent request using the `page_token`, items that come after
    /// the `page_token` are shown (up to `max_results`).
    ///
    /// If the page token is provided, all objects starting at that page token are queried
    pub page_token: Option<String>,

    /// Filter results to include only objects whose names begin with this prefix.
    pub prefix: Option<String>,

    /// Set of properties to return. Defaults to `NoAcl`.
    pub projection: Option<Projection>,

    /// Filter results to objects whose names are lexicographically equal to or after
    /// `start_offset`. If `end_offset` is also set, the objects listed have names between
    /// `start_offset` (inclusive) and `end_offset` (exclusive).
    pub start_offset: Option<String>,

    /// If true, lists all versions of an object as distinct results in order of increasing
    /// generation number. The default value for versions is false. For more information, see
    /// Object Versioning.
    pub versions: Option<bool>,
}