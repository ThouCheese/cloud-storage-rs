/// A rule that might induce an `Action` if met.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified
    /// age.
    pub age: Option<i32>,
    /// A date in `RFC 3339` format with only the date part (for instance, "2013-01-15"). This
    /// condition is satisfied when an object is created before midnight of the specified date in
    /// UTC.
    #[serde(default, with = "crate::rfc3339_date::option")]
    pub created_before: Option<time::Date>,
    /// Relevant only for versioned objects. If the value is true, this condition matches the live
    /// version of objects; if the value is `false`, it matches noncurrent versions of objects.
    pub is_live: Option<bool>,
    /// Objects having any of the storage classes specified by this condition will be matched.
    /// Values include STANDARD, NEARLINE, COLDLINE, MULTI_REGIONAL, REGIONAL, and
    /// DURABLE_REDUCED_AVAILABILITY.
    pub matches_storage_class: Option<Vec<String>>,
    /// Relevant only for versioned objects. If the value is N, this condition is satisfied when
    /// there are at least N versions (including the live version) newer than this version of the
    /// object.
    #[serde(default, deserialize_with = "crate::from_str_opt")]
    pub num_newer_versions: Option<i32>,
}