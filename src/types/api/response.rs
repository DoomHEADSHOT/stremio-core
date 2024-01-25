use core::fmt;

use crate::types::{
    addon::Descriptor,
    library::LibraryItem,
    profile::{AuthKey, User},
    True,
};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use derive_more::TryInto;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum APIResult<T> {
    Err { error: APIError },
    Ok { result: T },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(Default))]
pub struct APIError {
    pub message: String,
    pub code: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionResponse {
    pub addons: Vec<Descriptor>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "authKey")]
    pub key: AuthKey,
    pub user: User,
}

impl fmt::Debug for AuthResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthResponse")
            .field("key", &"<SENSITIVE>")
            .field("user", &self.user)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataExportResponse {
    pub export_id: String,
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
pub struct LibraryItemModified(
    pub String,
    #[serde(with = "ts_milliseconds")] pub DateTime<Utc>,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: True,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LinkCodeResponse {
    pub code: String,
    pub link: String,
    pub qrcode: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkAuthKey {
    pub auth_key: String,
}

#[derive(Clone, TryInto, Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[serde(untagged)]
pub enum LinkDataResponse {
    AuthKey(LinkAuthKey),
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModalAddon {
    pub name: String,
    pub manifest_url: Url,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetModalResponse {
    pub id: String,
    pub title: String,
    pub message: String,
    pub image_url: Url,
    pub addon: Option<ModalAddon>,
    pub external_url: Option<Url>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNotificationResponse {
    pub id: String,
    pub title: String,
    pub message: String,
    pub external_url: Option<Url>,
}

/// API response for the [`LibraryItem`]s which skips invalid items
/// when deserializing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde_as]
#[serde(transparent)]
pub struct LibraryItemsResponse(#[serde_as(as = "VecSkipError<_>")] pub Vec<LibraryItem>);

impl LibraryItemsResponse {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
