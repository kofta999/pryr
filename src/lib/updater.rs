use self_update::TempDir;
use std::process::Command;

fn stop_daemon() -> anyhow::Result<()> {
    #[cfg(unix)]
    Command::new("systemctl")
        .args(["--user", "stop", "pryrd"])
        .status()?;

    #[cfg(windows)]
    Command::new("powershell")
        .args(["-Command", "Stop-ScheduledTask -TaskName PryrDaemon"])
        .status()?;

    anyhow::Ok(())
}

fn start_daemon() -> anyhow::Result<()> {
    #[cfg(unix)]
    Command::new("systemctl")
        .args(["--user", "start", "pryrd"])
        .status()?;

    #[cfg(windows)]
    Command::new("powershell")
        .args(["-Command", "Start-ScheduledTask -TaskName PryrDaemon"])
        .status()?;

    anyhow::Ok(())
}

pub fn run_update() -> anyhow::Result<()> {
    let repo_owner = "kofta999";
    let repo_name = "pryr";
    let current_version = env!("CARGO_PKG_VERSION");
    let current_exe = std::env::current_exe()?;
    let install_dir = current_exe.parent().unwrap().to_path_buf();
    unsafe {
        std::env::set_var("TMPDIR", TempDir::new_in(&install_dir)?.path());
    }

    let daemon_exe_name = if cfg!(windows) { "pryrd.exe" } else { "pryrd" };
    let daemon_install_path = install_dir.join(daemon_exe_name);

    stop_daemon()?;

    println!("Updating pryrd...");
    self_update::backends::github::Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .current_version(current_version)
        .bin_install_path(daemon_install_path)
        .bin_name(daemon_exe_name)
        .show_download_progress(true)
        .no_confirm(true)
        .build()?
        .update()?;

    println!("Updating pryr...");
    let status = self_update::backends::github::Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .current_version(current_version)
        .bin_name("pryr")
        .show_download_progress(true)
        .no_confirm(true)
        .build()?
        .update()?;

    start_daemon()?;

    if status.updated() {
        println!(
            "✨ Successfully updated pryr to version {}!",
            status.version()
        );
    } else {
        println!("You are already on the latest version ({current_version}).");
    }

    Ok(())
}
