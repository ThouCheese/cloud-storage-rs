/// The parameters that are optionally supplied when reading an object.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadParameters {
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<usize>,

    /// Makes the operation conditional on whether the object's current generation matches the given value.
    /// Setting to 0 makes the operation succeed only if there are no live versions of the object.
    pub if_generation_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    /// If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    pub if_generation_not_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    pub if_metageneration_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    pub if_metageneration_not_match: Option<usize>,

    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    /// Acceptable values are:
    /// `full`: Include all properties.
    /// `noAcl`: Omit the owner, acl property.
    pub projection: Option<String>,
}