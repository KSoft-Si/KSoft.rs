use crate::{
    ApiResponse,
    make_request,
    model::*,
};
use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::endpoint;

pub struct Bans {
    pub http: Arc<HttpClient>
}

impl Bans {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    async fn _check_bans() {}

    pub async fn advanced_paginate(&self, page: u8, per_page: u8) -> HttpResult<ApiResponse<BanList, Error400>>{
        let builder = self.http.clone().get(endpoint("/bans/list").as_str())
            .query(&[("per_page", per_page)])
            .query(&[("page", page)]);

        make_request::<BanList, Error400>(builder).await
    }

    pub async fn paginate(&self) -> HttpResult<ApiResponse<BanList, Error400>> {
        self.advanced_paginate(1, 20).await
    }

    pub async fn add<S: ToString>(&self,
      user_id: u64,
      reason: S,
      proof: S,
      moderator: Option<u64>,
      user_name: Option<String>,
      user_discriminator: Option<u16>,
      appeal_possible: Option<bool>)
    -> HttpResult<ApiResponse<BanAdditionResponse, Error409>>{
        let builder = self.http.clone().post(endpoint("/bans/add").as_str())
            .form(&BanAddition {
                user_id,
                reason: reason.to_string(),
                proof: proof.to_string(),
                moderator,
                user_name,
                user_discriminator,
                appeal_possible
            });

        make_request::<BanAdditionResponse, Error409>(builder).await
    }

    pub async fn check_ban(&self, user_id: u64) -> HttpResult<BanCheckResponse> {
        let response = self.http.clone().get(endpoint("/bans/check").as_str())
            .query(&[("user", user_id)])
            .send()
            .await?;

        response.json::<BanCheckResponse>().await
    }

    async fn ban_info(&self, user_id: u64) -> HttpResult<ApiResponse<BanInfoResponse, Error404>> {
        let builder = self.http.clone().get(endpoint("/bans/info").as_str())
            .query(&[("user", user_id)]);

        make_request::<BanInfoResponse, Error404>(builder).await
    }

    async fn delete_forcing(&self, user_id: u64) -> HttpResult<ApiResponse<BanDeletionResponse, Error401>> {
        let builder = self.http.clone().delete(endpoint("/bans/delete").as_str())
            .query(&[("user", user_id)])
            .query(&[("force", true)]);

        make_request::<BanDeletionResponse, Error401>(builder).await
    }

    async fn delete(&self, user_id: u64) -> HttpResult<ApiResponse<BanDeletionResponse, Error401>> {
        let builder = self.http.clone().delete(endpoint("/bans/delete").as_str())
            .query(&[("user", user_id)]);

        make_request::<BanDeletionResponse, Error401>(builder).await
    }
}

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