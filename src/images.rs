use reqwest::{Client as HttpClient, StatusCode, Error as HttpError, Response};
use std::sync::Arc;
use serde_json::json;
use serde::Deserialize;
use crate::response::ApiResponse;
use crate::model::error::Error404;
use crate::error::api::ResponseError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Images {
    token: String,
    pub http: Arc<HttpClient>
}

impl Images {
    pub fn new(token: String, http_client: Arc<HttpClient>) -> Self {
        Self {
            token,
            http: http_client
        }
    }

    pub async fn random_image(&self, tag: impl ToString, nsfw: bool) -> Result<ApiResponse<Image>, HttpError>{
        return match self.http.clone().get("/images/random-image")
            .json(&json!({"tag": tag.to_string(), "nsfw": nsfw}))
            .send()
            .await {
            Err(e) => { Err(e) },
            Ok(res) => {
                match res.status() {
                    StatusCode(e) => {
                        return match e {
                            200u16 => {
                                let image = res.json::<Image>().await?;
                                Ok(ApiResponse::Success(image))
                            },
                            _ => {
                                let error = res.json::<Error404>().await?;
                                Ok(ApiResponse::Failed(ResponseError::E404(error)))
                            }
                        }
                    }
                    _ => {
                        let error = res.json::<Error404>().await?;
                        Ok(ApiResponse::Failed(ResponseError::E404(error)))
                    }
                }
            }
        };
    }

    pub async fn random_meme(&self) -> Result<RedditImage, HttpError>{
        return match self.http.clone().get("/images/random-meme")
            .send()
            .await {
            Err(e) => { Err(e) },
            Ok(res) => {
                let image = res.json::<RedditImage>().await?;
                Ok(image)
            }
        };
    }

    pub async fn random_aww(&self) -> Result<RedditImage, HttpError>{
        return self.http.clone().get("/images/random-aww")
            .send()
            .await?
            .json::<RedditImage>()
            .await
    }

    pub async fn random_reddit(&self, subreddit: impl ToString, remove_nsfw: bool, span: SpanType) -> Result<ApiResponse<RedditImage>, HttpError>{
        let response = self.http.clone().get(format!("/images/rand-reddit/{}", subreddit.to_string()).as_str())
            .json(&json!({"remove_nsfw": remove_nsfw, "span": span}))
            .send()
            .await?;

        return match response.status() {
            StatusCode(e) => {
                match e {
                    200u16 => {
                        let image = response.json::<RedditImage>().await?;
                        Ok(ApiResponse::Success(image))
                    },
                    _ => {
                        let error = response.json::<Error404>().await?;
                        Ok(ApiResponse::Failed(ResponseError::E404(error)))
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub enum SpanType {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All
}

impl Display for SpanType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SpanType::Hour => write!(f, "hour"),
            SpanType::Day => write!(f, "day"),
            SpanType::Week => write!(f, "week"),
            SpanType::Month => write!(f, "month"),
            SpanType::Year => write!(f, "year"),
            SpanType::All => write!(f, "all")
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub snowflake: String,
    pub nsfw: bool,
    pub tag: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedditImage {
    pub title: String,
    pub image_url: String,
    pub source: String,
    pub subreddit: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub comments: u64,
    pub created_at: u64,
    pub nsfw: bool,
    pub author: String,
    pub awards: u64
}