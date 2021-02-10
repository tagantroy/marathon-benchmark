use crate::benchmark_results::ExecutionReport;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct FileManager {
    uuid: Uuid,
    working_dir: PathBuf,
    suite_name: String,
    iteration: u32,
}

impl FileManager {
    pub fn new(uuid: Uuid, working_dir: PathBuf, suite_name: String, iteration: u32) -> Self {
        FileManager {
            uuid,
            working_dir,
            suite_name,
            iteration,
        }
    }

    pub async fn save_execution_report(
        &self,
        report: ExecutionReport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let report = serde_json::to_string(&report)?;
        let working_dir = self.get_working_dir()?;
        let path = working_dir.join("execution_report.json");
        std::fs::write(path, report)?;
        Ok(())
    }

    pub fn prepare_working_dir(
        &self,
        apk: &str,
        test_apk: &str,
        files: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let working_dir = self.get_working_dir()?;

        std::fs::create_dir_all(&working_dir).expect("Cannot create working dir");

        std::fs::copy(apk, working_dir.join("application.apk"))
            .expect("Cannot copy application.apk");
        std::fs::copy(test_apk, working_dir.join("test_application.apk"))
            .expect("Cannot copy test_application.apk");

        for f in files {
            let file_name = Path::new(&f).file_name().expect("Expected file name");
            let dest = working_dir.join(file_name);
            std::fs::copy(&f, &dest)
                .expect(format!("Cannot copy file: {} to {:?}", f, &dest).as_str());
        }
        Ok(())
    }

    pub fn clean_up_working_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        let working_dir = self.get_working_dir()?;
        std::fs::remove_file(working_dir.join("application.apk"))?;
        std::fs::remove_file(working_dir.join("test_application.apk"))?;
        Ok(())
    }

    pub fn get_working_dir(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(self.get_run_dir())
    }

    pub fn get_tools_results_dir(
        &self,
        tool_name: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(self.get_results_dir_for_iteration()?.join(tool_name))
    }

    pub fn get_results_dir_for_iteration(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let dir = self.get_results_dir();
        Ok(dir)
    }

    fn get_results_dir(&self) -> PathBuf {
        self.get_run_dir().join("results")
    }

    fn get_run_dir(&self) -> PathBuf {
        let iteration_str = format!("{}", self.iteration);
        Path::new(self.working_dir.as_path())
            .join(self.uuid.to_string())
            .join(&self.suite_name)
            .join(&iteration_str)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_run_dir() {
        let uuid = Uuid::new_v4();
        let suite_name = "test_suite".to_owned();
        let iteration = 1;
        let working_dir = PathBuf::from("/test/marathon/dir");
        let file_manager =
            FileManager::new(uuid, working_dir.clone(), suite_name.clone(), iteration);
        let run_dir = file_manager.get_run_dir();
        let run_dir_str = run_dir.to_str().expect("Cannot convert path to string");
        let expected = &format!(
            "{}/{}/{}/{}",
            working_dir.to_str().expect("Cannot convert path to string"),
            uuid.to_string(),
            suite_name,
            iteration
        );
        assert_eq!(expected, run_dir_str)
    }

    #[test]
    fn test_get_results_dir() {
        let uuid = Uuid::new_v4();
        let suite_name = "test_suite".to_owned();
        let iteration = 1;
        let working_dir = PathBuf::from("/test/marathon/dir");
        let file_manager =
            FileManager::new(uuid, working_dir.clone(), suite_name.clone(), iteration);
        let results_dir = file_manager.get_results_dir();
        let results_dir_str = results_dir.to_str().expect("Cannot convert path to string");
        let expected = &format!(
            "{}/{}/{}/{}/{}",
            working_dir.to_str().expect("Cannot convert path to string"),
            uuid.to_string(),
            suite_name,
            iteration,
            "results"
        );
        assert_eq!(expected, results_dir_str)
    }

    #[test]
    fn test_get_results_dir_for_iteration() {
        let uuid = Uuid::new_v4();
        let suite_name = "test_suite".to_owned();
        let iteration = 5;
        let working_dir = PathBuf::from("/test/marathon/dir");
        let file_manager =
            FileManager::new(uuid, working_dir.clone(), suite_name.clone(), iteration);
        let results_dir = file_manager
            .get_results_dir_for_iteration()
            .expect("Cannot convert path to string");
        let results_dir_str = results_dir.to_str().expect("Cannot convert path to string");
        let expected = &format!(
            "{}/{}/{}/{}/{}",
            working_dir.to_str().expect("Cannot convert path to string"),
            uuid.to_string(),
            suite_name,
            iteration,
            "results"
        );
        assert_eq!(expected, results_dir_str)
    }
}
