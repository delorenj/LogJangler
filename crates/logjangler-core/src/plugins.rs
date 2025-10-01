use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    async fn init(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
}
