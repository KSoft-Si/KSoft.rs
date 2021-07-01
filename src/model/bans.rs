use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct BanList {
    pub ban_count: u64,
    pub page_count: u64,
    pub per_page: u64,
    pub page: u64,
    pub on_page: u64,
    pub next_page: u64,
    pub previous_page: Option<u64>,
    pub data: Vec<BanData>
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanData {
    pub id: String,
    pub name: String,
    pub discriminator: String,
    pub moderator_id: String,
    pub reason: String,
    pub proof: String,
    pub is_ban_active: bool,
    pub can_be_appealed: bool,
    pub timestamp: String,
    pub appeal_reason: Option<String>,
    pub appeal_date: Option<String>
}

#[derive(Clone, Debug, Serialize)]
pub struct BanAddition {
    #[serde(rename = "user")]
    pub user_id: u64,
    pub reason: String,
    pub proof: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mod")]
    pub moderator: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_discriminator: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appeal_possible: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanAdditionResponse {
    pub success: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanDeletionResponse {
    pub done: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanCheckResponse {
    pub is_banned: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanInfoResponse {
    pub id: String,
    pub name: String,
    pub discriminator: String,
    pub moderator_id: String,
    pub reason: String,
    pub proof: String,
    pub is_ban_active: bool,
    pub can_be_appealed: bool,
    pub timestamp: String,
    pub appeal_reason: Option<String>,
    pub appeal_date: Option<String>,
    pub requested_by: String,
    pub exists: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawBanUpdate {
    pub data: Vec<BanUpdate>,
    #[serde(rename = "current_timestamp")]
    pub timestamp: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanUpdate {
    pub id: u64,
    pub reason: String,
    pub proof: String,
    #[serde(rename = "moderator_id")]
    pub moderator: u64,
    pub active: bool
}
