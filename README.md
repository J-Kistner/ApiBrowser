# FRC Finger Lakes Regional Browser

A terminal-based user interface (TUI) application for browsing FRC Finger Lakes Regional data from The Blue Alliance API.

## Features

- 📊 View team rankings sorted by rank
- 🤖 Browse detailed team statistics (auto, teleop, endgame averages)
- 📅 See all matches (completed and scheduled) for each team
- 🎯 View match details with score breakdowns
- 🔄 Navigate between teams from match screens
- 💾 Response caching for fast loading
- 🧪 Test mode for viewing historical data

## Prerequisites

- The Blue Alliance API key ([Get one here](https://www.thebluealliance.com/account))

## Installation

### Quick Install (Recommended)

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/J-Kistner/ApiBrowser/main/install.sh | bash
```

Then add to your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Install via Cargo

If you have Rust installed:
```bash
cargo install apibrowser
```

### Download Pre-built Binary

Download the latest release for your platform from the [releases page](https://github.com/J-Kistner/ApiBrowser/releases):
- **Linux**: `apibrowser-linux-x86_64`
- **macOS (Intel)**: `apibrowser-macos-x86_64`
- **macOS (Apple Silicon)**: `apibrowser-macos-aarch64`
- **Windows**: `apibrowser-windows-x86_64.exe`

Make it executable (Linux/macOS):
```bash
chmod +x apibrowser-*
sudo mv apibrowser-* /usr/local/bin/apibrowser
```

### Build from Source

1. Clone this repository:
```bash
git clone https://github.com/J-Kistner/ApiBrowser
cd ApiBrowser
```

2. Build the project:
```bash
cargo build --release
```

3. Binary will be at `target/release/apibrowser`

## Configuration

You need a Blue Alliance API key. Get one at: https://www.thebluealliance.com/account

The application will look for your API key in the following order:

### Option 1: Config Directory (Recommended for installed binaries)
```bash
mkdir -p ~/.config/apibrowser
echo "TBA_API_KEY=your_key_here" > ~/.config/apibrowser/.env
```

This works from any directory you run the command from.

### Option 2: Current Directory (Good for development)
```bash
echo "TBA_API_KEY=your_key_here" > .env
```

This requires you to run the command from the same directory.

### Option 3: Environment Variable (Good for CI/Docker)
```bash
export TBA_API_KEY=your_key_here
apibrowser
```

## Usage

### Normal Mode (2026 Data)
```bash
cargo run --release
```

### Test Mode (2024 Historical Data)
```bash
cargo run --release -- --test-mode
# or
cargo run --release -- -t
```

### Custom Event
```bash
cargo run --release -- --event-key 2024nyro
```

## Navigation

### Team List Screen
- `↑/↓` or `j/k` - Navigate through teams
- `/` - Search for team by number
- `r` - Refresh data from API
- `Enter` - View team details
- `q` - Quit application

### Search Mode (Team List)
- Type digits to enter team number
- `Enter` - Jump to team and exit search
- `Backspace` - Delete last digit
- `Esc` - Cancel search

### Team Detail Screen
- `↑/↓` or `j/k` - Navigate through matches
- `r` - Refresh data from API
- `Enter` - View match details
- `Esc` or `Backspace` - Return to team list
- `q` - Quit application

### Match Detail Screen
- `↑/↓` or `j/k` - Navigate through teams in the match
- `r` - Refresh data from API
- `Enter` - View selected team's details
- `Esc` or `Backspace` - Return to previous screen
- `q` - Quit application

## Screen Overview

### 1. Team List
The main screen shows all teams:
- **With rankings**: Sorted by rank (ranked teams first, then unranked teams by number)
- **Without rankings**: Sorted by team number
- Rank (0 if unranked)
- Team Number
- Team Name
- Win-Loss-Tie Record
- RP Average (Ranking Points)
- Press `/` to search for a team by number
- Team Name
- Win-Loss-Tie Record
- RP Average (Ranking Points)

### 2. Team Detail
Shows comprehensive statistics for a selected team:
- **Season Averages**: Auto, Teleop, Endgame points, RP average
- **Match Record**: Wins, losses, ties
- **OPR Stats**: Offensive Power Rating
- **Match Schedule**: All matches (past and upcoming)
  - ✓ Completed matches show scores and results
  - ⏰ Scheduled matches show date/time

### 3. Match Detail
Shows detailed information about a specific match:
- Match name and time
- Blue Alliance teams and scores
- Red Alliance teams and scores
- Score breakdown (auto, teleop, endgame)
- Click on any team to view their details

## Data Caching

The app caches all API responses to improve performance and reduce API calls:
- Cache location: `~/.cache/apibrowser/{event_key}/`
- Uses ETags for conditional requests (304 Not Modified)
- Respects TBA's Cache-Control headers

To clear the cache, simply delete the cache directory.

## API Rate Limiting

The Blue Alliance has rate limits on their API. The app uses caching to minimize API calls. If you encounter rate limit errors:
- Wait a few minutes before retrying
- Use cached data (automatic on subsequent runs)
- Avoid running multiple instances simultaneously

## Troubleshooting

### "TBA API requires authentication for all endpoints"
Make sure you've created a `.env` file with your API key:
```
TBA_API_KEY=your_key_here
```

Get your API key from: https://www.thebluealliance.com/account

### "Failed to fetch event data"
- Check that the event key is correct (e.g., `2026nyro`)
- Verify your API key is valid
- Ensure you have internet connectivity
- The event may not exist yet or may have a different key

### "No matches played yet"
If viewing an event that hasn't started, team statistics won't be available. Scheduled matches will still be shown.

## Event Keys

The app defaults to the Finger Lakes Regional:
- 2026: `2026nyro` (current year)
- 2024: `2024nyro` (test mode)

To find event keys for other events, visit [The Blue Alliance](https://www.thebluealliance.com).

## Development

### Project Structure
```
src/
├── main.rs           # Entry point and TUI event loop
├── config.rs         # Configuration and CLI args
├── cache.rs          # Response caching
├── stats.rs          # Team statistics calculations
├── api/
│   ├── client.rs     # HTTP client
│   ├── models.rs     # Data structures
│   └── endpoints.rs  # API endpoints
└── ui/
    ├── app.rs        # App state and navigation
    ├── team_list.rs  # Team list view
    ├── team_detail.rs# Team detail view
    └── match_detail.rs# Match detail view
```

### Building from Source
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

## License

[Add your license here]

## Credits

- Data provided by [The Blue Alliance](https://www.thebluealliance.com)
- Built with [Ratatui](https://github.com/ratatui-org/ratatui)
