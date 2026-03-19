# pryr 🕌

> A blazing-fast, uncompromising Islamic prayer time daemon and screen locker for Linux and Windows.

Notifications are too easy to dismiss when you're deep in a coding session. **`pryr` doesn't just remind you to pray; it forces you.** Built entirely in Rust, `pryr` runs silently in the background. It calculates daily prayer times, sends escalating desktop warnings, and when Iqamah hits, it ruthlessly locks your session for a configurable duration. You cannot dismiss it. You can only go pray.

---

### ✨ Features

- **Uncompromising Lockdown:** Stubbornly locks your screen during Iqamah. If you unlock it early, it instantly locks it again until the duration is over.
- **Escalating Warnings:** Native desktop notifications before lockdown at configurable intervals (default: 5 minutes and 2 minutes before Iqamah).
- **Configurable Lockdown Boundaries:** Fine-tune exactly when warnings fire, when the lockdown begins, and how long after Iqamah the session stays locked.
- **Zero-Overhead IPC:** A lightning-fast CLI (`pryr`) communicates with the background daemon (`pryrd`) via Unix Domain Sockets (Linux) or Named Pipes (Windows).
- **Dynamic Configuration:** Update your location and calculation methods on the fly using the CLI, or adjust Iqamah offsets via a simple TOML file. Hot-reloads without dropping the daemon.
- **Native Screen Locking:** Uses `loginctl` on Linux (Wayland/X11) and the native `LockWorkStation` API on Windows to cleanly and forcefully lock your device.
- **Self-Updating:** `pryr` can update itself to the latest version with a single command.

---

### 🚀 Installation

`pryr` is distributed as pre-compiled binaries for Linux and Windows (x86_64). You don't need Rust installed to run it.

#### Linux

Run the following command to download the latest release, add it to your `$PATH`, and automatically set up the systemd background service:

```bash
curl -fsSL https://raw.githubusercontent.com/kofta999/pryr/master/install.sh | bash

```

_(Note: Requires `systemd` and `loginctl` to be present on your system)._

#### Windows

Open PowerShell and run the following command to download the release, add it to your `PATH`, and register the silent logon background task via the Windows Task Scheduler:

```powershell
irm https://raw.githubusercontent.com/kofta999/pryr/master/install.ps1 | iex

```

---

### 💻 Usage

The `pryr` CLI acts as a remote control for the background daemon.

```bash
# Set your location, calculation method, and Madhab (automatically fetches coordinates)
pryr configure --city "Suez, Egypt" --method egyptian --madhab shafi

# View the full schedule for today (Adhan and Iqamah times)
pryr schedule

# Get the live countdown to the next prayer or lockdown event
pryr status

# Output schedule or status as raw JSON (useful for scripting and integrations)
pryr schedule --json
pryr status --json

# Reload the configuration file dynamically without restarting the daemon
pryr reload-config

# Update pryr to the latest version
pryr update
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

Pass `--json` to get machine-readable output instead:

```bash
pryr schedule --json
```

```json
{"DailySchedule":{"fajr":{"prayer_time":"2026-03-10T04:45:00Z","iqamah_time":"2026-03-10T05:05:00Z"},"dhuhr":{"prayer_time":"2026-03-10T12:05:00Z","iqamah_time":"2026-03-10T12:20:00Z"},"asr":{"prayer_time":"2026-03-10T15:32:00Z","iqamah_time":"2026-03-10T15:47:00Z"},"maghrib":{"prayer_time":"2026-03-10T18:01:00Z","iqamah_time":"2026-03-10T18:11:00Z"},"isha":{"prayer_time":"2026-03-10T19:20:00Z","iqamah_time":"2026-03-10T19:35:00Z"}}}
```

---

### ⚙️ Configuration

On first run, `pryr` automatically generates a default configuration file based on your operating system:

- **Linux:** `~/.config/pryr/config.toml`
- **Windows:** `%APPDATA%\pryr\config.toml` _(usually `C:\Users\YourName\AppData\Roaming\pryr\config.toml`)_

The easiest way to update your location is using the `pryr configure` command, which automatically queries a geocoding API to find your latitude and longitude and updates the daemon instantly.

For fine-grained control over your Iqamah delays or screen-locking behavior, you can manually edit the `config.toml` file:

```toml
[location]
lat = 29.9668
long = 32.5498

[prayer-config]
method = "Egyptian"
madhab = "Shafi"

[jumuah]
# Minutes BEFORE Dhuhr on Fridays to send the "Get ready for Jumu'ah" warning (default: 45)
early-warning = 45
# Minutes to lock the screen after Dhuhr starts for the Khutbah and prayer (default: 45)
lockdown-duration = 45

[iqamah-offset]
fajr = 20
dhuhr = 15
asr = 15
maghrib = 10
isha = 15

[options]
# Set to false to only receive notifications without locking the screen
lock-screen = true

[lockdown]
# Minutes before Iqamah to show the first warning notification (default: 5)
warning-before-iqamah = 5
# Minutes before Iqamah to initiate the screen lock (default: 2)
lock-before-iqamah = 2
# Minutes after Iqamah before the screen is unlocked (default: 10)
unlock-after-iqamah = 10

```

After manually modifying the file, simply run `pryr reload-config` to instantly apply the new math.

---

### 🏗️ Architecture

`pryr` is built with a decoupled Client-Server architecture to ensure maximum performance and zero missed events:

1. **`pryrd` (The Heart):** A Tokio-powered asynchronous state machine. It handles time math, schedules system sleeps, executes lockdowns, and broadcasts state changes via an `mpsc` watch channel. It runs invisibly via `systemd` (Linux) or Task Scheduler (Windows).
2. **The Nerves:** `pryrd` binds to `/run/user/$UID/pryr.sock` (Linux) or `\\.\pipe\pryr-ipc` (Windows). It listens for Newline-Delimited JSON requests and responds instantly using zero-cost cached data.
3. **`pryr` (The Face):** A lightweight, synchronous CLI built with `clap`. It connects to the socket/pipe, sends remote procedure calls, formats the JSON response with `owo-colors`, and exits in milliseconds. (Network requests, like geocoding a city, are strictly isolated to the CLI to keep the daemon purely offline).

---

### 📝 License & Author

Developed by **Mostafa Mahmoud**.

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.
