use super::ObjectAccessControl;

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObjectAccessControlList {
    kind: String,
    items: Vec<ObjectAccessControl>,
}