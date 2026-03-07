use anyhow::Context;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use pryr::ipc::IpcRequest;
use pryr::ipc::{IpcResponse, connect_ipc};
use std::io::Write;
use std::io::{BufRead, BufReader};

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

    let mut stream = connect_ipc().context("Could not connect to daemon. Is pryrd running?")?;

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
