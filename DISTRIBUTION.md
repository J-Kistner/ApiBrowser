# Distribution Setup Guide

## Repository Setup

Your git repository is initialized and ready! Here's how to push it to GitHub:

### 1. Create GitHub Repository

1. Go to https://github.com/new
2. Create a new repository named `ApiBrowser` (or your preferred name)
3. **Do NOT** initialize with README, .gitignore, or license (we already have these)
4. Copy the repository URL (e.g., `git@github.com:J-kistner/ApiBrowser.git`)

### 2. Push to GitHub

```bash
# Add GitHub as remote
git remote add origin git@github.com:J-kistner/ApiBrowser.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Enable GitHub Actions

GitHub Actions are already configured in `.github/workflows/release.yml`. They will automatically run when you create a release tag.

### 4. Create Your First Release

```bash
# Tag the current commit
git tag -a v0.1.0 -m "Initial release"

# Push the tag to GitHub
git push origin v0.1.0
```

This will automatically:
- Build binaries for Linux (x86_64, musl)
- Build binaries for macOS (Intel, Apple Silicon)
- Build binaries for Windows (x86_64)
- Create a GitHub release with all binaries attached

### 5. Update Documentation

After creating the GitHub repository, update these files with your actual username:

- `README.md` - Replace `J-kistner` with your GitHub username
- `install.sh` - Replace `J-kistner` with your GitHub username
- `.github/workflows/release.yml` - Should work as-is
- `Cargo.toml` - Update the repository URL

You can do a find-and-replace:
```bash
# Replace J-kistner with your actual username
sed -i 's/J-kistner/YOUR_GITHUB_USERNAME/g' README.md install.sh homebrew-formula.rb

# Update Cargo.toml repository field manually
```

## Distribution Methods

Once your release is created, users can install via:

### 1. Quick Install Script (Easiest)
```bash
curl -sSL https://raw.githubusercontent.com/J-kistner/ApiBrowser/main/install.sh | bash
```

### 2. Direct Download
From the [Releases page](https://github.com/J-kistner/ApiBrowser/releases)

### 3. Cargo Install (Rust users)
After publishing to crates.io:
```bash
cargo install apibrowser
```

### 4. Docker
```bash
docker pull ghcr.io/J-kistner/apibrowser:latest
docker run -it --rm -e TBA_API_KEY=your_key ghcr.io/J-kistner/apibrowser
```

## Publishing to crates.io (Optional)

1. Create an account at https://crates.io
2. Get your API token from https://crates.io/me
3. Login:
   ```bash
   cargo login
   ```
4. Publish:
   ```bash
   cargo publish
   ```

## Updating the Release Workflow

The GitHub Actions workflow will run on every tag push that starts with `v`.

To create a new release:
```bash
# Update version in Cargo.toml
# Update CHANGELOG.md

git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to v0.2.0"
git push

git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

## Binaries Location

After the workflow runs, binaries will be available at:
```
https://github.com/J-kistner/ApiBrowser/releases/download/v0.1.0/apibrowser-linux-x86_64
https://github.com/J-kistner/ApiBrowser/releases/download/v0.1.0/apibrowser-macos-x86_64
https://github.com/J-kistner/ApiBrowser/releases/download/v0.1.0/apibrowser-macos-aarch64
https://github.com/J-kistner/ApiBrowser/releases/download/v0.1.0/apibrowser-windows-x86_64.exe
```

## Security Note

The `.env` file containing your API key is already in `.gitignore` and will NOT be committed to the repository. Users need to provide their own API key.

## Next Steps

1. Push to GitHub following the steps above
2. Create your first release tag
3. Wait for GitHub Actions to build (takes ~5-10 minutes)
4. Share the installation link with users!

## Support

Users can report issues at: https://github.com/J-kistner/ApiBrowser/issues
