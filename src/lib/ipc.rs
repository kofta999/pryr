use crate::{daemon::DaemonState, prayers::PrayerTodaySchedule};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcRequest {
    GetStatus,
    GetTodaySchedule,
    ReloadConfig,
}

#[derive(Serialize, Deserialize)]
pub enum IpcResponse {
    CurrentState(DaemonState),          // Response for GetStatus
    DailySchedule(PrayerTodaySchedule), // Response for GetTodaySchedule
    Success,                            // Response for ReloadConfig
    Error(String),
}

#[cfg(unix)]
pub struct IpcListener(tokio::net::UnixListener);

#[cfg(unix)]
impl IpcListener {
    pub async fn bind() -> anyhow::Result<Self> {
        let socket_path =
            crate::system::get_socket_path().ok_or(anyhow!("Socket path does not exist"))?;
        // Ignore error if socket doesn't exist
        let _ = tokio::fs::remove_file(&path).await;
        Ok(Self(tokio::net::UnixListener::bind(socket_path)?))
    }

    pub async fn accept(&self) -> anyhow::Result<tokio::net::UnixStream> {
        let (stream, _) = self.0.accept().await?;
        Ok(stream)
    }
}

#[cfg(unix)]
pub fn connect_ipc() -> anyhow::Result<std::os::unix::net::UnixStream> {
    let path = crate::system::get_socket_path().unwrap();
    Ok(std::os::unix::net::UnixStream::connect(path)?)
}

#[cfg(windows)]
pub struct IpcListener;

#[cfg(windows)]
impl IpcListener {
    pub async fn bind() -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub async fn accept(&self) -> anyhow::Result<tokio::net::windows::named_pipe::NamedPipeServer> {
        use anyhow::Ok;
        use tokio::net::windows::named_pipe::ServerOptions;
        let server = ServerOptions::new().create(r"\\.\pipe\pryr-ipc")?;
        server.connect().await?;

        Ok(server)
    }
}

#[cfg(windows)]
pub fn connect_ipc() -> anyhow::Result<std::fs::File> {
    Ok(std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(r"\\.\pipe\pryr-ipc")?)
}
