# pryr 🕌

> A blazing-fast, uncompromising Islamic prayer time daemon and screen locker for Linux.

Notifications are too easy to dismiss when you're deep in a coding session. **`pryr` doesn't just remind you to pray; it forces you.** Built entirely in Rust, `pryr` runs silently in the background as a systemd daemon. It calculates daily prayer times, sends escalating desktop warnings, and when Iqamah hits, it ruthlessly locks your Wayland/X11 session using `loginctl` for a configurable duration. You cannot dismiss it. You can only go pray.

---

### ✨ Features

- **Uncompromising Lockdown:** Stubbornly locks your screen during Iqamah. If you unlock it early, it instantly locks it again until the duration is over.
- **Escalating Warnings:** Native desktop notifications at T-minus 5 minutes and 2 minutes before lockdown.
- **Zero-Overhead IPC:** A lightning-fast CLI (`pryr`) communicates with the background daemon (`pryrd`) via Unix Domain Sockets.
- **Dynamic Configuration:** Adjust prayer calculation methods, Madhab, and Iqamah offsets via a simple TOML file. Hot-reload the config without dropping the daemon.
- **Wayland Native:** Uses `loginctl` to lock the session, making it perfectly compatible with modern Wayland compositors (Hyprland, Sway) and X11.

---

### 🚀 Installation

`pryr` is distributed as a pre-compiled binary for Linux (x86_64). You don't need Rust installed to run it.

Run the following command to download the latest release, add it to your `$PATH`, and automatically set up the systemd background service:

```bash
curl -fsSL https://raw.githubusercontent.com/kofta999/pryr/master/install.sh | bash

```

_(Note: Requires `systemd` and `loginctl` to be present on your system)._

---

### 💻 Usage

The `pryr` CLI acts as a remote control for the background daemon.

```bash
# View the full schedule for today (Adhan and Iqamah times)
pryr schedule

# Get the live countdown to the next prayer or lockdown event
pryr status

# Reload the configuration file dynamically without restarting the daemon
pryr reload-config

```

#### Example Output:

```text
┌─ Today's Prayer Schedule
    Prayer        Adhan         Iqamah
  ─────────────────────────────────────
  ✓ Fajr         04:45 AM       05:05 AM
  ✓ Dhuhr        12:05 PM       12:20 PM
  ○ Asr          03:32 PM       03:47 PM
  ○ Maghrib      06:01 PM       06:11 PM
  ○ Isha         07:20 PM       07:35 PM
└───────────────────────────────────────

```

---

### ⚙️ Configuration

On first run, `pryr` automatically generates a default configuration file at `~/.config/pryr/config.toml`.

By default, it uses the Egyptian General Authority of Survey method, but you can customize everything from your exact coordinates to the lockdown duration.

```toml
[location]
lat = 29.9668
long = 32.5498

[prayer-config]
method = "Egyptian"
madhab = "Shafi"

[iqamah-offset]
fajr = 20
dhuhr = 15
asr = 15
maghrib = 10
isha = 15

[options]
# Set to false to only receive notifications without locking the screen
lock-screen = true

```

After modifying the file, simply run `pryr reload-config` to instantly apply the new math.

---

### 🏗️ Architecture

`pryr` is built with a decoupled Client-Server architecture to ensure maximum performance and zero missed events:

1. **`pryrd` (The Heart):** A Tokio-powered asynchronous state machine. It handles time math, schedules system sleeps, executes `loginctl` lockdowns, and broadcasts state changes via an `mpsc` watch channel.
2. **Unix Domain Sockets (The Nerves):** `pryrd` binds to `/run/user/$UID/pryr.sock`. It listens for Newline-Delimited JSON requests and responds instantly using zero-cost cached data.
3. **`pryr` (The Face):** A lightweight, synchronous CLI built with `clap`. It connects to the socket, sends remote procedure calls, formats the JSON response with `owo-colors`, and exits in milliseconds.

---

### 📝 License & Author

Developed by **Mostafa Mahmoud**.

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
