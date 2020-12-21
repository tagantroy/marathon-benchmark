use crate::devices::provider::Provider;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::process::{Child, Command};

pub struct LocalEmulatorProvider {
    name: String,
    number_of_emulators: u32,
    processes: HashMap<u32, Child>,
}

impl LocalEmulatorProvider {
    pub fn new(name: String, number_of_emulators: u32) -> Self {
        LocalEmulatorProvider {
            name,
            number_of_emulators,
            processes: Default::default(),
        }
    }

    async fn start_emulator(&mut self, idx: u32) -> Result<(), Box<dyn std::error::Error>> {
        let process = Command::new("/home/ivanbalaksha/Android/Sdk/emulator/emulator")
            .arg("-no-window")
            .arg("-read-only")
            .arg(self.name.clone())
            .spawn()?;
        self.processes.insert(idx, process);
        Ok(())
    }
}

#[async_trait]
impl Provider for LocalEmulatorProvider {
    async fn prepare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..self.number_of_emulators {
            self.start_emulator(i).await?;
        }
        Ok(())
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn terminate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (_key, child) in self.processes.iter_mut() {
            child.kill().await?;
        }
        Ok(())
    }
}
