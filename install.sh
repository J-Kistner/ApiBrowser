#!/bin/bash
# ApiBrowser Installation Script

set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    case "$ARCH" in
      x86_64) PLATFORM="linux-x86_64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
      x86_64) PLATFORM="macos-x86_64" ;;
      arm64) PLATFORM="macos-aarch64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Get latest release version
LATEST_VERSION=$(curl -s https://api.github.com/repos/J-Kistner/ApiBrowser/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
  echo "Failed to fetch latest version"
  exit 1
fi

echo "Installing ApiBrowser $LATEST_VERSION for $PLATFORM..."

# Download binary
DOWNLOAD_URL="https://github.com/J-Kistner/ApiBrowser/releases/download/${LATEST_VERSION}/apibrowser-${PLATFORM}"
INSTALL_DIR="${HOME}/.local/bin"
INSTALL_PATH="${INSTALL_DIR}/apibrowser"

mkdir -p "$INSTALL_DIR"
curl -L "$DOWNLOAD_URL" -o "$INSTALL_PATH"
chmod +x "$INSTALL_PATH"

echo "✓ ApiBrowser installed to $INSTALL_PATH"
echo ""

# Setup config directory
CONFIG_DIR="${HOME}/.config/apibrowser"
CONFIG_FILE="${CONFIG_DIR}/.env"

mkdir -p "$CONFIG_DIR"

# Check if API key already exists
if [ -f "$CONFIG_FILE" ]; then
  echo "✓ Configuration already exists at $CONFIG_FILE"
else
  echo "Setting up configuration..."
  echo ""
  echo "You need a Blue Alliance API key."
  echo "Get one at: https://www.thebluealliance.com/account"
  echo ""
  read -p "Enter your TBA API key (or press Enter to skip): " API_KEY
  
  if [ -n "$API_KEY" ]; then
    echo "TBA_API_KEY=$API_KEY" > "$CONFIG_FILE"
    echo "✓ Configuration saved to $CONFIG_FILE"
  else
    echo "Skipped configuration. You can set it up later with:"
    echo "  mkdir -p ~/.config/apibrowser"
    echo "  echo 'TBA_API_KEY=your_key_here' > ~/.config/apibrowser/.env"
  fi
fi

echo ""
echo "Make sure $INSTALL_DIR is in your PATH:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Run with: apibrowser"
