use crate::tools::tool::Tool;
use async_trait::async_trait;
use std::error::Error;
use std::path::PathBuf;

pub struct FlightRecorder {}

impl Default for FlightRecorder {
    fn default() -> Self {
        FlightRecorder {}
    }
}

#[async_trait]
impl Tool for FlightRecorder {
    async fn start(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn jvm_args(&self, output_dir: PathBuf) -> Vec<String> {
        let file_path_os_str = output_dir.join("report.jfr").into_os_string();
        let file_path = file_path_os_str.to_str().unwrap();
        std::fs::create_dir_all(output_dir);
        let params = format!("-XX:StartFlightRecording=filename={}", file_path);
        vec!["-XX:+FlightRecorder".to_owned(), params]
    }

    fn get_name(&self) -> &str {
        "flightrecorder"
    }
}
