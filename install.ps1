# ==========================================
# 1. Setup Variables
# ==========================================
$Repo = "kofta999/pryr"
$ZipUrl = "https://github.com/$Repo/releases/latest/download/pryr-x86_64-pc-windows-msvc.zip"
$InstallDir = "$env:LOCALAPPDATA\pryr\bin"
$ConfigDir = "$env:APPDATA\pryr"

Write-Host "Installing pryr for Windows..." -ForegroundColor Cyan

# ==========================================
# 2. Download & Extract Binaries
# ==========================================
Write-Host "Downloading latest release..." -ForegroundColor Yellow
$TempZip = "$env:TEMP\pryr.zip"
Invoke-WebRequest -Uri $ZipUrl -OutFile $TempZip

Write-Host "Extracting binaries to $InstallDir..." -ForegroundColor Yellow
if (!(Test-Path $InstallDir)) { New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null }
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force

Remove-Item $TempZip

# ==========================================
# 3. Add to PATH
# ==========================================
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    Write-Host "Adding $InstallDir to user PATH..." -ForegroundColor Yellow
    $NewPath = $UserPath + ";$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    $env:Path = $env:Path + ";$InstallDir" # Update current session
}

# ==========================================
# 4. Create Scheduled Task (The Daemon)
# ==========================================
Write-Host "Setting up silent background task..." -ForegroundColor Yellow
$TaskName = "PryrDaemon"

# Unregister old task if it exists
if (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue) {
    Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
}

# Define the task to run silently at logon
$Action = New-ScheduledTaskAction -Execute "$InstallDir\pryrd.exe"
$Trigger = New-ScheduledTaskTrigger -AtLogOn
# Settings: Don't kill it if on laptop battery, run hidden
$Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -Hidden -ExecutionTimeLimit 0

Register-ScheduledTask -TaskName $TaskName -Action $Action -Trigger $Trigger -Settings $Settings | Out-Null

# ==========================================
# 5. Start the Daemon
# ==========================================
Write-Host "Starting the pryrd daemon..." -ForegroundColor Yellow
Start-ScheduledTask -TaskName $TaskName

Write-Host "✨ pryr successfully installed and running!" -ForegroundColor Green
Write-Host "Please restart your terminal to use the 'pryr' command." -ForegroundColor Cyan
