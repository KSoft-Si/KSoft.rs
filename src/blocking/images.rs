use reqwest::blocking::{Client as HttpClient};
use std::sync::Arc;
use crate::{
    endpoint,
    model::*,
    HttpResult
};
use super::make_request;
use crate::model::images::*;
use crate::prelude::*;

pub struct Images {
    http: Arc<HttpClient>
}

impl Images {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    ///Gets a random image based in a given tag
    ///
    /// # Example
    /// ```rust,ignore
    /// if let Ok(res) = client.images.random_image("doge", false) {
    ///     match res {
    ///         Ok(image) => {
    ///             //do something with the image
    ///         },
    ///         Err(why) => {
    ///             //do something with the <ImageError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub fn random_image(&self, tag: impl ToString, nsfw: bool) -> HttpResult<Image, ImageError>{
        if tag.to_string().is_empty() { panic!("Tag param cannot be empty") }

        let builder = self.http.get(endpoint("/images/random-image").as_str())
            .query(&[("tag", tag.to_string())])
            .query(&[("nsfw", nsfw)]);


        make_request::<Image, ImageError>(builder)
    }

    ///Gets a random meme from reddit
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(meme) = client.images.random_meme() {
    ///     //do something here
    /// }
    /// ```
    pub fn random_meme(&self) -> reqwest::Result<RedditImage>{
        self.http.get(endpoint("/images/random-meme").as_str())
            .send()?
            .json::<RedditImage>()
    }

    ///Gets a random cute image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(aww) = client.images.random_aww() {
    ///     //do something with the image
    /// }
    /// ```
    pub fn random_aww(&self) -> reqwest::Result<RedditImage>{
        return self.http.get(endpoint("/images/random-aww").as_str())
            .send()?
            .json::<RedditImage>()
    }

    ///Gets a random post from a given subreddit
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.images.random_reddit("Technology", true, SpanType::Day) {
    ///     match res {
    ///         Ok(red) => {
    ///             //do something with the reddit image
    ///         },
    ///         Err(why) => {
    ///             //do something with the <ImageError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub fn random_reddit(&self, subreddit: impl ToString, remove_nsfw: bool, span: SpanType) -> HttpResult<RedditImage, ImageError>{
        if subreddit.to_string().is_empty() { panic!("You have to specify a subreddit to search in") }

        let builder = self.http.get(endpoint(format!("/images/rand-reddit/{}", subreddit.to_string())).as_str())
            .query(&[("remove_nsfw", remove_nsfw)])
            .query(&[("span", span.to_string())]);

        make_request::<RedditImage, ImageError>(builder)
    }

    ///Gets a random WikiHow image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(wiki_image) = client.images.random_wikihow(false) {
    ///     //do something with the image
    /// }
    /// ```
    pub fn random_wikihow(&self, nsfw: bool) -> reqwest::Result<WikiHowImage> {
        return self.http.get(endpoint("/images/random-wikihow").as_str())
            .query(&[("nsfw", nsfw)])
            .send()?
            .json::<WikiHowImage>()
    }

    ///Gets a list of all tags available
    ///
    /// # Example
    ///
    /// if let Ok(tags) = client.images.get_tags() {
    ///     //do something with all tags
    /// }
    /// ```
    pub fn get_tags(&self) -> reqwest::Result<TagList> {
        return self.http.get(endpoint("/images/tags").as_str())
            .send()?
            .json::<TagList>()
    }

    ///Gets an image using its Snowflake
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.images.get_image("i-8ta8p52f-27") {
    //     match res {
    //         Ok(img) => {
    //             //do something with the image
    //         },
    //         Err(why) => {
    //             //do something with the <ImageError> struct
    //         }
    //     }
    // }
    /// ```
    pub fn get_image(&self, sf: impl ToString) -> HttpResult<Image, ImageError> {
        if sf.to_string().is_empty() { panic!("Snowflake id cannot be empty") }

        let builder = self.http.get(endpoint(format!("/images/image/{}", sf.to_string())).as_str());

        make_request::<Image, ImageError>(builder)
    }

    ///Get a tag using its name
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(tag) = client.images.get_tag("doge") {
    ///     //do something with the tag
    /// }
    /// ```
    pub fn get_tag(&self, tag: impl ToString) -> reqwest::Result<TagList> {
        if tag.to_string().is_empty() { panic!("Tag cannot be empty") }

        self.http.get(endpoint(format!("/images/tags/{}", tag.to_string())).as_str())
            .send()?
            .json::<TagList>()
    }

    ///Get a random NSFW image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(nsfw) = client.images.random_nsfw(true) {
    ///     //do something with the image
    /// }
    /// ```
    pub fn random_nsfw(&self, gifs: bool) -> reqwest::Result<RedditImage> {
        self.http.get(endpoint("/images/random-nsfw").as_str())
            .query(&[("gifs", gifs)])
            .send()?
            .json::<RedditImage>()
    }
}