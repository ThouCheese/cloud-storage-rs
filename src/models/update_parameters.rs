/// The parameters that are optionally supplied when updating an object.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParameters {
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<usize>,

    /// Makes the operation conditional on whether the object's current generation matches the given value.
    /// Setting to 0 makes the operation succeed only if there are no live versions of the object.
    pub if_generation_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current generation does not match the given value.
    /// If no live object exists, the precondition fails.
    /// Setting to 0 makes the operation succeed only if there is a live version of the object.
    pub if_generation_not_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    pub if_metageneration_match: Option<usize>,

    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    pub if_metageneration_not_match: Option<usize>,

    /// Apply a predefined set of access controls to this object.
    ///
    /// Acceptable values are:
    /// `authenticatedRead`: Object owner gets OWNER access, and allAuthenticatedUsers get READER access.
    /// `bucketOwnerFullControl`: Object owner gets OWNER access, and project team owners get OWNER access.
    /// `bucketOwnerRead`: Object owner gets OWNER access, and project team owners get READER access.
    /// `private`: Object owner gets OWNER access.
    /// `projectPrivate`: Object owner gets OWNER access, and project team members get access according to their roles.
    /// `publicRead`: Object owner gets OWNER access, and allUsers get READER access.
    /// If `iamConfiguration.uniformBucketLevelAccess.enabled` is set to `true`, requests that include this parameter fail with a 400 Bad Request response.
    pub predefined_acl: Option<String>,

    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    /// Acceptable values are:
    /// `full`: Include all properties.
    /// `noAcl`: Omit the owner, acl property.
    pub projection: Option<String>,
}