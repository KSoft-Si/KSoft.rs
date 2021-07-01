use serde::Deserialize;

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
    pub created_at: f64,
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
