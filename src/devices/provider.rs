use async_trait::async_trait;

#[async_trait]
pub trait Provider {
    async fn prepare(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn terminate(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
