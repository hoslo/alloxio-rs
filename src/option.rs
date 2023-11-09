use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::wire;


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDirectory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_exists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<wire::Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_type: Option<wire::WriteType>,
}

/// CreateFile holds the options for creating a file.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_policy_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<wire::Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_action: Option<wire::TTLAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_type: Option<wire::WriteType>,
}

/// Delete holds the options for deleting a path.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

/// Exists holds the options for checking whether a path exists.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exists;

/// Free holds the options for freeing a path.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Free {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

/// GetStatus holds the options for fetching a path status.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatus;

/// ListStatus holds the options for listing a path.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_metadata_type: Option<wire::LoadMetadataType>,
}

/// Mount holds the options for mounting a path.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

/// OpenFile holds the options for opening a file.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_policy_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_type: Option<wire::ReadType>,
}

/// Rename holds the options for renaming a path.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rename;

/// SetAttribute holds the options for setting path attributes.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<wire::Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_action: Option<wire::TTLAction>,
}

/// Unmount holds the options for unmounting a path.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Unmount;