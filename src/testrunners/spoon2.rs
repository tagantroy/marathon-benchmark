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
        let stderr = File::create(working_dir.join("runner_stderr.txt")).unwrap();
        let stdout = File::create(working_dir.join("runner_stdout.txt")).unwrap();
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

//
//usage: spoon-runner [-h] TEST_APK OTHER_APK... [--title TITLE] [-e KEY VALUE]
//                     [--class-name CLASS_NAME] [--method-name METHOD_NAME]
//                     [--small] [--output OUTPUT] [--sdk SDK] [--always-zero]
//                     [--allow-no-devices] [--sequential]
//                     [--init-script INIT_SCRIPT] [--grant-all] [--disable-gif]
//                     [--adb-timeout ADB_TIMEOUT] [--serial SERIAL]...
//                     [--skip-serial SKIP_SERIAL]... [--shard] [--debug]
//                     [--coverage] [--single-instrumentation-call]
//                     [--clear-app-data]
//
// optional arguments:
//   -h, --help                      show this help message and exit
//
//   --title TITLE                   Execution title
//
//   -e KEY VALUE, --es KEY VALUE    Instrumentation runner arguments.
//
//   --class-name CLASS_NAME         Fully-qualified test class to run
//
//   --method-name METHOD_NAME       Method name inside --class-name to run
//
//   --small, --medium, --large      Test size to run
//
//   --output OUTPUT                 Output path. Defaults to spoon-output/ in
//                                   the working directory if unset
//
//   --sdk SDK                       Android SDK path. Defaults to ANDROID_HOME
//                                   if unset.
//
//   --always-zero                   Always use 0 for the exit code regardless of
//                                   execution failure
//
//   --allow-no-devices              Do not fail if zero devices connected
//
//   --sequential                    Execute tests sequentially (one device at a
//                                   time)
//
//   --init-script INIT_SCRIPT       Script file executed between each devices
//
//   --grant-all                     Grant all runtime permissions during
//                                   installation on M+
//
//   --disable-gif                   Disable GIF generation
//
//   --adb-timeout ADB_TIMEOUT       Maximum execution time per test. Parsed by
//                                   java.time.Duration.
//
//   --serial SERIAL                 Device serials to use. If empty all devices
//                                   will be used.
//
//   --skip-serial SKIP_SERIAL       Device serials to skip
//
//   --shard                         Shard tests across all devices
//
//   --debug                         Enable debug logging
//
//   --coverage                      Enable code coverage
//
//   --single-instrumentation-call   Run all tests in a single instrumentation
//                                   call
//
//   --clear-app-data                Runs 'adb pm clear app.package.name' to
//                                   clear app data before each test.
//
//
// positional arguments:
//   TEST_APK                        Test APK
//
//   OTHER_APK                       Other APKs to install before test APK (e.g.,
//                                   main app or helper/buddy APKs)
//
