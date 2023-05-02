/// The parameters that are optionally supplied when rewriting an object.
#[derive(Debug, PartialEq, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RewriteParameters {
    ///Resource name of the Cloud KMS key that will be used to encrypt the object.
    /// The Cloud KMS key must be located in same location as the object.
    //
    // If the parameter is not specified, the request uses the destination bucket's default encryption key, if any, or the Google-managed encryption key.
    pub destination_kms_key_name: Option<String>,

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

    /// Makes the operation conditional on there being a live destination object with a generation number that does not match the given value.
    /// If no live destination object exists, the precondition fails.
    /// Setting `ifGenerationNotMatch` to 0 makes the operation succeed if there is a live version of the object.
    pub if_generation_not_match: Option<usize>,

    /// Makes the operation conditional on there being a live destination object with a metageneration number that matches the given value.
    pub if_metageneration_match: Option<usize>,

    /// Makes the operation conditional on there being a live destination object with a metageneration number that does not match the given value.
    pub if_metageneration_not_match: Option<usize>,

    /// Makes the operation conditional on whether the source object's generation matches the given value.
    pub if_source_generation_match: Option<usize>,

    /// Makes the operation conditional on whether the source object's generation does not match the given value.
    pub if_source_generation_not_match: Option<usize>,

    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.
    pub if_source_metageneration_match: Option<usize>,

    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    pub if_source_metageneration_not_match: Option<usize>,

    /// The maximum number of bytes that will be rewritten per rewrite request.
    /// Most callers shouldn't need to specify this parameter - it is primarily in place to support testing.
    /// If specified the value must be an integral multiple of 1 MiB (1048576).
    /// Also, this only applies to requests where the source and destination span locations and/or storage classes.
    /// Finally, this value must not change across rewrite calls else you'll get an error that the `rewriteToken` is invalid.
    pub max_bytes_rewritten_per_call: Option<usize>,

    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    /// Acceptable values are:
    /// `full`: Include all properties.
    /// `noAcl`: Omit the owner, acl property.
    pub projection: Option<String>,

    /// Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite response 'done' flag is true.
    /// Calls that provide a `rewriteToken` can omit all other request fields, but if included those fields must match the values provided in the first rewrite request.
    pub rewrite_token: Option<String>,

    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    pub source_generation: Option<usize>,
}