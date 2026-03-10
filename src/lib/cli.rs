use crate::ipc::IpcResponse;
use owo_colors::OwoColorize;

pub trait PrettyPrint {
    fn pretty_print(&self);
}

impl PrettyPrint for IpcResponse {
    fn pretty_print(&self) {
        match self {
            IpcResponse::CurrentState(daemon_state) => println!("{daemon_state}"),
            IpcResponse::DailySchedule(prayer_today_schedule) => {
                println!("{prayer_today_schedule}")
            }
            IpcResponse::Success => println!("{}", "Success".green()),
            IpcResponse::Error(e) => println!("{}: {}", "Error".red(), e),
        }
    }
}
