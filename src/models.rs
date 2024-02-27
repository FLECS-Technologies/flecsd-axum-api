#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsAppDeletePathParams {
    pub app: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsAppDeleteQueryParams {
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsAppGetPathParams {
    pub app: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsAppGetQueryParams {
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FlunderBrowseGetQueryParams {
    #[serde(rename = "q")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigGetPathParams {
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigPostPathParams {
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdLogsGetPathParams {
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdStartPostPathParams {
    #[validate(regex = "RE_INSTANCEINSTANCEIDSTARTPOSTPATHPARAMS_INSTANCE_ID")]
    pub instance_id: String,
}

lazy_static::lazy_static! {
    static ref RE_INSTANCEINSTANCEIDSTARTPOSTPATHPARAMS_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdStopPostPathParams {
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesGetQueryParams {
    #[serde(rename = "app")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesInstanceIdDeletePathParams {
    #[validate(regex = "RE_INSTANCESINSTANCEIDDELETEPATHPARAMS_INSTANCE_ID")]
    pub instance_id: String,
}

lazy_static::lazy_static! {
    static ref RE_INSTANCESINSTANCEIDDELETEPATHPARAMS_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesInstanceIdGetPathParams {
    #[validate(regex = "RE_INSTANCESINSTANCEIDGETPATHPARAMS_INSTANCE_ID")]
    pub instance_id: String,
}

lazy_static::lazy_static! {
    static ref RE_INSTANCESINSTANCEIDGETPATHPARAMS_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesInstanceIdPatchPathParams {
    #[validate(regex = "RE_INSTANCESINSTANCEIDPATCHPATHPARAMS_INSTANCE_ID")]
    pub instance_id: String,
}

lazy_static::lazy_static! {
    static ref RE_INSTANCESINSTANCEIDPATCHPATHPARAMS_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobsJobIdDeletePathParams {
    pub job_id: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobsJobIdGetPathParams {
    pub job_id: i32,
}

/// Additional info

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AdditionalInfo {
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl AdditionalInfo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AdditionalInfo {
        AdditionalInfo {
            additional_info: None,
        }
    }
}

/// Converts the AdditionalInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AdditionalInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![self.additional_info.as_ref().map(|additional_info| {
                ["additionalInfo".to_string(), additional_info.to_string()].join(",")
            })];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AdditionalInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AdditionalInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub additional_info: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AdditionalInfo".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "additionalInfo" => intermediate_rep.additional_info.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AdditionalInfo".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AdditionalInfo {
            additional_info: intermediate_rep.additional_info.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AdditionalInfo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AdditionalInfo>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AdditionalInfo>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AdditionalInfo - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AdditionalInfo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AdditionalInfo as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AdditionalInfo - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Instance of an App

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppInstance {
    #[serde(rename = "instanceId")]
    #[validate(regex = "RE_APPINSTANCE_INSTANCE_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// Instance name
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,

    #[serde(rename = "appKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<models::AppKey>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::InstanceStatus>,

    #[serde(rename = "desired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired: Option<models::InstanceStatus>,
}

lazy_static::lazy_static! {
    static ref RE_APPINSTANCE_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

impl AppInstance {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppInstance {
        AppInstance {
            instance_id: None,
            instance_name: None,
            app_key: None,
            status: None,
            desired: None,
        }
    }
}

/// Converts the AppInstance value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppInstance {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.instance_id
                .as_ref()
                .map(|instance_id| ["instanceId".to_string(), instance_id.to_string()].join(",")),
            self.instance_name.as_ref().map(|instance_name| {
                ["instanceName".to_string(), instance_name.to_string()].join(",")
            }),
            // Skipping appKey in query parameter serialization

            // Skipping status in query parameter serialization

            // Skipping desired in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppInstance value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppInstance {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub instance_id: Vec<String>,
            pub instance_name: Vec<String>,
            pub app_key: Vec<models::AppKey>,
            pub status: Vec<models::InstanceStatus>,
            pub desired: Vec<models::InstanceStatus>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppInstance".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "instanceId" => intermediate_rep.instance_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "instanceName" => intermediate_rep.instance_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "appKey" => intermediate_rep.app_key.push(
                        <models::AppKey as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::InstanceStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "desired" => intermediate_rep.desired.push(
                        <models::InstanceStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppInstance".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppInstance {
            instance_id: intermediate_rep.instance_id.into_iter().next(),
            instance_name: intermediate_rep.instance_name.into_iter().next(),
            app_key: intermediate_rep.app_key.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            desired: intermediate_rep.desired.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppInstance> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppInstance>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppInstance>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppInstance - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppInstance> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppInstance as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppInstance - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppKey {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl AppKey {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppKey {
        AppKey {
            name: None,
            version: None,
        }
    }
}

/// Converts the AppKey value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppKey {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppKey value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppKey {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppKey".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppKey".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppKey {
            name: intermediate_rep.name.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppKey> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppKey>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppKey>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppKey - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppKey> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppKey as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppKey - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum AppStatus {
    #[serde(rename = "not installed")]
    NotInstalled,
    #[serde(rename = "manifest downloaded")]
    ManifestDownloaded,
    #[serde(rename = "token acquired")]
    TokenAcquired,
    #[serde(rename = "image downloaded")]
    ImageDownloaded,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "purged")]
    Purged,
    #[serde(rename = "orphaned")]
    Orphaned,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for AppStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppStatus::NotInstalled => write!(f, "not installed"),
            AppStatus::ManifestDownloaded => write!(f, "manifest downloaded"),
            AppStatus::TokenAcquired => write!(f, "token acquired"),
            AppStatus::ImageDownloaded => write!(f, "image downloaded"),
            AppStatus::Installed => write!(f, "installed"),
            AppStatus::Removed => write!(f, "removed"),
            AppStatus::Purged => write!(f, "purged"),
            AppStatus::Orphaned => write!(f, "orphaned"),
            AppStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for AppStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "not installed" => std::result::Result::Ok(AppStatus::NotInstalled),
            "manifest downloaded" => std::result::Result::Ok(AppStatus::ManifestDownloaded),
            "token acquired" => std::result::Result::Ok(AppStatus::TokenAcquired),
            "image downloaded" => std::result::Result::Ok(AppStatus::ImageDownloaded),
            "installed" => std::result::Result::Ok(AppStatus::Installed),
            "removed" => std::result::Result::Ok(AppStatus::Removed),
            "purged" => std::result::Result::Ok(AppStatus::Purged),
            "orphaned" => std::result::Result::Ok(AppStatus::Orphaned),
            "unknown" => std::result::Result::Ok(AppStatus::Unknown),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsInstallPost400Response {
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl AppsInstallPost400Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppsInstallPost400Response {
        AppsInstallPost400Response {
            additional_info: None,
        }
    }
}

/// Converts the AppsInstallPost400Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppsInstallPost400Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![self.additional_info.as_ref().map(|additional_info| {
                ["additionalInfo".to_string(), additional_info.to_string()].join(",")
            })];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppsInstallPost400Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppsInstallPost400Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub additional_info: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppsInstallPost400Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "additionalInfo" => intermediate_rep.additional_info.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppsInstallPost400Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppsInstallPost400Response {
            additional_info: intermediate_rep.additional_info.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppsInstallPost400Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppsInstallPost400Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppsInstallPost400Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppsInstallPost400Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppsInstallPost400Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppsInstallPost400Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppsInstallPost400Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsInstallPost500Response {
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl AppsInstallPost500Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppsInstallPost500Response {
        AppsInstallPost500Response {
            additional_info: None,
        }
    }
}

/// Converts the AppsInstallPost500Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppsInstallPost500Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![self.additional_info.as_ref().map(|additional_info| {
                ["additionalInfo".to_string(), additional_info.to_string()].join(",")
            })];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppsInstallPost500Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppsInstallPost500Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub additional_info: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppsInstallPost500Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "additionalInfo" => intermediate_rep.additional_info.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppsInstallPost500Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppsInstallPost500Response {
            additional_info: intermediate_rep.additional_info.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppsInstallPost500Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppsInstallPost500Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppsInstallPost500Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppsInstallPost500Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppsInstallPost500Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppsInstallPost500Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppsInstallPost500Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsInstallPostRequest {
    #[serde(rename = "appKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<models::AppKey>,

    /// License key for App installation
    #[serde(rename = "licenseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
}

impl AppsInstallPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppsInstallPostRequest {
        AppsInstallPostRequest {
            app_key: None,
            license_key: None,
        }
    }
}

/// Converts the AppsInstallPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppsInstallPostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping appKey in query parameter serialization
            self.license_key
                .as_ref()
                .map(|license_key| ["licenseKey".to_string(), license_key.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppsInstallPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppsInstallPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub app_key: Vec<models::AppKey>,
            pub license_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppsInstallPostRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "appKey" => intermediate_rep.app_key.push(
                        <models::AppKey as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "licenseKey" => intermediate_rep.license_key.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppsInstallPostRequest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppsInstallPostRequest {
            app_key: intermediate_rep.app_key.into_iter().next(),
            license_key: intermediate_rep.license_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppsInstallPostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppsInstallPostRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppsInstallPostRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppsInstallPostRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppsInstallPostRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppsInstallPostRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppsInstallPostRequest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AppsSideloadPostRequest {
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,

    #[serde(rename = "licenseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
}

impl AppsSideloadPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AppsSideloadPostRequest {
        AppsSideloadPostRequest {
            manifest: None,
            license_key: None,
        }
    }
}

/// Converts the AppsSideloadPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AppsSideloadPostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.manifest
                .as_ref()
                .map(|manifest| ["manifest".to_string(), manifest.to_string()].join(",")),
            self.license_key
                .as_ref()
                .map(|license_key| ["licenseKey".to_string(), license_key.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AppsSideloadPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AppsSideloadPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub manifest: Vec<String>,
            pub license_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AppsSideloadPostRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "manifest" => intermediate_rep.manifest.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "licenseKey" => intermediate_rep.license_key.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AppsSideloadPostRequest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AppsSideloadPostRequest {
            manifest: intermediate_rep.manifest.into_iter().next(),
            license_key: intermediate_rep.license_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AppsSideloadPostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AppsSideloadPostRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AppsSideloadPostRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AppsSideloadPostRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AppsSideloadPostRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AppsSideloadPostRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AppsSideloadPostRequest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeviceLicenseActivationStatusGet200Response {
    #[serde(rename = "isValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
}

impl DeviceLicenseActivationStatusGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> DeviceLicenseActivationStatusGet200Response {
        DeviceLicenseActivationStatusGet200Response { is_valid: None }
    }
}

/// Converts the DeviceLicenseActivationStatusGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeviceLicenseActivationStatusGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![self
            .is_valid
            .as_ref()
            .map(|is_valid| ["isValid".to_string(), is_valid.to_string()].join(","))];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeviceLicenseActivationStatusGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeviceLicenseActivationStatusGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub is_valid: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val =
                match string_iter.next() {
                    Some(x) => x,
                    None => return std::result::Result::Err(
                        "Missing value while parsing DeviceLicenseActivationStatusGet200Response"
                            .to_string(),
                    ),
                };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "isValid" => intermediate_rep.is_valid.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => return std::result::Result::Err(
                        "Unexpected key while parsing DeviceLicenseActivationStatusGet200Response"
                            .to_string(),
                    ),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeviceLicenseActivationStatusGet200Response {
            is_valid: intermediate_rep.is_valid.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeviceLicenseActivationStatusGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DeviceLicenseActivationStatusGet200Response>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DeviceLicenseActivationStatusGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeviceLicenseActivationStatusGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<DeviceLicenseActivationStatusGet200Response>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeviceLicenseActivationStatusGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeviceLicenseActivationStatusGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FlunderBrowseGet200Response {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::FlunderBrowseGet200ResponseDataInner>>,
}

impl FlunderBrowseGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> FlunderBrowseGet200Response {
        FlunderBrowseGet200Response { data: None }
    }
}

/// Converts the FlunderBrowseGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FlunderBrowseGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FlunderBrowseGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FlunderBrowseGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::FlunderBrowseGet200ResponseDataInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing FlunderBrowseGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in FlunderBrowseGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing FlunderBrowseGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FlunderBrowseGet200Response {
            data: intermediate_rep.data.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FlunderBrowseGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FlunderBrowseGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<FlunderBrowseGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for FlunderBrowseGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<FlunderBrowseGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <FlunderBrowseGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into FlunderBrowseGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FlunderBrowseGet200ResponseDataInner {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl FlunderBrowseGet200ResponseDataInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> FlunderBrowseGet200ResponseDataInner {
        FlunderBrowseGet200ResponseDataInner {
            topic: None,
            value: None,
            encoding: None,
            timestamp: None,
        }
    }
}

/// Converts the FlunderBrowseGet200ResponseDataInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FlunderBrowseGet200ResponseDataInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.topic
                .as_ref()
                .map(|topic| ["topic".to_string(), topic.to_string()].join(",")),
            self.value
                .as_ref()
                .map(|value| ["value".to_string(), value.to_string()].join(",")),
            self.encoding
                .as_ref()
                .map(|encoding| ["encoding".to_string(), encoding.to_string()].join(",")),
            self.timestamp
                .as_ref()
                .map(|timestamp| ["timestamp".to_string(), timestamp.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FlunderBrowseGet200ResponseDataInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FlunderBrowseGet200ResponseDataInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub topic: Vec<String>,
            pub value: Vec<String>,
            pub encoding: Vec<String>,
            pub timestamp: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing FlunderBrowseGet200ResponseDataInner"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "topic" => intermediate_rep.topic.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "encoding" => intermediate_rep.encoding.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "timestamp" => intermediate_rep.timestamp.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing FlunderBrowseGet200ResponseDataInner"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FlunderBrowseGet200ResponseDataInner {
            topic: intermediate_rep.topic.into_iter().next(),
            value: intermediate_rep.value.into_iter().next(),
            encoding: intermediate_rep.encoding.into_iter().next(),
            timestamp: intermediate_rep.timestamp.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FlunderBrowseGet200ResponseDataInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FlunderBrowseGet200ResponseDataInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<FlunderBrowseGet200ResponseDataInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FlunderBrowseGet200ResponseDataInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<FlunderBrowseGet200ResponseDataInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FlunderBrowseGet200ResponseDataInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FlunderBrowseGet200ResponseDataInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstalledApp {
    #[serde(rename = "appKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<models::AppKey>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::AppStatus>,

    #[serde(rename = "desired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired: Option<models::AppStatus>,

    #[serde(rename = "installedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_size: Option<i32>,
}

impl InstalledApp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstalledApp {
        InstalledApp {
            app_key: None,
            status: None,
            desired: None,
            installed_size: None,
        }
    }
}

/// Converts the InstalledApp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstalledApp {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping appKey in query parameter serialization

            // Skipping status in query parameter serialization

            // Skipping desired in query parameter serialization
            self.installed_size.as_ref().map(|installed_size| {
                ["installedSize".to_string(), installed_size.to_string()].join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstalledApp value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstalledApp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub app_key: Vec<models::AppKey>,
            pub status: Vec<models::AppStatus>,
            pub desired: Vec<models::AppStatus>,
            pub installed_size: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstalledApp".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "appKey" => intermediate_rep.app_key.push(
                        <models::AppKey as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::AppStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "desired" => intermediate_rep.desired.push(
                        <models::AppStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "installedSize" => intermediate_rep.installed_size.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstalledApp".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstalledApp {
            app_key: intermediate_rep.app_key.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            desired: intermediate_rep.desired.into_iter().next(),
            installed_size: intermediate_rep.installed_size.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstalledApp> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstalledApp>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstalledApp>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstalledApp - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstalledApp> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstalledApp as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstalledApp - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailConffile {
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl InstanceDetailConffile {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailConffile {
        InstanceDetailConffile {
            container: None,
            host: None,
        }
    }
}

/// Converts the InstanceDetailConffile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailConffile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.container
                .as_ref()
                .map(|container| ["container".to_string(), container.to_string()].join(",")),
            self.host
                .as_ref()
                .map(|host| ["host".to_string(), host.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailConffile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailConffile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub container: Vec<String>,
            pub host: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailConffile".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "container" => intermediate_rep.container.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "host" => intermediate_rep.host.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceDetailConffile".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailConffile {
            container: intermediate_rep.container.into_iter().next(),
            host: intermediate_rep.host.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailConffile> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailConffile>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailConffile>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstanceDetailConffile - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstanceDetailConffile> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstanceDetailConffile as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstanceDetailConffile - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Hostname of an instance
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailHostname(String);

impl validator::Validate for InstanceDetailHostname {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for InstanceDetailHostname {
    fn from(x: String) -> Self {
        InstanceDetailHostname(x)
    }
}

impl std::string::ToString for InstanceDetailHostname {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for InstanceDetailHostname {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(InstanceDetailHostname(x.to_string()))
    }
}

impl std::convert::From<InstanceDetailHostname> for String {
    fn from(x: InstanceDetailHostname) -> Self {
        x.0
    }
}

impl std::ops::Deref for InstanceDetailHostname {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for InstanceDetailHostname {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

/// IP address of an instance
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailIpAddress(String);

impl validator::Validate for InstanceDetailIpAddress {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for InstanceDetailIpAddress {
    fn from(x: String) -> Self {
        InstanceDetailIpAddress(x)
    }
}

impl std::string::ToString for InstanceDetailIpAddress {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for InstanceDetailIpAddress {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(InstanceDetailIpAddress(x.to_string()))
    }
}

impl std::convert::From<InstanceDetailIpAddress> for String {
    fn from(x: InstanceDetailIpAddress) -> Self {
        x.0
    }
}

impl std::ops::Deref for InstanceDetailIpAddress {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for InstanceDetailIpAddress {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

/// Bind mounts of an instance

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailMounts {
    #[serde(rename = "mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::InstanceDetailMountsMountsInner>>,
}

impl InstanceDetailMounts {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailMounts {
        InstanceDetailMounts { mounts: None }
    }
}

/// Converts the InstanceDetailMounts value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailMounts {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping mounts in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailMounts value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailMounts {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub mounts: Vec<Vec<models::InstanceDetailMountsMountsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailMounts".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "mounts" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceDetailMounts".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceDetailMounts".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailMounts {
            mounts: intermediate_rep.mounts.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailMounts> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailMounts>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailMounts>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstanceDetailMounts - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstanceDetailMounts> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstanceDetailMounts as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstanceDetailMounts - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailMountsMountsInner {
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl InstanceDetailMountsMountsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailMountsMountsInner {
        InstanceDetailMountsMountsInner {
            container: None,
            host: None,
        }
    }
}

/// Converts the InstanceDetailMountsMountsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailMountsMountsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.container
                .as_ref()
                .map(|container| ["container".to_string(), container.to_string()].join(",")),
            self.host
                .as_ref()
                .map(|host| ["host".to_string(), host.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailMountsMountsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailMountsMountsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub container: Vec<String>,
            pub host: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailMountsMountsInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "container" => intermediate_rep.container.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "host" => intermediate_rep.host.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceDetailMountsMountsInner"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailMountsMountsInner {
            container: intermediate_rep.container.into_iter().next(),
            host: intermediate_rep.host.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailMountsMountsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailMountsMountsInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailMountsMountsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceDetailMountsMountsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceDetailMountsMountsInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceDetailMountsMountsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceDetailMountsMountsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

/// Allocated network ports of an instance

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailPorts {
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<models::InstanceDetailPortsPortsInner>>,
}

impl InstanceDetailPorts {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailPorts {
        InstanceDetailPorts { ports: None }
    }
}

/// Converts the InstanceDetailPorts value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailPorts {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping ports in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailPorts value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailPorts {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ports: Vec<Vec<models::InstanceDetailPortsPortsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailPorts".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "ports" => return std::result::Result::Err(
                        "Parsing a container in this style is not supported in InstanceDetailPorts"
                            .to_string(),
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceDetailPorts".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailPorts {
            ports: intermediate_rep.ports.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailPorts> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailPorts>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailPorts>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstanceDetailPorts - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstanceDetailPorts> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstanceDetailPorts as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstanceDetailPorts - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailPortsPortsInner {
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl InstanceDetailPortsPortsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailPortsPortsInner {
        InstanceDetailPortsPortsInner {
            container: None,
            host: None,
        }
    }
}

/// Converts the InstanceDetailPortsPortsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailPortsPortsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.container
                .as_ref()
                .map(|container| ["container".to_string(), container.to_string()].join(",")),
            self.host
                .as_ref()
                .map(|host| ["host".to_string(), host.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailPortsPortsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailPortsPortsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub container: Vec<String>,
            pub host: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailPortsPortsInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "container" => intermediate_rep.container.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "host" => intermediate_rep.host.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceDetailPortsPortsInner"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailPortsPortsInner {
            container: intermediate_rep.container.into_iter().next(),
            host: intermediate_rep.host.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailPortsPortsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailPortsPortsInner>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailPortsPortsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstanceDetailPortsPortsInner - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstanceDetailPortsPortsInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceDetailPortsPortsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceDetailPortsPortsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

/// Automatic volumes of an instance

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailVolumes {
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<models::InstanceDetailVolumesVolumesInner>>,
}

impl InstanceDetailVolumes {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailVolumes {
        InstanceDetailVolumes { volumes: None }
    }
}

/// Converts the InstanceDetailVolumes value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailVolumes {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping volumes in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailVolumes value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailVolumes {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub volumes: Vec<Vec<models::InstanceDetailVolumesVolumesInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailVolumes".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "volumes" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceDetailVolumes".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceDetailVolumes".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailVolumes {
            volumes: intermediate_rep.volumes.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailVolumes> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailVolumes>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailVolumes>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstanceDetailVolumes - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstanceDetailVolumes> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstanceDetailVolumes as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstanceDetailVolumes - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceDetailVolumesVolumesInner {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl InstanceDetailVolumesVolumesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceDetailVolumesVolumesInner {
        InstanceDetailVolumesVolumesInner {
            name: None,
            path: None,
        }
    }
}

/// Converts the InstanceDetailVolumesVolumesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceDetailVolumesVolumesInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.path
                .as_ref()
                .map(|path| ["path".to_string(), path.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceDetailVolumesVolumesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceDetailVolumesVolumesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub path: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceDetailVolumesVolumesInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "path" => intermediate_rep.path.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceDetailVolumesVolumesInner"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceDetailVolumesVolumesInner {
            name: intermediate_rep.name.into_iter().next(),
            path: intermediate_rep.path.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceDetailVolumesVolumesInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceDetailVolumesVolumesInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceDetailVolumesVolumesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceDetailVolumesVolumesInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceDetailVolumesVolumesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceDetailVolumesVolumesInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceDetailVolumesVolumesInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceId(String);

impl validator::Validate for InstanceId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for InstanceId {
    fn from(x: String) -> Self {
        InstanceId(x)
    }
}

impl std::string::ToString for InstanceId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for InstanceId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(InstanceId(x.to_string()))
    }
}

impl std::convert::From<InstanceId> for String {
    fn from(x: InstanceId) -> Self {
        x.0
    }
}

impl std::ops::Deref for InstanceId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for InstanceId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigGet200Response {
    #[serde(rename = "networkAdapters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_adapters:
        Option<Vec<models::InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner>>,

    #[serde(rename = "usbDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb_devices: Option<Vec<models::InstanceInstanceIdConfigGet200ResponseUsbDevicesInner>>,
}

impl InstanceInstanceIdConfigGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigGet200Response {
        InstanceInstanceIdConfigGet200Response {
            network_adapters: None,
            usb_devices: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping networkAdapters in query parameter serialization

            // Skipping usbDevices in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub network_adapters:
                Vec<Vec<models::InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner>>,
            pub usb_devices:
                Vec<Vec<models::InstanceInstanceIdConfigGet200ResponseUsbDevicesInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceInstanceIdConfigGet200Response"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "networkAdapters" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceInstanceIdConfigGet200Response".to_string()),
                    "usbDevices" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceInstanceIdConfigGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigGet200Response {
            network_adapters: intermediate_rep.network_adapters.into_iter().next(),
            usb_devices: intermediate_rep.usb_devices.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceInstanceIdConfigGet200Response>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdConfigGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigGet200Response>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "connected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,

    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(rename = "subnetMask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,

    #[serde(rename = "gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

impl InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
        InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
            name: None,
            active: None,
            connected: None,
            ip_address: None,
            subnet_mask: None,
            gateway: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.active
                .as_ref()
                .map(|active| ["active".to_string(), active.to_string()].join(",")),
            self.connected
                .as_ref()
                .map(|connected| ["connected".to_string(), connected.to_string()].join(",")),
            self.ip_address
                .as_ref()
                .map(|ip_address| ["ipAddress".to_string(), ip_address.to_string()].join(",")),
            self.subnet_mask
                .as_ref()
                .map(|subnet_mask| ["subnetMask".to_string(), subnet_mask.to_string()].join(",")),
            self.gateway
                .as_ref()
                .map(|gateway| ["gateway".to_string(), gateway.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub active: Vec<bool>,
            pub connected: Vec<bool>,
            pub ip_address: Vec<String>,
            pub subnet_mask: Vec<String>,
            pub gateway: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "active" => intermediate_rep.active.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "connected" => intermediate_rep.connected.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "subnetMask" => intermediate_rep.subnet_mask.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gateway" => intermediate_rep.gateway.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner {
            name: intermediate_rep.name.into_iter().next(),
            active: intermediate_rep.active.into_iter().next(),
            connected: intermediate_rep.connected.into_iter().next(),
            ip_address: intermediate_rep.ip_address.into_iter().next(),
            subnet_mask: intermediate_rep.subnet_mask.into_iter().next(),
            gateway: intermediate_rep.gateway.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<
            InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner,
        >,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigGet200ResponseNetworkAdaptersInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "vid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vid: Option<i32>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "connected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

impl InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
        InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
            device: None,
            pid: None,
            port: None,
            vendor: None,
            vid: None,
            active: None,
            connected: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigGet200ResponseUsbDevicesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.device
                .as_ref()
                .map(|device| ["device".to_string(), device.to_string()].join(",")),
            self.pid
                .as_ref()
                .map(|pid| ["pid".to_string(), pid.to_string()].join(",")),
            self.port
                .as_ref()
                .map(|port| ["port".to_string(), port.to_string()].join(",")),
            self.vendor
                .as_ref()
                .map(|vendor| ["vendor".to_string(), vendor.to_string()].join(",")),
            self.vid
                .as_ref()
                .map(|vid| ["vid".to_string(), vid.to_string()].join(",")),
            self.active
                .as_ref()
                .map(|active| ["active".to_string(), active.to_string()].join(",")),
            self.connected
                .as_ref()
                .map(|connected| ["connected".to_string(), connected.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigGet200ResponseUsbDevicesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub device: Vec<String>,
            pub pid: Vec<i32>,
            pub port: Vec<String>,
            pub vendor: Vec<String>,
            pub vid: Vec<i32>,
            pub active: Vec<bool>,
            pub connected: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InstanceInstanceIdConfigGet200ResponseUsbDevicesInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "device" => intermediate_rep.device.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pid" => intermediate_rep.pid.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "port" => intermediate_rep.port.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vendor" => intermediate_rep.vendor.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vid" => intermediate_rep.vid.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "active" => intermediate_rep.active.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "connected" => intermediate_rep.connected.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigGet200ResponseUsbDevicesInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigGet200ResponseUsbDevicesInner {
            device: intermediate_rep.device.into_iter().next(),
            pid: intermediate_rep.pid.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            vendor: intermediate_rep.vendor.into_iter().next(),
            vid: intermediate_rep.vid.into_iter().next(),
            active: intermediate_rep.active.into_iter().next(),
            connected: intermediate_rep.connected.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseUsbDevicesInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseUsbDevicesInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseUsbDevicesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigGet200ResponseUsbDevicesInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigGet200ResponseUsbDevicesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigGet200ResponseUsbDevicesInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigGet200ResponseUsbDevicesInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigPostRequest {
    #[serde(rename = "networkAdapters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_adapters:
        Option<Vec<models::InstanceInstanceIdConfigPostRequestNetworkAdaptersInner>>,

    #[serde(rename = "usbDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb_devices: Option<Vec<models::InstanceInstanceIdConfigPostRequestUsbDevicesInner>>,
}

impl InstanceInstanceIdConfigPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigPostRequest {
        InstanceInstanceIdConfigPostRequest {
            network_adapters: None,
            usb_devices: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigPostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping networkAdapters in query parameter serialization

            // Skipping usbDevices in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub network_adapters:
                Vec<Vec<models::InstanceInstanceIdConfigPostRequestNetworkAdaptersInner>>,
            pub usb_devices: Vec<Vec<models::InstanceInstanceIdConfigPostRequestUsbDevicesInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceInstanceIdConfigPostRequest"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "networkAdapters" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceInstanceIdConfigPostRequest".to_string()),
                    "usbDevices" => return std::result::Result::Err("Parsing a container in this style is not supported in InstanceInstanceIdConfigPostRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigPostRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigPostRequest {
            network_adapters: intermediate_rep.network_adapters.into_iter().next(),
            usb_devices: intermediate_rep.usb_devices.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigPostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceInstanceIdConfigPostRequest>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdConfigPostRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigPostRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigPostRequest>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigPostRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigPostRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
        InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
            name: None,
            active: None,
            ip_address: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigPostRequestNetworkAdaptersInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.active
                .as_ref()
                .map(|active| ["active".to_string(), active.to_string()].join(",")),
            self.ip_address
                .as_ref()
                .map(|ip_address| ["ipAddress".to_string(), ip_address.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigPostRequestNetworkAdaptersInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub active: Vec<bool>,
            pub ip_address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InstanceInstanceIdConfigPostRequestNetworkAdaptersInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "active" => intermediate_rep.active.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigPostRequestNetworkAdaptersInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigPostRequestNetworkAdaptersInner {
            name: intermediate_rep.name.into_iter().next(),
            active: intermediate_rep.active.into_iter().next(),
            ip_address: intermediate_rep.ip_address.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestNetworkAdaptersInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestNetworkAdaptersInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestNetworkAdaptersInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigPostRequestNetworkAdaptersInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestNetworkAdaptersInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigPostRequestNetworkAdaptersInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigPostRequestNetworkAdaptersInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdConfigPostRequestUsbDevicesInner {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "vid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vid: Option<i32>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl InstanceInstanceIdConfigPostRequestUsbDevicesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdConfigPostRequestUsbDevicesInner {
        InstanceInstanceIdConfigPostRequestUsbDevicesInner {
            device: None,
            pid: None,
            port: None,
            vendor: None,
            vid: None,
            active: None,
        }
    }
}

/// Converts the InstanceInstanceIdConfigPostRequestUsbDevicesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdConfigPostRequestUsbDevicesInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.device
                .as_ref()
                .map(|device| ["device".to_string(), device.to_string()].join(",")),
            self.pid
                .as_ref()
                .map(|pid| ["pid".to_string(), pid.to_string()].join(",")),
            self.port
                .as_ref()
                .map(|port| ["port".to_string(), port.to_string()].join(",")),
            self.vendor
                .as_ref()
                .map(|vendor| ["vendor".to_string(), vendor.to_string()].join(",")),
            self.vid
                .as_ref()
                .map(|vid| ["vid".to_string(), vid.to_string()].join(",")),
            self.active
                .as_ref()
                .map(|active| ["active".to_string(), active.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdConfigPostRequestUsbDevicesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdConfigPostRequestUsbDevicesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub device: Vec<String>,
            pub pid: Vec<i32>,
            pub port: Vec<String>,
            pub vendor: Vec<String>,
            pub vid: Vec<i32>,
            pub active: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InstanceInstanceIdConfigPostRequestUsbDevicesInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "device" => intermediate_rep.device.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pid" => intermediate_rep.pid.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "port" => intermediate_rep.port.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vendor" => intermediate_rep.vendor.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vid" => intermediate_rep.vid.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "active" => intermediate_rep.active.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstanceInstanceIdConfigPostRequestUsbDevicesInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdConfigPostRequestUsbDevicesInner {
            device: intermediate_rep.device.into_iter().next(),
            pid: intermediate_rep.pid.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            vendor: intermediate_rep.vendor.into_iter().next(),
            vid: intermediate_rep.vid.into_iter().next(),
            active: intermediate_rep.active.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestUsbDevicesInner> and HeaderValue

#[cfg(feature = "server")]
impl
    std::convert::TryFrom<
        header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestUsbDevicesInner>,
    > for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestUsbDevicesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdConfigPostRequestUsbDevicesInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdConfigPostRequestUsbDevicesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdConfigPostRequestUsbDevicesInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdConfigPostRequestUsbDevicesInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceInstanceIdLogsGet200Response {
    #[serde(rename = "stdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,

    #[serde(rename = "stderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
}

impl InstanceInstanceIdLogsGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstanceInstanceIdLogsGet200Response {
        InstanceInstanceIdLogsGet200Response {
            stdout: None,
            stderr: None,
        }
    }
}

/// Converts the InstanceInstanceIdLogsGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstanceInstanceIdLogsGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.stdout
                .as_ref()
                .map(|stdout| ["stdout".to_string(), stdout.to_string()].join(",")),
            self.stderr
                .as_ref()
                .map(|stderr| ["stderr".to_string(), stderr.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstanceInstanceIdLogsGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstanceInstanceIdLogsGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub stdout: Vec<String>,
            pub stderr: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstanceInstanceIdLogsGet200Response"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "stdout" => intermediate_rep.stdout.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "stderr" => intermediate_rep.stderr.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstanceInstanceIdLogsGet200Response"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstanceInstanceIdLogsGet200Response {
            stdout: intermediate_rep.stdout.into_iter().next(),
            stderr: intermediate_rep.stderr.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstanceInstanceIdLogsGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstanceInstanceIdLogsGet200Response>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstanceInstanceIdLogsGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstanceInstanceIdLogsGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstanceInstanceIdLogsGet200Response>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstanceInstanceIdLogsGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstanceInstanceIdLogsGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

/// Instance name
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstanceName(String);

impl validator::Validate for InstanceName {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for InstanceName {
    fn from(x: String) -> Self {
        InstanceName(x)
    }
}

impl std::string::ToString for InstanceName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for InstanceName {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(InstanceName(x.to_string()))
    }
}

impl std::convert::From<InstanceName> for String {
    fn from(x: InstanceName) -> Self {
        x.0
    }
}

impl std::ops::Deref for InstanceName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for InstanceName {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum InstanceStatus {
    #[serde(rename = "not created")]
    NotCreated,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "resources ready")]
    ResourcesReady,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "orphaned")]
    Orphaned,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for InstanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InstanceStatus::NotCreated => write!(f, "not created"),
            InstanceStatus::Requested => write!(f, "requested"),
            InstanceStatus::ResourcesReady => write!(f, "resources ready"),
            InstanceStatus::Created => write!(f, "created"),
            InstanceStatus::Stopped => write!(f, "stopped"),
            InstanceStatus::Running => write!(f, "running"),
            InstanceStatus::Orphaned => write!(f, "orphaned"),
            InstanceStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for InstanceStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "not created" => std::result::Result::Ok(InstanceStatus::NotCreated),
            "requested" => std::result::Result::Ok(InstanceStatus::Requested),
            "resources ready" => std::result::Result::Ok(InstanceStatus::ResourcesReady),
            "created" => std::result::Result::Ok(InstanceStatus::Created),
            "stopped" => std::result::Result::Ok(InstanceStatus::Stopped),
            "running" => std::result::Result::Ok(InstanceStatus::Running),
            "orphaned" => std::result::Result::Ok(InstanceStatus::Orphaned),
            "unknown" => std::result::Result::Ok(InstanceStatus::Unknown),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesCreatePostRequest {
    #[serde(rename = "appKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<models::AppKey>,

    /// Instance name
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

impl InstancesCreatePostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstancesCreatePostRequest {
        InstancesCreatePostRequest {
            app_key: None,
            instance_name: None,
        }
    }
}

/// Converts the InstancesCreatePostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstancesCreatePostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping appKey in query parameter serialization
            self.instance_name.as_ref().map(|instance_name| {
                ["instanceName".to_string(), instance_name.to_string()].join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstancesCreatePostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstancesCreatePostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub app_key: Vec<models::AppKey>,
            pub instance_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstancesCreatePostRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "appKey" => intermediate_rep.app_key.push(
                        <models::AppKey as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "instanceName" => intermediate_rep.instance_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstancesCreatePostRequest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstancesCreatePostRequest {
            app_key: intermediate_rep.app_key.into_iter().next(),
            instance_name: intermediate_rep.instance_name.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstancesCreatePostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstancesCreatePostRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstancesCreatePostRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InstancesCreatePostRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InstancesCreatePostRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InstancesCreatePostRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InstancesCreatePostRequest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesInstanceIdGet200Response {
    #[serde(rename = "instanceId")]
    #[validate(regex = "RE_INSTANCESINSTANCEIDGET200RESPONSE_INSTANCE_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// Instance name
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,

    #[serde(rename = "appKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<models::AppKey>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::InstanceStatus>,

    #[serde(rename = "desired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired: Option<models::InstanceStatus>,

    #[serde(rename = "conffiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conffiles: Option<Vec<models::InstanceDetailConffile>>,

    /// Hostname of an instance
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// IP address of an instance
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<models::InstanceDetailPorts>,

    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<models::InstanceDetailVolumes>,
}

lazy_static::lazy_static! {
    static ref RE_INSTANCESINSTANCEIDGET200RESPONSE_INSTANCE_ID: regex::Regex = regex::Regex::new(r"^[0-9a-f]{8}$").unwrap();
}

impl InstancesInstanceIdGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstancesInstanceIdGet200Response {
        InstancesInstanceIdGet200Response {
            instance_id: None,
            instance_name: None,
            app_key: None,
            status: None,
            desired: None,
            conffiles: None,
            hostname: None,
            ip_address: None,
            ports: None,
            volumes: None,
        }
    }
}

/// Converts the InstancesInstanceIdGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstancesInstanceIdGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.instance_id
                .as_ref()
                .map(|instance_id| ["instanceId".to_string(), instance_id.to_string()].join(",")),
            self.instance_name.as_ref().map(|instance_name| {
                ["instanceName".to_string(), instance_name.to_string()].join(",")
            }),
            // Skipping appKey in query parameter serialization

            // Skipping status in query parameter serialization

            // Skipping desired in query parameter serialization

            // Skipping conffiles in query parameter serialization
            self.hostname
                .as_ref()
                .map(|hostname| ["hostname".to_string(), hostname.to_string()].join(",")),
            self.ip_address
                .as_ref()
                .map(|ip_address| ["ipAddress".to_string(), ip_address.to_string()].join(",")),
            // Skipping ports in query parameter serialization

            // Skipping volumes in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstancesInstanceIdGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstancesInstanceIdGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub instance_id: Vec<String>,
            pub instance_name: Vec<String>,
            pub app_key: Vec<models::AppKey>,
            pub status: Vec<models::InstanceStatus>,
            pub desired: Vec<models::InstanceStatus>,
            pub conffiles: Vec<Vec<models::InstanceDetailConffile>>,
            pub hostname: Vec<String>,
            pub ip_address: Vec<String>,
            pub ports: Vec<models::InstanceDetailPorts>,
            pub volumes: Vec<models::InstanceDetailVolumes>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstancesInstanceIdGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "instanceId" => intermediate_rep.instance_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instanceName" => intermediate_rep.instance_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "appKey" => intermediate_rep.app_key.push(<models::AppKey as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::InstanceStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "desired" => intermediate_rep.desired.push(<models::InstanceStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "conffiles" => return std::result::Result::Err("Parsing a container in this style is not supported in InstancesInstanceIdGet200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "hostname" => intermediate_rep.hostname.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ports" => intermediate_rep.ports.push(<models::InstanceDetailPorts as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volumes" => intermediate_rep.volumes.push(<models::InstanceDetailVolumes as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InstancesInstanceIdGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstancesInstanceIdGet200Response {
            instance_id: intermediate_rep.instance_id.into_iter().next(),
            instance_name: intermediate_rep.instance_name.into_iter().next(),
            app_key: intermediate_rep.app_key.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            desired: intermediate_rep.desired.into_iter().next(),
            conffiles: intermediate_rep.conffiles.into_iter().next(),
            hostname: intermediate_rep.hostname.into_iter().next(),
            ip_address: intermediate_rep.ip_address.into_iter().next(),
            ports: intermediate_rep.ports.into_iter().next(),
            volumes: intermediate_rep.volumes.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstancesInstanceIdGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstancesInstanceIdGet200Response>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstancesInstanceIdGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstancesInstanceIdGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstancesInstanceIdGet200Response>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstancesInstanceIdGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstancesInstanceIdGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InstancesInstanceIdPatchRequest {
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl InstancesInstanceIdPatchRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InstancesInstanceIdPatchRequest {
        InstancesInstanceIdPatchRequest { to: None }
    }
}

/// Converts the InstancesInstanceIdPatchRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InstancesInstanceIdPatchRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![self
            .to
            .as_ref()
            .map(|to| ["to".to_string(), to.to_string()].join(","))];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InstancesInstanceIdPatchRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InstancesInstanceIdPatchRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub to: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InstancesInstanceIdPatchRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InstancesInstanceIdPatchRequest"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InstancesInstanceIdPatchRequest {
            to: intermediate_rep.to.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InstancesInstanceIdPatchRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InstancesInstanceIdPatchRequest>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InstancesInstanceIdPatchRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InstancesInstanceIdPatchRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<InstancesInstanceIdPatchRequest>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InstancesInstanceIdPatchRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InstancesInstanceIdPatchRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Job {
    #[serde(rename = "id")]
    #[validate(range(min = 1, max = 4294967295))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::JobStatus>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "numSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_steps: Option<i32>,

    #[serde(rename = "currentStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step: Option<models::JobStep>,

    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<models::JobResult>,
}

impl Job {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Job {
        Job {
            id: None,
            status: None,
            description: None,
            num_steps: None,
            current_step: None,
            result: None,
        }
    }
}

/// Converts the Job value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Job {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.id
                .as_ref()
                .map(|id| ["id".to_string(), id.to_string()].join(",")),
            // Skipping status in query parameter serialization
            self.description
                .as_ref()
                .map(|description| ["description".to_string(), description.to_string()].join(",")),
            self.num_steps
                .as_ref()
                .map(|num_steps| ["numSteps".to_string(), num_steps.to_string()].join(",")),
            // Skipping currentStep in query parameter serialization

            // Skipping result in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Job value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<u32>,
            pub status: Vec<models::JobStatus>,
            pub description: Vec<String>,
            pub num_steps: Vec<i32>,
            pub current_step: Vec<models::JobStep>,
            pub result: Vec<models::JobResult>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err("Missing value while parsing Job".to_string())
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::JobStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "numSteps" => intermediate_rep.num_steps.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "currentStep" => intermediate_rep.current_step.push(
                        <models::JobStep as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "result" => intermediate_rep.result.push(
                        <models::JobResult as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Job".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Job {
            id: intermediate_rep.id.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            num_steps: intermediate_rep.num_steps.into_iter().next(),
            current_step: intermediate_rep.current_step.into_iter().next(),
            result: intermediate_rep.result.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Job> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Job>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Job>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Job - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Job> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Job as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Job - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobId(i32);

impl validator::Validate for JobId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<i32> for JobId {
    fn from(x: i32) -> Self {
        JobId(x)
    }
}

impl std::convert::From<JobId> for i32 {
    fn from(x: JobId) -> Self {
        x.0
    }
}

impl std::ops::Deref for JobId {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for JobId {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

/// Job metadata for accepted requests

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobMeta {
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i32>,
}

impl JobMeta {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> JobMeta {
        JobMeta { job_id: None }
    }
}

/// Converts the JobMeta value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JobMeta {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![self
            .job_id
            .as_ref()
            .map(|job_id| ["jobId".to_string(), job_id.to_string()].join(","))];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JobMeta value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JobMeta {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub job_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing JobMeta".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "jobId" => intermediate_rep.job_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing JobMeta".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JobMeta {
            job_id: intermediate_rep.job_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JobMeta> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<JobMeta>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<JobMeta>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for JobMeta - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<JobMeta> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <JobMeta as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into JobMeta - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobResult {
    #[serde(rename = "exit_code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl JobResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> JobResult {
        JobResult {
            exit_code: None,
            message: None,
        }
    }
}

/// Converts the JobResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JobResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.exit_code
                .as_ref()
                .map(|exit_code| ["exit_code".to_string(), exit_code.to_string()].join(",")),
            self.message
                .as_ref()
                .map(|message| ["message".to_string(), message.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JobResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JobResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub exit_code: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing JobResult".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "exit_code" => intermediate_rep.exit_code.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing JobResult".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JobResult {
            exit_code: intermediate_rep.exit_code.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JobResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<JobResult>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<JobResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for JobResult - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<JobResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <JobResult as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into JobResult - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum JobStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Queued => write!(f, "queued"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Cancelled => write!(f, "cancelled"),
            JobStatus::Successful => write!(f, "successful"),
            JobStatus::Failed => write!(f, "failed"),
            JobStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "pending" => std::result::Result::Ok(JobStatus::Pending),
            "queued" => std::result::Result::Ok(JobStatus::Queued),
            "running" => std::result::Result::Ok(JobStatus::Running),
            "cancelled" => std::result::Result::Ok(JobStatus::Cancelled),
            "successful" => std::result::Result::Ok(JobStatus::Successful),
            "failed" => std::result::Result::Ok(JobStatus::Failed),
            "unknown" => std::result::Result::Ok(JobStatus::Unknown),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JobStep {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "num")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,

    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,

    #[serde(rename = "unitsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_total: Option<i32>,

    #[serde(rename = "unitsDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_done: Option<i32>,

    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

impl JobStep {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> JobStep {
        JobStep {
            description: None,
            num: None,
            unit: None,
            units_total: None,
            units_done: None,
            rate: None,
        }
    }
}

/// Converts the JobStep value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JobStep {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.description
                .as_ref()
                .map(|description| ["description".to_string(), description.to_string()].join(",")),
            self.num
                .as_ref()
                .map(|num| ["num".to_string(), num.to_string()].join(",")),
            self.unit
                .as_ref()
                .map(|unit| ["unit".to_string(), unit.to_string()].join(",")),
            self.units_total
                .as_ref()
                .map(|units_total| ["unitsTotal".to_string(), units_total.to_string()].join(",")),
            self.units_done
                .as_ref()
                .map(|units_done| ["unitsDone".to_string(), units_done.to_string()].join(",")),
            self.rate
                .as_ref()
                .map(|rate| ["rate".to_string(), rate.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JobStep value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JobStep {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub description: Vec<String>,
            pub num: Vec<i32>,
            pub unit: Vec<i32>,
            pub units_total: Vec<i32>,
            pub units_done: Vec<i32>,
            pub rate: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing JobStep".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "num" => intermediate_rep.num.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "unit" => intermediate_rep.unit.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "unitsTotal" => intermediate_rep.units_total.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "unitsDone" => intermediate_rep.units_done.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "rate" => intermediate_rep.rate.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing JobStep".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JobStep {
            description: intermediate_rep.description.into_iter().next(),
            num: intermediate_rep.num.into_iter().next(),
            unit: intermediate_rep.unit.into_iter().next(),
            units_total: intermediate_rep.units_total.into_iter().next(),
            units_done: intermediate_rep.units_done.into_iter().next(),
            rate: intermediate_rep.rate.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JobStep> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<JobStep>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<JobStep>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for JobStep - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<JobStep> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <JobStep as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into JobStep - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// License key for App installation
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LicenseKey(String);

impl validator::Validate for LicenseKey {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for LicenseKey {
    fn from(x: String) -> Self {
        LicenseKey(x)
    }
}

impl std::string::ToString for LicenseKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::str::FromStr for LicenseKey {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(LicenseKey(x.to_string()))
    }
}

impl std::convert::From<LicenseKey> for String {
    fn from(x: LicenseKey) -> Self {
        x.0
    }
}

impl std::ops::Deref for LicenseKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for LicenseKey {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SystemPingGet200Response {
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl SystemPingGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> SystemPingGet200Response {
        SystemPingGet200Response {
            additional_info: None,
        }
    }
}

/// Converts the SystemPingGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SystemPingGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![self.additional_info.as_ref().map(|additional_info| {
                ["additionalInfo".to_string(), additional_info.to_string()].join(",")
            })];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SystemPingGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SystemPingGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub additional_info: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SystemPingGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "additionalInfo" => intermediate_rep.additional_info.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SystemPingGet200Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SystemPingGet200Response {
            additional_info: intermediate_rep.additional_info.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SystemPingGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SystemPingGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SystemPingGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SystemPingGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SystemPingGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SystemPingGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SystemPingGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SystemVersionGet200Response {
    #[serde(rename = "api")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,

    #[serde(rename = "core")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<String>,
}

impl SystemVersionGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> SystemVersionGet200Response {
        SystemVersionGet200Response {
            api: None,
            core: None,
        }
    }
}

/// Converts the SystemVersionGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SystemVersionGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.api
                .as_ref()
                .map(|api| ["api".to_string(), api.to_string()].join(",")),
            self.core
                .as_ref()
                .map(|core| ["core".to_string(), core.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SystemVersionGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SystemVersionGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub api: Vec<String>,
            pub core: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SystemVersionGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "api" => intermediate_rep.api.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "core" => intermediate_rep.core.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SystemVersionGet200Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SystemVersionGet200Response {
            api: intermediate_rep.api.into_iter().next(),
            core: intermediate_rep.core.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SystemVersionGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SystemVersionGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SystemVersionGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SystemVersionGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SystemVersionGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SystemVersionGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SystemVersionGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
