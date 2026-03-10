#!/usr/bin/env bash
set -e

# ==========================================
# 1. Setup Colors & Variables
# ==========================================
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REPO="kofta999/pryr"
TARBALL_URL="https://github.com/$REPO/releases/latest/download/pryr-x86_64-unknown-linux-gnu.tar.gz"
BIN_DIR="$HOME/.local/bin"
SYSTEMD_DIR="$HOME/.config/systemd/user"

echo -e "${BLUE}Installing pryr...${NC}"

# ==========================================
# 2. Check Architecture & OS
# ==========================================
if [ "$(uname -s)" != "Linux" ]; then
    echo -e "Error: pryr currently only supports Linux."
    exit 1
fi

if [ "$(uname -m)" != "x86_64" ]; then
    echo -e "Error: pryr install script currently only supports x86_64 architecture."
    exit 1
fi

# ==========================================
# 3. Download & Extract Binaries
# ==========================================
echo -e "${YELLOW}Downloading latest release...${NC}"
TMP_DIR=$(mktemp -d)
curl -fsSL "$TARBALL_URL" -o "$TMP_DIR/pryr.tar.gz"

echo -e "${YELLOW}Extracting binaries to $BIN_DIR...${NC}"
mkdir -p "$BIN_DIR"
tar -xzf "$TMP_DIR/pryr.tar.gz" -C "$TMP_DIR"

# Move binaries and make them executable
mv "$TMP_DIR/pryr-linux-x86_64/pryr" "$BIN_DIR/"
mv "$TMP_DIR/pryr-linux-x86_64/pryrd" "$BIN_DIR/"
chmod +x "$BIN_DIR/pryr" "$BIN_DIR/pryrd"

# Clean up temp folder
rm -rf "$TMP_DIR"

# ==========================================
# 4. Add to PATH (if necessary)
# ==========================================
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $BIN_DIR to PATH in ~/.bashrc and ~/.zshrc...${NC}"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    if [ -f "$HOME/.zshrc" ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
    fi
    export PATH="$HOME/.local/bin:$PATH"
fi

# ==========================================
# 5. Create Systemd User Service
# ==========================================
echo -e "${YELLOW}Setting up systemd background service...${NC}"
mkdir -p "$SYSTEMD_DIR"

cat << EOF > "$SYSTEMD_DIR/pryrd.service"
[Unit]
Description=Pryr Prayer Time & Lockdown Daemon
After=graphical-session.target

[Service]
ExecStart=$BIN_DIR/pryrd
Restart=always
RestartSec=3
Environment="PATH=$BIN_DIR:/usr/bin:/bin"
Environment="DISPLAY=:0"
Environment="WAYLAND_DISPLAY=wayland-1"

[Install]
WantedBy=default.target
EOF

# ==========================================
# 6. Enable and Start the Daemon
# ==========================================
echo -e "${YELLOW}Starting the pryrd daemon...${NC}"
systemctl --user daemon-reload
systemctl --user enable --now pryrd.service

echo -e "${GREEN}✨ pryr successfully installed and running!${NC}"
echo -e "Please run ${BLUE}pryr status${NC} to check the daemon (you may need to restart your terminal first)."
