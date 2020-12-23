use crate::testrunners::Runner;
use async_trait::async_trait;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

// Spoon https://github.com/square/spoon
// Use latest master branch build
pub struct SpoonRunner {
    jar_file: String,
}

impl SpoonRunner {
    pub fn new(jar_file: String) -> Self {
        SpoonRunner { jar_file }
    }
}

#[async_trait]
impl Runner for SpoonRunner {
    async fn start(
        &self,
        jvm_args: Vec<String>,
        working_dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let stderr = File::create(working_dir.join("runner_stderr.txt"))
            .expect("Cannot create stderr log file");
        let stdout = File::create(working_dir.join("runner_stdout.txt"))
            .expect("Cannot create stdout log file");
        Command::new("java")
            .args(jvm_args)
            .args(vec!["-jar", &self.jar_file])
            .args(vec!["--apk", "application.apk"])
            .args(vec!["--test-apk", "test_application.apk"])
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
