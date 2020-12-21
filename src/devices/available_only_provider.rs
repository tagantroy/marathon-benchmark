use crate::devices::provider::Provider;
use async_trait::async_trait;

pub struct AvailableOnlyProvider {}

impl AvailableOnlyProvider {
    pub fn new() -> Self {
        AvailableOnlyProvider {}
    }
}

#[async_trait]
impl Provider for AvailableOnlyProvider {
    async fn prepare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    async fn terminate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
