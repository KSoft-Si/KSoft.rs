use reqwest::{Client as HttpClient};
use crate::{
    make_request,
    endpoint,
    model::*,
    HttpResult
};
use crate::model::images::*;
use crate::prelude::*;

pub struct Images {
    http: HttpClient
}

impl Images {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http: http_client
        }
    }

    ///Gets a random image based in a given tag
    ///
    /// # Example
    /// ```rust,ignore
    /// if let Ok(res) = client.images.random_image("doge", false).await {
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
    pub async fn random_image(&self, tag: impl ToString, nsfw: bool) -> HttpResult<Image, ImageError>{
        if tag.to_string().is_empty() { panic!("Tag param cannot be empty") }

        let builder = self.http.get(endpoint("/images/random-image").as_str())
            .query(&[("tag", tag.to_string())])
            .query(&[("nsfw", nsfw)]);


        make_request::<Image, ImageError>(builder).await
    }

    ///Gets a random meme from reddit
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(meme) = client.images.random_meme().await {
    ///     //do something here
    /// }
    /// ```
    pub async fn random_meme(&self) -> reqwest::Result<RedditImage>{
        let response = self.http.get(endpoint("/images/random-meme").as_str())
            .send()
            .await?;

        let image = response.json::<RedditImage>().await?;
        Ok(image)
    }

    ///Gets a random cute image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(aww) = client.images.random_aww().await {
    ///     //do something with the image
    /// }
    /// ```
    pub async fn random_aww(&self) -> reqwest::Result<RedditImage>{
        return self.http.get(endpoint("/images/random-aww").as_str())
            .send()
            .await?
            .json::<RedditImage>()
            .await
    }

    ///Gets a random post from a given subreddit
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.images.random_reddit("Technology", true, SpanType::Day).await {
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
    pub async fn random_reddit(&self, subreddit: impl ToString, remove_nsfw: bool, span: SpanType) -> HttpResult<RedditImage, ImageError>{
        if subreddit.to_string().is_empty() { panic!("You have to specify a subreddit to search in") }

        let builder = self.http.get(endpoint(format!("/images/rand-reddit/{}", subreddit.to_string())).as_str())
            .query(&[("remove_nsfw", remove_nsfw)])
            .query(&[("span", span.to_string())]);

        make_request::<RedditImage, ImageError>(builder).await
    }

    ///Gets a random WikiHow image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(wiki_image) = client.images.random_wikihow(false).await {
    ///     //do something with the image
    /// }
    /// ```
    pub async fn random_wikihow(&self, nsfw: bool) -> reqwest::Result<WikiHowImage> {
        return self.http.get(endpoint("/images/random-wikihow").as_str())
            .query(&[("nsfw", nsfw)])
            .send()
            .await?
            .json::<WikiHowImage>()
            .await
    }

    ///Gets a list of all tags available
    ///
    /// # Example
    ///
    /// if let Ok(tags) = client.images.get_tags().await {
    ///     //do something with all tags
    /// }
    /// ```
    pub async fn get_tags(&self) -> reqwest::Result<TagList> {
        return self.http.get(endpoint("/images/tags").as_str())
            .send()
            .await?
            .json::<TagList>()
            .await
    }

    ///Gets an image using its Snowflake
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.images.get_image("i-8ta8p52f-27").await {
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
    pub async fn get_image(&self, sf: impl ToString) -> HttpResult<Image, ImageError> {
        if sf.to_string().is_empty() { panic!("Snowflake id cannot be empty") }

        let builder = self.http.get(endpoint(format!("/images/image/{}", sf.to_string())).as_str());

        make_request::<Image, ImageError>(builder).await
    }

    ///Get a tag using its name
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(tag) = client.images.get_tag("doge").await {
    ///     //do something with the tag
    /// }
    /// ```
    pub async fn get_tag(&self, tag: impl ToString) -> reqwest::Result<TagList> {
        if tag.to_string().is_empty() { panic!("Tag cannot be empty") }

        let response = self.http.get(endpoint(format!("/images/tags/{}", tag.to_string())).as_str())
            .send()
            .await?;

        response.json::<TagList>().await
    }

    ///Get a random NSFW image
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(nsfw) = client.images.random_nsfw(true).await {
    ///     //do something with the image
    /// }
    /// ```
    pub async fn random_nsfw(&self, gifs: bool) -> reqwest::Result<RedditImage> {
        let response = self.http.get(endpoint("/images/random-nsfw").as_str())
            .query(&[("gifs", gifs)])
            .send()
            .await?;

        response.json::<RedditImage>().await
    }
}