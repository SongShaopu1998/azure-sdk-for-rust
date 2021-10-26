#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conversion {
    pub id: String,
    pub settings: ConversionSettings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<conversion::Output>,
    pub error: Error,
    pub status: ConversionStatus,
    #[serde(rename = "creationTime")]
    pub creation_time: String,
}
pub mod conversion {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Output {
        #[serde(rename = "outputAssetUri", default, skip_serializing_if = "Option::is_none")]
        pub output_asset_uri: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionList {
    pub conversions: Vec<Conversion>,
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionInputSettings {
    #[serde(rename = "storageContainerUri")]
    pub storage_container_uri: String,
    #[serde(rename = "storageContainerReadListSas", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_read_list_sas: Option<String>,
    #[serde(rename = "blobPrefix", default, skip_serializing_if = "Option::is_none")]
    pub blob_prefix: Option<String>,
    #[serde(rename = "relativeInputAssetPath")]
    pub relative_input_asset_path: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionOutputSettings {
    #[serde(rename = "storageContainerUri")]
    pub storage_container_uri: String,
    #[serde(rename = "storageContainerWriteSas", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_write_sas: Option<String>,
    #[serde(rename = "blobPrefix", default, skip_serializing_if = "Option::is_none")]
    pub blob_prefix: Option<String>,
    #[serde(rename = "outputAssetFilename", default, skip_serializing_if = "Option::is_none")]
    pub output_asset_filename: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateConversionSettings {
    pub settings: ConversionSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionSettings {
    #[serde(rename = "inputLocation")]
    pub input_location: ConversionInputSettings,
    #[serde(rename = "outputLocation")]
    pub output_location: ConversionOutputSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConversionStatus {
    NotStarted,
    Running,
    Cancelled,
    Failed,
    Succeeded,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Box<Option<Error>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionProperties {
    pub id: String,
    #[serde(rename = "arrInspectorPort", default, skip_serializing_if = "Option::is_none")]
    pub arr_inspector_port: Option<i64>,
    #[serde(rename = "handshakePort", default, skip_serializing_if = "Option::is_none")]
    pub handshake_port: Option<i64>,
    #[serde(rename = "elapsedTimeMinutes", default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "maxLeaseTimeMinutes", default, skip_serializing_if = "Option::is_none")]
    pub max_lease_time_minutes: Option<i64>,
    pub size: SessionSize,
    pub status: SessionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teraflops: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSessionSettings {
    #[serde(rename = "maxLeaseTimeMinutes")]
    pub max_lease_time_minutes: i64,
    pub size: SessionSize,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSessionSettings {
    #[serde(rename = "maxLeaseTimeMinutes")]
    pub max_lease_time_minutes: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SessionStatus {
    Error,
    Expired,
    Starting,
    Ready,
    Stopped,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SessionSize {
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionsList {
    pub sessions: Vec<SessionProperties>,
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}