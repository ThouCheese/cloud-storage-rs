/// Acceptable values of `projection` properties to return from `Object::list` requests.
#[derive(Debug, PartialEq, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Projection {
    /// Include all properties.
    Full,
    /// Omit the owner, acl property.
    NoAcl,
}