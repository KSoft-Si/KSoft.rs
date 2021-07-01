use crate::{make_request, endpoint, model::*, HttpResult, EventHandler};
use reqwest::{Client as HttpClient};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::model::bans::*;
use tracing::warn;

pub struct Bans {
    http: HttpClient
}

impl Bans {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http: http_client,
        }
    }

    pub(crate) fn event_handler(&self, handler: impl EventHandler + Send + Sync + 'static) {
        let client = self.http.clone();
        tokio::spawn(async move {
            let delay = tokio::time::Duration::from_secs(5 * 60);
            let endpoint = endpoint("/bans/updates");

            let mut last_check = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() - 60 * 10;

            loop {
                match client.get(endpoint.as_str())
                    .query(&[("timestamp", last_check)])
                    .send()
                    .await {
                    Ok(res) => {
                        match res.json().await {
                            Ok(RawBanUpdate { data, timestamp }) => {
                                last_check = timestamp;
                                if data.len() >= 1 {
                                    handler.ban_updated(data).await;
                                }
                            },
                            Err(e) => {
                                warn!("Something went wrong when deserializing ban updates response from server: {:#?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        warn!("KSoft.si server responded with an error while trying to get ban updates: {:#?}", e);
                    }
                }

                tokio::time::sleep(delay).await;
            }
        });
    }

    ///Get a list of X number of bans from X page
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.advanced_paginate(2, 20).await {
    ///     match res {
    ///         Ok(bans) => {
    ///             //do something with ban list
    ///         },
    ///         Err(why) => {
    ///             //do something with the <BanError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn advanced_paginate(&self, page: u8, per_page: u8) -> HttpResult<BanList, BanError>{
        let builder = self.http.get(endpoint("/bans/list").as_str())
            .query(&[("per_page", per_page)])
            .query(&[("page", page)]);

        make_request::<BanList, BanError>(builder).await
    }

    ///Shortcut to advanced_paginate() but with default parameters
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.paginate().await {
    ///     match res {
    ///         Ok(bans) => {
    ///             //do something with ban list
    ///         },
    ///         Err(why) => {
    ///             //do something with the <BanError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn paginate(&self) -> HttpResult<BanList, BanError> {
        self.advanced_paginate(1, 20).await
    }

    /// Reports an user
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.add(23123123, "some reason", "some proof", None, None, None, Some(true)).await {
    ///     match res {
    ///         Ok(response) => {
    ///             //Do something with the response
    ///         },
    ///         Err(why) => {
    ///             //Domething with <BanError>
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn add<S: ToString>(&self,
      user_id: u64,
      reason: S,
      proof: S,
      moderator: Option<u64>,
      user_name: Option<String>,
      user_discriminator: Option<u16>,
      appeal_possible: Option<bool>)
    -> HttpResult<BanAdditionResponse, BanError>{
        if reason.to_string().is_empty() { panic!("Reason param cannot be empty") }
        if proof.to_string().is_empty() { panic!("Proof param cannot be empty") }

        let builder = self.http.post(endpoint("/bans/add").as_str())
            .form(&BanAddition {
                user_id,
                reason: reason.to_string(),
                proof: proof.to_string(),
                moderator,
                user_name,
                user_discriminator,
                appeal_possible
            });

        make_request::<BanAdditionResponse, BanError>(builder).await
    }

    ///Check if user is banned ny its id
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(ban) = client.bans.check_ban(12335454).await {
    ///     //do something with the ban
    /// }
    /// ```
    pub async fn check_ban(&self, user_id: u64) -> reqwest::Result<BanCheckResponse> {
        let response = self.http.get(endpoint("/bans/check").as_str())
            .query(&[("user", user_id)])
            .send()
            .await?;

        response.json::<BanCheckResponse>().await
    }

    ///Retrieve info about a ban
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.ban_info(1231231234124).await {
    ///     match res {
    ///         Ok(ban) => {
    ///             //do something with ban info
    ///         },
    ///         Err(why) => {
    ///             //do something with the <BanError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn ban_info(&self, user_id: u64) -> HttpResult<BanInfoResponse, BanError> {
        let builder = self.http.get(endpoint("/bans/info").as_str())
            .query(&[("user", user_id)]);

        make_request::<BanInfoResponse, BanError>(builder).await
    }

    ///Forces the deletion of an user ban. **Must have BAN_MANAGER permission on ksoft to use it**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.delete_forcing(1231231234124).await {
    ///     match res {
    ///         Ok(ban) => {
    ///             //do something with ban info
    ///         },
    ///         Err(why) => {
    ///             //do something with the <BanError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn delete_forcing(&self, user_id: u64) -> HttpResult<BanDeletionResponse, BanError> {
        let builder = self.http.delete(endpoint("/bans/delete").as_str())
            .query(&[("user", user_id)])
            .query(&[("force", true)]);

        make_request::<BanDeletionResponse, BanError>(builder).await
    }

    ///Deletes an user ban. **Must have BAN_MANAGER permission on ksoft to use it**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.bans.delete(1231231234124).await {
    ///     match res {
    ///         Ok(ban) => {
    ///             //do something with ban info
    ///         },
    ///         Err(why) => {
    ///             //do something with the <BanError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn delete(&self, user_id: u64) -> HttpResult<BanDeletionResponse, BanError> {
        let builder = self.http.delete(endpoint("/bans/delete").as_str())
            .query(&[("user", user_id)]);

        make_request::<BanDeletionResponse, BanError>(builder).await
    }
}