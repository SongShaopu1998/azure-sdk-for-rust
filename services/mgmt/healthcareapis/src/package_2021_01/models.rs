#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<services_properties::ProvisioningState>,
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<ServiceAccessPoliciesInfo>,
    #[serde(rename = "cosmosDbConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cosmos_db_configuration: Option<ServiceCosmosDbConfigurationInfo>,
    #[serde(rename = "authenticationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<ServiceAuthenticationConfigurationInfo>,
    #[serde(rename = "corsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<ServiceCorsConfigurationInfo>,
    #[serde(rename = "exportConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub export_configuration: Option<ServiceExportConfigurationInfo>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties::PublicNetworkAccess>,
    #[serde(rename = "acrConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub acr_configuration: Option<ServiceAcrConfigurationInfo>,
}
pub mod services_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Creating,
        Accepted,
        Verifying,
        Updating,
        Failed,
        Canceled,
        Deprovisioned,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
pub type ServiceAccessPoliciesInfo = Vec<ServiceAccessPolicyEntry>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccessPolicyEntry {
    #[serde(rename = "objectId")]
    pub object_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCosmosDbConfigurationInfo {
    #[serde(rename = "offerThroughput", default, skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<i64>,
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAuthenticationConfigurationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "smartProxyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub smart_proxy_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<ServiceCorsConfigurationOriginEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<ServiceCorsConfigurationHeaderEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<ServiceCorsConfigurationMethodEntry>,
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceExportConfigurationInfo {
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAcrConfigurationInfo {
    #[serde(rename = "loginServers", default, skip_serializing_if = "Vec::is_empty")]
    pub login_servers: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationOriginEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationHeaderEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationMethodEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesPatchDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub kind: services_resource::Kind,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<services_resource::Identity>,
}
pub mod services_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "fhir")]
        Fhir,
        #[serde(rename = "fhir-Stu3")]
        FhirStu3,
        #[serde(rename = "fhir-R4")]
        FhirR4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Identity {
        #[serde(rename = "principalId", skip_serializing)]
        pub principal_id: Option<String>,
        #[serde(rename = "tenantId", skip_serializing)]
        pub tenant_id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<identity::Type>,
    }
    pub mod identity {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            SystemAssigned,
            None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsInternal>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetailsInternal {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesDescriptionListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServicesDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesPropertiesUpdateParameters {
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties_update_parameters::PublicNetworkAccess>,
}
pub mod services_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesNameAvailabilityInfo {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<services_name_availability_info::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod services_name_availability_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultsDescription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<operation_results_description::Status>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation_results_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionDescription {
    #[serde(flatten)]
    pub private_endpoint_connection: PrivateEndpointConnection,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceDescription {
    #[serde(flatten)]
    pub private_link_resource: PrivateLinkResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResultDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnectionDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResultDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResourceDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}