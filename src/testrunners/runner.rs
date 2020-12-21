use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait Runner {
    async fn start(
        &self,
        jvm_args: Vec<String>,
        working_dir: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn required_files(&self) -> Vec<String>;
}
