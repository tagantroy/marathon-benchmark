use std::process::Stdio;
use tokio::process::Command;

pub async fn restart_adb_server() -> Result<(), Box<dyn std::error::Error>> {
    kill_adb_server().await?;
    start_adb_server().await?;
    Ok(())
}

async fn start_adb_server() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("adb")
        .arg("start-server")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Ok(())
}

async fn kill_adb_server() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("adb")
        .arg("kill-server")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Ok(())
}
