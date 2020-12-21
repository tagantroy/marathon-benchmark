use crate::testrunners::Runner;
use async_trait::async_trait;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

// Fork runner https://github.com/shazam/fork
pub struct ForkRunner {
    program: String,
    config_file: String,
}

impl ForkRunner {
    pub fn new(program: String, config_file: String) -> Self {
        ForkRunner {
            program,
            config_file,
        }
    }
}

#[async_trait]
impl Runner for ForkRunner {
    async fn start(
        &self,
        jvm_args: Vec<String>,
        working_dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let config_path = Path::new(&self.config_file);
        let config_file_name = config_path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let stderr = File::create(working_dir.join("runner_stderr.txt")).unwrap();
        let stdout = File::create(working_dir.join("runner_stdout.txt")).unwrap();
        Command::new(&self.program)
            .env("FORK_RUNNER_OPTS", jvm_args.join(" "))
            .args(vec!["--apk", "application.apk"])
            .args(vec!["--test-apk", "test_application.apk"])
            .args(vec!["--config", &config_file_name])
            .current_dir(working_dir)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()?
            .wait()
            .await?;
        Ok(())
    }

    fn required_files(&self) -> Vec<String> {
        vec![self.config_file.clone()]
    }
}
