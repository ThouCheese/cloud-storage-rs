/// Any type of role we can encounter.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    /// Full access.
    Owner,
    /// Write, but not administer.
    Writer,
    /// Only read access.
    Reader,
}