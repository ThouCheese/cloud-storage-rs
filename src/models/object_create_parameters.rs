/// The parameters that are optionally supplied when creating an object.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateParameters {
    /// Setting this value is equivalent of setting the `contentEncoding` metadata property of the object.
    /// This can be useful when uploading an object with `uploadType=media` to indicate the encoding of the content being uploaded.
    pub content_encoding: Option<String>,

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

    /// Resource name of the Cloud KMS key that will be used to encrypt the object.
    /// If not specified, the request uses the bucket's default Cloud KMS key, if any, or a Google-managed encryption key.
    pub kms_key_name: Option<String>,

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