//!
//! Support for Slack Bots API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::*;

impl<'a> SlackClientSession<'a> {
    ///
    /// https://api.slack.com/methods/bots.info
    ///
    pub async fn bots_info(
        &self,
        req: &SlackApiBotsInfoRequest,
    ) -> ClientResult<SlackApiBotsInfoResponse> {
        self.http_api
            .http_get("bots.info", &vec![("bot", req.bot.as_ref())])
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBotsInfoRequest {
    pub bot: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBotsInfoResponse {
    pub bot: SlackBotInfo,
}
