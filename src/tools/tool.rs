use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait Tool {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn jvm_args(&self, output_dir: PathBuf) -> Vec<String>;
    fn get_name(&self) -> &str;
}
