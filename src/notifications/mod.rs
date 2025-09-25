mod discord;

pub use discord::DiscordNotifier;

#[async_trait::async_trait]
pub trait Notifier: Send + Sync {
    async fn send(&self, content: String) -> anyhow::Result<()>;
}
