use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::Deserialize;
use crate::ApiResponse;
use crate::model::*;
use crate::{make_request, endpoint};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Images {
    pub http: Arc<HttpClient>
}

impl Images {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    pub async fn random_image(&self, tag: impl ToString, nsfw: bool) -> HttpResult<ApiResponse<Image, Error404>>{
        let builder = self.http.clone().get(endpoint("/images/random-image").as_str())
            .query(&[("tag", tag.to_string())])
            .query(&[("nsfw", nsfw)]);


        make_request::<Image, Error404>(builder).await
    }

    pub async fn random_meme(&self) -> HttpResult<RedditImage>{
        let response = self.http.clone().get(endpoint("/images/random-meme").as_str())
            .send()
            .await?;

        let image = response.json::<RedditImage>().await?;
        Ok(image)
    }

    pub async fn random_aww(&self) -> HttpResult<RedditImage>{
        return self.http.clone().get(endpoint("/images/random-aww").as_str())
            .send()
            .await?
            .json::<RedditImage>()
            .await
    }

    pub async fn random_reddit(&self, subreddit: impl ToString, remove_nsfw: bool, span: SpanType) -> HttpResult<ApiResponse<RedditImage, Error404>>{
        let builder = self.http.clone().get(endpoint(format!("/images/rand-reddit/{}", subreddit.to_string())).as_str())
            .query(&[("remove_nsfw", remove_nsfw)])
            .query(&[("span", span.to_string())]);

        make_request::<RedditImage, Error404>(builder).await
    }

    pub async fn random_wikihow(&self, nsfw: bool) -> HttpResult<WikiHowImage> {
        return self.http.clone().get(endpoint("/images/random-wikihow").as_str())
            .query(&[("nsfw", nsfw)])
            .send()
            .await?
            .json::<WikiHowImage>()
            .await
    }

    pub async fn get_tags(&self) -> HttpResult<TagList> {
        return self.http.clone().get(endpoint("/images/tags").as_str())
            .send()
            .await?
            .json::<TagList>()
            .await
    }

    pub async fn get_image(&self, sf: impl AsRef<str>) -> HttpResult<ApiResponse<Image, Error404>> {
        let builder = self.http.clone().get(endpoint(format!("/images/image/{}", sf.as_ref())).as_str());

        make_request::<Image, Error404>(builder).await
    }

    pub async fn get_tag(&self, tag: impl AsRef<str>) -> HttpResult<ApiResponse<TagList, Error404>> {
        let builder = self.http.clone().get(endpoint(format!("/images/tags/{}", tag.as_ref())).as_str());

        make_request::<TagList, Error404>(builder).await
    }

    pub async fn random_nsfw(&self, gifs: bool) -> HttpResult<RedditImage> {
        let response = self.http.clone().get(endpoint("/images/random-nsfw").as_str())
            .query(&[("gifs", gifs)])
            .send()
            .await?;

        response.json::<RedditImage>().await
    }
}

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
    pub created_at: i64,
    pub nsfw: bool,
    pub author: String,
    pub awards: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct WikiHowImage {
    pub url: String,
    pub title: String,
    pub nsfw: bool,
    pub article_url: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagModel {
    pub name: String,
    pub nsfw: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagList {
    pub models: Option<Vec<TagModel>>,
    pub tags: Vec<String>,
    pub nsfw_tags: Option<Vec<String>>
}