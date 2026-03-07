use anyhow::Context;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use pryr::ipc::IpcResponse;
use pryr::{ipc::IpcRequest, system::get_socket_path};
use std::io::{BufRead, BufReader};
use std::{io::Write, os::unix::net::UnixStream};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Schedule,
    ReloadConfig,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let request: IpcRequest = match cli.command {
        Commands::Status => IpcRequest::GetStatus,
        Commands::Schedule => IpcRequest::GetTodaySchedule,
        Commands::ReloadConfig => IpcRequest::ReloadConfig,
    };

    let stream_path =
        get_socket_path().ok_or_else(|| anyhow::anyhow!("Could not determine socket path"))?;
    let mut stream = UnixStream::connect(stream_path)
        .context("Could not connect to daemon. Is pryrd running?")?;

    let request_string = serde_json::to_string(&request)? + "\n";
    stream.write_all(request_string.as_bytes())?;
    stream.flush()?;

    let mut buf_reader = BufReader::new(&stream);
    let mut s = String::new();

    buf_reader.read_line(&mut s)?;

    let response: IpcResponse = serde_json::from_str(&s)?;

    match response {
        IpcResponse::CurrentState(daemon_state) => println!("{daemon_state}"),
        IpcResponse::DailySchedule(prayer_today_schedule) => println!("{prayer_today_schedule}"),
        IpcResponse::Success => println!("{}", "Success".green()),
        IpcResponse::Error(e) => println!("{}: {}", "Error".red(), e),
    }

    Ok(())
}
