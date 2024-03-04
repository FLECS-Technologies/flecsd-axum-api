#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "/v2";
pub const API_VERSION: &str = "2.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AppsAppDeleteResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// No such app or app/version combination
    Status404_NoSuchAppOrApp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AppsAppGetResponse {
    /// Success
    Status200_Success(Vec<models::InstalledApp>),
    /// No such app or app/version combination
    Status404_NoSuchAppOrApp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AppsGetResponse {
    /// Success
    Status200_Success(Vec<models::InstalledApp>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AppsInstallPostResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// Malformed request
    Status400_MalformedRequest(models::AdditionalInfo),
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AppsSideloadPostResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// Malformed request
    Status400_MalformedRequest(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeviceLicenseActivationPostResponse {
    /// Success
    Status200_Success(models::AdditionalInfo),
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeviceLicenseActivationStatusGetResponse {
    /// Success
    Status200_Success(models::DeviceLicenseActivationStatusGet200Response),
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FlunderBrowseGetResponse {
    /// Success
    Status200_Success(models::FlunderBrowseGet200Response),
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesCreatePostResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// Malformed request
    Status400_MalformedRequest(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesGetResponse {
    /// Success
    Status200_Success(Vec<models::AppInstance>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdConfigGetResponse {
    /// Success
    Status200_Success(models::InstanceConfig),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdConfigPostResponse {
    /// Success
    Status200_Success(models::InstanceConfig),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdDeleteResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdGetResponse {
    /// Success
    Status200_Success(models::InstancesInstanceIdGet200Response),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdLogsGetResponse {
    /// Success
    Status200_Success(models::InstancesInstanceIdLogsGet200Response),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
    /// Internal server error
    Status500_InternalServerError(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdPatchResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdStartPostResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InstancesInstanceIdStopPostResponse {
    /// Accepted
    Status202_Accepted(models::JobMeta),
    /// No instance with this instance_id found
    Status404_NoInstanceWithThisInstance,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum JobsGetResponse {
    /// Success
    Status200_Success(Vec<models::Job>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum JobsJobIdDeleteResponse {
    /// Success
    Status200_Success,
    /// Not found
    Status404_NotFound,
    /// Job not finished
    Status400_JobNotFinished(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum JobsJobIdGetResponse {
    /// Success
    Status200_Success(models::Job),
    /// Not found
    Status404_NotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SystemInfoGetResponse {
    /// Sucess
    Status200_Sucess(models::SystemInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SystemPingGetResponse {
    /// Success
    Status200_Success(models::AdditionalInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SystemVersionGetResponse {
    /// Success
    Status200_Success(models::SystemVersionGet200Response),
}

/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {
    /// Uninstall one or all versions an App.
    ///
    /// AppsAppDelete - DELETE /v2/apps/{app}
    async fn apps_app_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::AppsAppDeletePathParams,
        query_params: models::AppsAppDeleteQueryParams,
    ) -> Result<AppsAppDeleteResponse, String>;

    /// Query all versions or specific versions of an App.
    ///
    /// AppsAppGet - GET /v2/apps/{app}
    async fn apps_app_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::AppsAppGetPathParams,
        query_params: models::AppsAppGetQueryParams,
    ) -> Result<AppsAppGetResponse, String>;

    /// Query installed Apps.
    ///
    /// AppsGet - GET /v2/apps
    async fn apps_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<AppsGetResponse, String>;

    /// Install an App from the FLECS marketplace.
    ///
    /// AppsInstallPost - POST /v2/apps/install
    async fn apps_install_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::AppsInstallPostRequest,
    ) -> Result<AppsInstallPostResponse, String>;

    /// Sideload an App from its manifest.
    ///
    /// AppsSideloadPost - POST /v2/apps/sideload
    async fn apps_sideload_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::AppsSideloadPostRequest,
    ) -> Result<AppsSideloadPostResponse, String>;

    /// Execute device activation.
    ///
    /// DeviceLicenseActivationPost - POST /v2/device/license/activation
    async fn device_license_activation_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<DeviceLicenseActivationPostResponse, String>;

    /// Check if device is activated.
    ///
    /// DeviceLicenseActivationStatusGet - GET /v2/device/license/activation/status
    async fn device_license_activation_status_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<DeviceLicenseActivationStatusGetResponse, String>;

    /// Retrieve stored flunder topics alongside their values.
    ///
    /// FlunderBrowseGet - GET /v2/flunder/browse
    async fn flunder_browse_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::FlunderBrowseGetQueryParams,
    ) -> Result<FlunderBrowseGetResponse, String>;

    /// Create new instance of an installed App.
    ///
    /// InstancesCreatePost - POST /v2/instances/create
    async fn instances_create_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::InstancesCreatePostRequest,
    ) -> Result<InstancesCreatePostResponse, String>;

    /// Query all instances of one or all Apps.
    ///
    /// InstancesGet - GET /v2/instances
    async fn instances_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::InstancesGetQueryParams,
    ) -> Result<InstancesGetResponse, String>;

    /// Get configuration of an Instance.
    ///
    /// InstancesInstanceIdConfigGet - GET /v2/instances/{instance_id}/config
    async fn instances_instance_id_config_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdConfigGetPathParams,
    ) -> Result<InstancesInstanceIdConfigGetResponse, String>;

    /// Update configuration of an Instance.
    ///
    /// InstancesInstanceIdConfigPost - POST /v2/instances/{instance_id}/config
    async fn instances_instance_id_config_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdConfigPostPathParams,
        body: models::InstanceConfig,
    ) -> Result<InstancesInstanceIdConfigPostResponse, String>;

    /// Delete a single instance.
    ///
    /// InstancesInstanceIdDelete - DELETE /v2/instances/{instance_id}
    async fn instances_instance_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdDeletePathParams,
    ) -> Result<InstancesInstanceIdDeleteResponse, String>;

    /// Obtain details of an App instance.
    ///
    /// InstancesInstanceIdGet - GET /v2/instances/{instance_id}
    async fn instances_instance_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdGetPathParams,
    ) -> Result<InstancesInstanceIdGetResponse, String>;

    /// Retrieve logs of an Instance.
    ///
    /// InstancesInstanceIdLogsGet - GET /v2/instances/{instance_id}/logs
    async fn instances_instance_id_logs_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdLogsGetPathParams,
    ) -> Result<InstancesInstanceIdLogsGetResponse, String>;

    /// Update or downgrade Instance to another App version.
    ///
    /// InstancesInstanceIdPatch - PATCH /v2/instances/{instance_id}
    async fn instances_instance_id_patch(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdPatchPathParams,
        body: models::InstancesInstanceIdPatchRequest,
    ) -> Result<InstancesInstanceIdPatchResponse, String>;

    /// Start an App instance.
    ///
    /// InstancesInstanceIdStartPost - POST /v2/instances/{instance_id}/start
    async fn instances_instance_id_start_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdStartPostPathParams,
    ) -> Result<InstancesInstanceIdStartPostResponse, String>;

    /// Stop an App instance.
    ///
    /// InstancesInstanceIdStopPost - POST /v2/instances/{instance_id}/stop
    async fn instances_instance_id_stop_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::InstancesInstanceIdStopPostPathParams,
    ) -> Result<InstancesInstanceIdStopPostResponse, String>;

    /// Retrieve a list of all pending/queued/running/failed/cancelled jobs.
    ///
    /// JobsGet - GET /v2/jobs
    async fn jobs_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<JobsGetResponse, String>;

    /// Cancel job or remove failed/successful/cancelled job from journal.
    ///
    /// JobsJobIdDelete - DELETE /v2/jobs/{job_id}
    async fn jobs_job_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::JobsJobIdDeletePathParams,
    ) -> Result<JobsJobIdDeleteResponse, String>;

    /// Retrieve information for specific job_id.
    ///
    /// JobsJobIdGet - GET /v2/jobs/{job_id}
    async fn jobs_job_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::JobsJobIdGetPathParams,
    ) -> Result<JobsJobIdGetResponse, String>;

    /// Get architecture and operating system information.
    ///
    /// SystemInfoGet - GET /v2/system/info
    async fn system_info_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<SystemInfoGetResponse, String>;

    /// Check daemon availability and connectivity.
    ///
    /// SystemPingGet - GET /v2/system/ping
    async fn system_ping_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<SystemPingGetResponse, String>;

    /// Get FLECS core and API version.
    ///
    /// SystemVersionGet - GET /v2/system/version
    async fn system_version_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<SystemVersionGetResponse, String>;
}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
