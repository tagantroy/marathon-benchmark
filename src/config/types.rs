use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub input: Input,
    pub test_suites: Vec<TestSuite>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub apk: String,
    pub test_apk: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceProvider {
    Docker { image: String, tag: String },
    LocalEmulator { name: String },
    AvailableOnly,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestRunner {
    Fork {
        program: String,
        config_file: String,
    },
    Marathon {
        program: String,
        marathon_file: String,
    },
    Spoon {
        jar_file: String,
    },
    Spoon2 {
        program: String,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub iterations: u32,
    pub emulators: u32,
    pub device_provider: DeviceProvider,
    pub test_runner: TestRunner,
}
