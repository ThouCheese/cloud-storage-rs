/// The parameters that are optionally supplied when composing an object.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComposeParameters {
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Acceptable values are:
    /// `authenticatedRead`: Object owner gets OWNER access, and allAuthenticatedUsers get READER access.
    /// `bucketOwnerFullControl`: Object owner gets OWNER access, and project team owners get OWNER access.
    /// `bucketOwnerRead`: Object owner gets OWNER access, and project team owners get READER access.
    /// `private`: Object owner gets OWNER access.
    /// `projectPrivate`: Object owner gets OWNER access, and project team members get access according to their roles.
    /// `publicRead`: Object owner gets OWNER access, and allUsers get READER access.
    /// If `iamConfiguration.uniformBucketLevelAccess.enabled` is set to `true`, requests that include this parameter fail with a 400 Bad Request response.
    pub destination_predefined_acl: Option<String>,

    /// Makes the operation conditional on there being a live destination object with a generation number that matches the given value.
    /// Setting `ifGenerationMatch` to 0 makes the operation succeed only if there is no live destination object.
    pub if_generation_match: Option<usize>,

    /// Makes the operation conditional on there being a live destination object with a metageneration number that matches the given value.
    pub if_metageneration_match: Option<usize>,

    /// Resource name of the Cloud KMS key that will be used to encrypt the composed object.
    /// If not specified, the request uses the bucket's default Cloud KMS key, if any, or a Google-managed encryption key.
    pub kms_key_name: Option<String>,
}