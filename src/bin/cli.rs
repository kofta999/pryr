use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use owo_colors::OwoColorize;
use pryr::config::{Config, Location};
use pryr::ipc::IpcRequest;
use pryr::ipc::{IpcResponse, connect_ipc};
use pryr::system::get_config_path;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process;

const LOCATION_API_BASE_URL: &str = "https://nominatim.openstreetmap.org/search?format=json";

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
    Configure(ConfigureArgs),
}

#[derive(Args)]
struct ConfigureArgs {
    #[arg(short, long)]
    city: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let request: IpcRequest = match cli.command {
        Commands::Status => IpcRequest::GetStatus,
        Commands::Schedule => IpcRequest::GetTodaySchedule,
        Commands::ReloadConfig => IpcRequest::ReloadConfig,
        Commands::Configure(args) => match get_location(&args.city) {
            Some(location) => {
                let config_path = get_config_path().context("Could not get config path")?;
                let mut config = Config::from_file(&config_path)?;

                config.location = location;
                config.save(&config_path)?;

                IpcRequest::ReloadConfig
            }
            None => {
                eprintln!("Could not find location for this city");
                process::exit(1)
            }
        },
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

// TODO: Make a menu with selections so the user won't pick wrong location
fn get_location(city: &str) -> Option<Location> {
    let mut res = ureq::get(format!("{LOCATION_API_BASE_URL}&q=${city}"))
        .call()
        .expect("Couldn't send request to Location API");
    let res = res
        .body_mut()
        .read_to_string()
        .expect("Couldn't read API response");

    let value: serde_json::Value =
        serde_json::from_str(&res).expect("Couldn't convert response to JSON");
    let locations = value.as_array()?;

    let first_match = locations.first()?;

    Some(Location {
        long: first_match["lon"]
            .as_str()?
            .parse::<f64>()
            .expect("Couldn't parse longitude"),
        lat: first_match["lat"]
            .as_str()?
            .parse::<f64>()
            .expect("Couldn't parse latitude"),
    })
}
