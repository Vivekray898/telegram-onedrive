/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use crate::{error::Result, message::TelegramMessage};
use proc_macros::{add_context, add_trace};

#[add_context]
#[add_trace]
pub async fn is_root_path_valid(root_path: &str, message: TelegramMessage) -> Result<bool> {
    if root_path.starts_with('/') {
        Ok(true)
    } else {
        let response = "directory path should start with /";
        message.reply(response).await.details(response)?;

        Ok(false)
    }
}
