use mozdevice::DeviceInfo;

use crate::adb_server::restart_adb_server;
use crate::benchmark_results::ExecutionReport;
use crate::config::{Config, DeviceProvider, TestRunner};
use crate::devices::{AvailableOnlyProvider, DockerProvider, LocalEmulatorProvider, Provider};
use crate::file_manager::FileManager;
use crate::monitoring::{ProcessMonitoring, SystemMonitoring};
use crate::run_config::RunConfig;
use crate::testrunners::{ForkRunner, MarathonRunner, Runner, SpoonRunner, SpoonRunner2};
use crate::tools::{FlightRecorder, Tool};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

impl Into<Vec<TestSuiteRunner>> for Config {
    fn into(self) -> Vec<TestSuiteRunner> {
        self.test_suites
            .iter()
            .map(|suite| {
                let provider: Box<dyn Provider> = match &suite.device_provider {
                    DeviceProvider::Docker { image, tag } => Box::new(DockerProvider::new(
                        image.clone(),
                        tag.clone(),
                        suite.emulators,
                    )),
                    DeviceProvider::LocalEmulator { name } => {
                        Box::new(LocalEmulatorProvider::new(name.clone(), suite.emulators))
                    }
                    DeviceProvider::AvailableOnly => Box::new(AvailableOnlyProvider::new()),
                };
                let runner: Box<dyn Runner> = match &suite.test_runner {
                    TestRunner::Fork {
                        program,
                        config_file,
                    } => Box::new(ForkRunner::new(program.clone(), config_file.clone())),
                    TestRunner::Marathon {
                        program,
                        marathon_file,
                    } => Box::new(MarathonRunner::new(program.clone(), marathon_file.clone())),
                    TestRunner::Spoon { jar_file } => Box::new(SpoonRunner::new(jar_file.clone())),
                    TestRunner::Spoon2 { program } => Box::new(SpoonRunner2::new(program.clone())),
                };
                TestSuiteRunner {
                    name: suite.name.clone(),
                    apk: self.input.apk.clone(),
                    test_apk: self.input.test_apk.clone(),
                    iterations: suite.iterations,
                    emulators: suite.emulators,
                    provider,
                    runner,
                }
            })
            .collect()
    }
}

pub struct TestSuiteRunner {
    name: String,
    apk: String,
    test_apk: String,
    iterations: u32,
    emulators: u32,
    provider: Box<dyn Provider>,
    runner: Box<dyn Runner>,
}

impl TestSuiteRunner {
    async fn run_iteration(
        &mut self,
        idx: u32,
        run_config: &RunConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let spinner_style = ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{prefix:.bold.dim} {spinner} {wide_msg}");
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(10);
        spinner.set_style(spinner_style);
        spinner.set_prefix(&format!("[{}] Iteration #{}", self.name, idx));

        let file_manager = FileManager::new(
            run_config.uuid,
            run_config.working_dir.clone(),
            self.name.clone(),
            idx,
        );

        let working_dir = file_manager.get_working_dir()?;

        file_manager.prepare_working_dir(
            &self.apk,
            &self.test_apk,
            self.runner.required_files(),
        )?;

        spinner.set_message("Prepare devices");
        restart_adb_server().await?;
        self.provider.prepare().await?;

        spinner.set_message("Connect devices");
        self.provider.connect().await?;

        spinner.set_message("Wait for devices");
        wait_for_devices(self.emulators);

        let tool = FlightRecorder::default();
        let results_dir = file_manager.get_tools_results_dir(&tool.get_name())?;
        tool.start().await?;

        spinner.set_message("Start monitoring");
        let process_monitoring = ProcessMonitoring::new();
        let system_monitoring = SystemMonitoring::new();
        process_monitoring.start();
        system_monitoring.start();

        spinner.set_message("Run tests");
        let test_run_start = Instant::now();
        let result = self
            .runner
            .start(tool.jvm_args(results_dir), working_dir)
            .await;
        let test_run_end = Instant::now();
        let report = ExecutionReport::new(self.name.clone(), idx, test_run_start, test_run_end);

        file_manager.save_execution_report(report).await?;

        tool.stop().await?;

        spinner.set_message("Terminate");
        file_manager.clean_up_working_dir()?;
        self.provider.terminate().await?;
        result
    }

    pub async fn start(
        &mut self,
        run_config: &RunConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for i in 1..self.iterations + 1 {
            self.run_iteration(i, run_config).await?;
        }
        Ok(())
    }
}

fn wait_for_devices(expected: u32) {
    let host = mozdevice::Host::default();
    let mut devices: Vec<DeviceInfo> = host.devices().expect("Cannot get all devices");
    while devices.len() < expected as usize {
        devices = host.devices().expect("Cannot get all devices");
    }
}
