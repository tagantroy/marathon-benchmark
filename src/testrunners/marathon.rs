use crate::testrunners::Runner;
use async_trait::async_trait;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

// Marathon  https://github.com/Malinskiy/marathon
pub struct MarathonRunner {
    program: String,
    marathon_file: String,
}

impl MarathonRunner {
    pub fn new(program: String, marathon_file: String) -> Self {
        MarathonRunner {
            program,
            marathon_file,
        }
    }
}

#[async_trait]
impl Runner for MarathonRunner {
    async fn start(
        &self,
        jvm_args: Vec<String>,
        working_dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let config_path = Path::new(&self.marathon_file);
        let config_file_name = config_path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let stderr = File::create(working_dir.join("runner_stderr.txt")).unwrap();
        let stdout = File::create(working_dir.join("runner_stdout.txt")).unwrap();

        Command::new(&self.program)
            .env("MARATHON_OPTS", jvm_args.join(" ")) //
            .args(vec!["-m", &config_file_name])
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .current_dir(working_dir)
            .spawn()?
            .wait()
            .await?;
        Ok(())
    }

    fn required_files(&self) -> Vec<String> {
        vec![self.marathon_file.clone()]
    }
}
