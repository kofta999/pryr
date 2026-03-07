use pryr::{
    config::Config,
    daemon::{DaemonSnapShot, DaemonState},
    ipc::{IpcRequest, IpcResponse},
    prayer_manager::{ActionableEvent, PrayerManager},
    system::System,
};
use salah::{Local, Utc};
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::{mpsc, watch},
};

const LOCKDOWN_POLL_SECONDS: u64 = 10;
const LOCKDOWN_DURATION_SECONDS: u64 = 10 * 60;

#[tokio::main]
async fn main() {
    let config = Config::from_file("test-config.toml").expect("Couldn't parse Configuration File");
    let (watch_tx, watch_rx) = watch::channel(DaemonSnapShot::default());
    let (mpsc_tx, mpsc_rx) = mpsc::channel::<IpcRequest>(10);

    let h1 = tokio::spawn(daemon_loop(config, watch_tx, mpsc_rx));
    let h2 = tokio::spawn(ipc_server_loop(watch_rx, mpsc_tx));

    h1.await.unwrap().unwrap();
    h2.await.unwrap().unwrap();
}

async fn ipc_server_loop(
    watch_rx: watch::Receiver<DaemonSnapShot>,
    mpsc_tx: mpsc::Sender<IpcRequest>,
) -> anyhow::Result<()> {
    let socket_path = "/run/user/1000/pryr.sock";

    // Ignore error if socket doesn't exist
    let _ = tokio::fs::remove_file(socket_path).await;

    let socket = tokio::net::UnixListener::bind(socket_path)?;

    loop {
        let (mut stream, _) = socket.accept().await?;
        let watch_rx_clone = watch_rx.clone();
        let mpsc_tx_clone = mpsc_tx.clone();

        tokio::spawn(async move {
            let (read_half, mut write_half) = stream.split();
            let mut buf_reader = BufReader::new(read_half);
            let mut s = String::new();
            buf_reader.read_line(&mut s).await?;

            let request: IpcRequest = serde_json::from_str(&s)?;

            let response: IpcResponse = match request {
                IpcRequest::GetStatus => {
                    let state = watch_rx_clone.borrow();
                    IpcResponse::CurrentState(state.current_state)
                }
                IpcRequest::GetTodaySchedule => {
                    let schedule = watch_rx_clone.borrow().daily_schedule.clone();
                    IpcResponse::DailySchedule(schedule)
                }
                IpcRequest::ReloadConfig => {
                    match mpsc_tx_clone.send(IpcRequest::ReloadConfig).await {
                        Ok(_) => IpcResponse::Success,
                        Err(e) => IpcResponse::Error(e.to_string()),
                    }
                }
            };

            let response_string = serde_json::to_string(&response)?;
            write_half.write_all(response_string.as_bytes()).await?;
            write_half.flush().await?;

            anyhow::Ok(())
        });
    }
}

async fn daemon_loop(
    mut config: Config,
    watch_tx: watch::Sender<DaemonSnapShot>,
    mut mpsc_rx: mpsc::Receiver<IpcRequest>,
) -> anyhow::Result<()> {
    let mut prayer_manager = PrayerManager::new(&config);
    let mut state = DaemonState::Calculating;

    loop {
        state = match state {
            DaemonState::Calculating => {
                let event = prayer_manager.get_next_actionable_event(Utc::now());

                let next_event = match event {
                    ActionableEvent::WaitForPrayer(name, time) => {
                        DaemonState::WaitingForPrayer(name, time)
                    }
                    ActionableEvent::WaitForIqamah(name, time) => {
                        DaemonState::WaitingForIqamah(name, time)
                    }
                    ActionableEvent::Skip => panic!("Shouldn't happen"),
                };

                watch_tx.send(DaemonSnapShot::new(
                    next_event,
                    &mut prayer_manager,
                    config.iqamah_offset,
                ))?;

                next_event
            }
            DaemonState::WaitingForPrayer(prayer, time) => {
                println!("Next prayer is {prayer:?} at {time}",);
                println!("Sleeping until prayer");

                tokio::select! {
                    biased;
                    Some(IpcRequest::ReloadConfig) = mpsc_rx.recv() => {
                        println!("Reloading config...");
                        (prayer_manager, config) = System::reload();
                        DaemonState::Calculating
                    },
                    _ = System::sleep_until_datetime(time) => {
                        println!("Woke up for prayer");

                        System::notify(
                            &format!("Prayer {prayer} has started"),
                            &format!(
                                "Iqamah in {} minutes",
                                prayer_manager
                                    .time_left_for_iqamah(prayer, time)
                                    .unwrap()
                                    .num_minutes()
                            ),
                        )
                        .await?;

                        let iqamah_time = prayer_manager.get_iqamah_time(prayer, time).unwrap();
                        let next_event = DaemonState::WaitingForIqamah(prayer, iqamah_time);

                        watch_tx.send(DaemonSnapShot::new(
                            next_event,
                            &mut prayer_manager,
                            config.iqamah_offset,
                        ))?;

                        next_event
                    },
                }
            }
            DaemonState::WaitingForIqamah(prayer, time) => {
                let five_min_before_iqamah = time - Duration::from_secs(5 * 60);
                let two_min_before_iqamah = time - Duration::from_secs(2 * 60);
                let one_half_min_before_iqamah = time - Duration::from_secs(15 * 6);

                // TODO: Add config reloading in this section
                System::sleep_until_datetime(five_min_before_iqamah).await;

                System::notify(
                    &format!("{prayer} Iqamah in 5 minutes"),
                    "Get ready! Lockdown in 3 minutes!",
                )
                .await
                .unwrap();

                System::sleep_until_datetime(two_min_before_iqamah).await;

                System::notify(
                    &format!("{prayer} Iqamah in 2 minutes"),
                    "Get ready! Lockdown in 30 seconds!",
                )
                .await
                .unwrap();

                System::sleep_until_datetime(one_half_min_before_iqamah).await;
                println!("Initiating lockdown");

                let next_event =
                    DaemonState::Lockdown(time + Duration::from_secs(LOCKDOWN_DURATION_SECONDS));

                watch_tx
                    .send(DaemonSnapShot {
                        current_state: next_event,
                        daily_schedule: prayer_manager.get_schedule(Local::now()),
                        offsets: config.iqamah_offset,
                    })
                    .unwrap();

                next_event
            }
            DaemonState::Lockdown(unlock_time) => {
                while Utc::now() < unlock_time {
                    if config.clone().options.lock_screen {
                        System::lock_screen().await.unwrap();
                    } else {
                        System::notify(
                            "Iqamah has started!!",
                            "Leave your PC and go pray already!",
                        )
                        .await
                        .unwrap();
                    }

                    tokio::time::sleep(Duration::from_secs(LOCKDOWN_POLL_SECONDS)).await;
                }

                println!("Lockdown finished");

                let next_event = DaemonState::Calculating;

                watch_tx.send(DaemonSnapShot::new(
                    next_event,
                    &mut prayer_manager,
                    config.iqamah_offset,
                ))?;

                next_event
            }
        }
    }
}
