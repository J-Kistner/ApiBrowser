# .env File Location Guide

## Quick Answer

After installing the binary, create your config file here:
```bash
mkdir -p ~/.config/apibrowser
echo "TBA_API_KEY=your_key_here" > ~/.config/apibrowser/.env
```

Then you can run `apibrowser` from **any directory**.

## How It Works

The application looks for your API key in **3 locations**, in this order:

### 1. Current Directory (`.env`)
**Best for:** Development, building from source
```bash
cd /path/to/ApiBrowser
echo "TBA_API_KEY=your_key_here" > .env
cargo run
```

### 2. Config Directory (`~/.config/apibrowser/.env`)
**Best for:** Installed binaries, system-wide usage
```bash
mkdir -p ~/.config/apibrowser
echo "TBA_API_KEY=your_key_here" > ~/.config/apibrowser/.env
apibrowser  # Can run from any directory
```

### 3. Environment Variable (`TBA_API_KEY`)
**Best for:** CI/CD, Docker, temporary usage
```bash
export TBA_API_KEY=your_key_here
apibrowser
```

## Installation Scenarios

### Scenario 1: Installed via curl script
```bash
# Install
curl -sSL https://raw.githubusercontent.com/J-Kistner/ApiBrowser/main/install.sh | bash

# The script will prompt for your API key and create:
# ~/.config/apibrowser/.env

# Run from anywhere
cd ~
apibrowser
```

### Scenario 2: Downloaded binary
```bash
# Download and install binary
curl -L https://github.com/J-Kistner/ApiBrowser/releases/download/v0.1.0/apibrowser-linux-x86_64 -o ~/.local/bin/apibrowser
chmod +x ~/.local/bin/apibrowser

# Create config
mkdir -p ~/.config/apibrowser
echo "TBA_API_KEY=your_key_here" > ~/.config/apibrowser/.env

# Run from anywhere
apibrowser
```

### Scenario 3: Built from source
```bash
# Clone and build
git clone https://github.com/J-Kistner/ApiBrowser
cd ApiBrowser
cargo build --release

# Option A: Use local .env (only works from repo directory)
echo "TBA_API_KEY=your_key_here" > .env
cargo run

# Option B: Use config directory (works from any directory)
mkdir -p ~/.config/apibrowser
echo "TBA_API_KEY=your_key_here" > ~/.config/apibrowser/.env
./target/release/apibrowser
```

### Scenario 4: Docker
```bash
# Pass as environment variable
docker run -it --rm -e TBA_API_KEY=your_key_here apibrowser

# Or mount config directory
docker run -it --rm -v ~/.config/apibrowser:/root/.config/apibrowser apibrowser
```

## Priority Order

If you have API keys in multiple locations, the application uses the **first one found**:

1. Current directory `.env`
2. Config directory `~/.config/apibrowser/.env`
3. Environment variable `TBA_API_KEY`

## Security

- ✅ `.env` in the repo is in `.gitignore` (never committed)
- ✅ Config directory is in your home folder (user-specific)
- ✅ File permissions should be `600` (readable only by you)

To secure your config:
```bash
chmod 600 ~/.config/apibrowser/.env
```

## Troubleshooting

**"TBA API requires authentication for all endpoints"**
- Your API key is not being found
- Check that one of the 3 locations has your key
- Verify the key is valid at https://www.thebluealliance.com/account

**Binary works in repo but not elsewhere**
- You're using `.env` in the repo directory
- Create `~/.config/apibrowser/.env` instead
- Or use `export TBA_API_KEY=your_key`

**Want to use a different location?**
- Set the environment variable instead:
  ```bash
  export TBA_API_KEY=$(cat /your/custom/path/.env | grep TBA_API_KEY | cut -d= -f2)
  apibrowser
  ```
