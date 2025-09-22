use crate::notifications::Notifier;
use anyhow::Result;
use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;

pub struct DiscordNotifier {
    webhook: DiscordWebhook,
    username: String,
}

impl DiscordNotifier {
    pub fn new(webhook_url: impl Into<String>, username: impl Into<String>) -> Result<Self> {
        let webhook = DiscordWebhook::new(webhook_url.into())?;
        Ok(Self {
            webhook,
            username: username.into(),
        })
    }
}

#[async_trait::async_trait]
impl Notifier for DiscordNotifier {
    async fn send(&self, content: String) -> Result<()> {
        self.webhook
            .send(&Message::new(|m| {
                m.content(content).username(self.username.clone())
            }))
            .await?;
        Ok(())
    }
}
