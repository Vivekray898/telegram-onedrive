/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

mod events;
mod handler;

use grammers_client::Update;
use proc_macros::add_trace;
use std::rc::Rc;
use std::sync::Arc;

pub use events::{EventType, HashMapExt};

use events::Events;
use handler::Handler;

use crate::env::BYPASS_PREFIX;
use crate::error::{Result, ResultUnwrapExt};
use crate::message::TelegramMessage;
use crate::state::{AppState, State};
use crate::tasker::Tasker;

pub struct Listener {
    pub events: Rc<Events>,
    pub state: AppState,
}

impl Listener {
    #[add_trace]
    pub async fn new(events: Events) -> Self {
        let events = Rc::new(events);
        let state = Arc::new(State::new().await);

        Self { events, state }
    }

    #[add_trace]
    pub async fn run(self) {
        tracing::debug!("listener started");

        let tasker = Tasker::new(self.state.clone()).await.unwrap();
        tokio::spawn(async move {
            tasker.run().await;
        });

        loop {
            if let Err(e) = self.handle_message().await {
                e.trace();
            }
        }
    }

    #[add_trace(context)]
    async fn handle_message(&self) -> Result<()> {
        let handler = Handler::new(self.events.clone(), self.state.clone());

        let client = &handler.state.telegram_bot;

        let update = client.next_update().await?;

        if let Some(Update::NewMessage(message_raw)) = update {
            if !message_raw.outgoing() && !message_raw.text().starts_with(BYPASS_PREFIX) {
                let message = TelegramMessage::new(client.clone(), message_raw);

                if let Err(e) = handler.handle_message(message.clone()).await {
                    e.send(message).await.unwrap_both().trace()
                }
            }
        }

        Ok(())
    }
}
