use crate::testrunners::Runner;
use async_trait::async_trait;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

// Spoon https://github.com/square/spoon
// Use latest master branch build
pub struct SpoonRunner2 {
    program: String,
}

impl SpoonRunner2 {
    pub fn new(program: String) -> Self {
        SpoonRunner2 { program }
    }
}

#[async_trait]
impl Runner for SpoonRunner2 {
    async fn start(
        &self,
        jvm_args: Vec<String>,
        working_dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let stderr = File::create(working_dir.join("runner_stderr.txt"))
            .expect("Cannot create stderr log file");
        let stdout = File::create(working_dir.join("runner_stdout.txt"))
            .expect("Cannot create stdout log file");
        Command::new(&self.program)
            .env("SPOON_RUNNER_OPTS", jvm_args.join(" "))
            .args(vec!["test_application.apk", "application.apk"])
            .arg("--shard")
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .current_dir(working_dir)
            .spawn()?
            .wait()
            .await?;
        Ok(())
    }

    fn required_files(&self) -> Vec<String> {
        vec![]
    }
}
