use crate::devices::provider::Provider;
use async_trait::async_trait;
use port_scanner::request_open_port;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;

pub struct DockerProvider {
    image: String,
    tag: String,
    number_of_emulators: u32,
    running_containers: HashMap<u32, RunningContainer>,
}

struct RunningContainer {
    id: String,
    port: u16,
}

impl RunningContainer {
    fn new(id: String, port: u16) -> Self {
        RunningContainer { id, port }
    }
}

impl DockerProvider {
    pub fn new(image: String, tag: String, number_of_emulators: u32) -> Self {
        DockerProvider {
            image,
            tag,
            number_of_emulators,
            running_containers: HashMap::new(),
        }
    }

    async fn start_container(&mut self, idx: u32) -> Result<(), Box<dyn std::error::Error>> {
        let port = request_open_port().expect("Cannot get next open port");
        let home_dir = home::home_dir().expect("Home dir is not available for this user");
        let adbkey_path = home_dir.join(".android").join("adbkey");
        let adbkey = std::fs::read_to_string(adbkey_path).expect("cannot read ~/.android/adbkey");

        let image = format!("{}:{}", self.image, self.tag);
        let adb_key_env = format!("ADBKEY=\"{}\"", adbkey);

        let output = Command::new("docker")
            .args(vec!["run", "--rm", "-d", "--privileged"])
            .args(vec!["-e".to_string(), adb_key_env])
            .args(vec!["--device", "/dev/kvm"])
            .args(vec!["--publish".to_string(), format!("{}:5555", port)])
            .arg(image)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;
        let stdout = std::str::from_utf8(&output.stdout).expect("Cannot get container id");
        self.running_containers
            .insert(idx, RunningContainer::new(stdout.to_string(), port));
        Ok(())
    }

    async fn adb_connect(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("localhost:{}", port);
        let output = Command::new("adb")
            .arg("connect")
            .arg(&address)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;
        let stdout = std::str::from_utf8(&output.stdout).expect("Cannot get adb connect response");

        let expected = format!("connected to {}", address);
        if stdout.contains(&expected) {
            Ok(())
        } else {
            Err(Box::new(Error::new(
                ErrorKind::ConnectionRefused,
                "Cannot connect to device",
            )))
        }
    }

    async fn stop_containers(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ids: Vec<String> = self
            .running_containers
            .values()
            .map(|c| &c.id)
            .map(|id| short_docker_id(id))
            .collect();
        Command::new("docker")
            .arg("stop")
            .args(ids)
            .stdout(Stdio::piped())
            .spawn()?
            .wait()
            .await?;
        Ok(())
    }
}

fn short_docker_id(full_id: &str) -> String {
    let (id, _) = full_id.split_at(12);
    id.to_string()
}

#[async_trait]
impl Provider for DockerProvider {
    async fn prepare(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for idx in 0..self.number_of_emulators {
            self.start_container(idx).await?;
        }
        Ok(())
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        for container in self.running_containers.values() {
            tryhard::retry_fn(|| self.adb_connect(container.port))
                .retries(10)
                .fixed_backoff(Duration::from_millis(1000))
                .await?;
        }
        Ok(())
    }

    async fn terminate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_containers().await?;
        Ok(())
    }
}
