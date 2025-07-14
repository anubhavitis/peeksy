

> ⚠️ **DEPRECATION NOTICE**: This CLI version of Peeksy is now deprecated. All future development has moved to the new GUI application: **[peeksy-app](https://github.com/anubhavitis/peeksy-app)**. 
> 
> The new GUI version offers a much better user experience with visual interface, drag-and-drop functionality, and enhanced features. Please migrate to [peeksy-app](https://github.com/anubhavitis/peeksy-app) for continued updates and support.


<img src="assets/peeksy.png" width="150" height="150" alt="Peeksy Logo" />
# Peeksy


A Rust-based tool that automatically renames screenshots and images using OpenAI's GPT-4 Vision API. The tool analyzes image content and generates descriptive, meaningful filenames following a consistent naming convention.

[View Demo](https://x.com/anubhavitis/status/1922303569639702976)

## Features

- **Automatic Image Analysis**: Uses GPT-4 Vision API for intelligent content recognition
- **Universal Image Renaming**: Rename any image file, not just screenshots
- **Bulk Rename Existing Screenshots**: Retroactively rename all your existing screenshots with AI-powered intelligent naming
- **Auto-Start on Boot**: Automatically starts when your machine restarts via LaunchD integration
- **Raycast Integration**: Native Raycast app for lightning-fast screenshot management
- **Intelligent Context Recognition**: Detects screenshots, album covers, artwork, posters, UI elements
- **Consistent Filename Formatting**: Lowercase, hyphen-separated naming convention
- **Real-time File Monitoring**: Automatic renaming as you take screenshots
- **Configurable Naming Rules**: Customizable prompt templates for naming preferences


## Requirements

- OpenAI API key with access to GPT-4 Vision API

Note: Peeksy will prompt you to enter OpenApi key them during first run. The values will be automatically saved for future use.


## 📦 Installation & Upgrade

v2.0 launched recently, [read here](https://github.com/anubhavitis/peeksy/releases/tag/v2.0)

⚠️ For existing users, first you'll have to remove the current installation, and get the fresh installation
```bash
brew remove peeksy
brew untap anubhavitis/peeksy
brew tap anubhavitis/peeksy
brew install peeksy
```

For new installations:

```bash
brew tap anubhavitis/peeksy
brew install peeksy
```



### Examples
```bash
# Start the Peeksy daemon (auto-starts on boot after setup)
peeksy start

# Check if the daemon is running and get its PID
peeksy status

# Stop the running daemon
peeksy stop

# Restart the daemon (useful after configuration changes)
peeksy restart

# Rename any image file using AI
peeksy rename-image "/path/to/your/image.jpg"

# Bulk rename all existing screenshots in your screenshots folder
peeksy rename-existing-screenshots

# View your current configuration settings
peeksy current-config

# View the contents of your prompt template file
peeksy view-prompt-file

# Update your OpenAI API key
peeksy update-api-key "sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# Update the path to your prompt template file
peeksy update-prompt-file-path "/path/to/your/custom-prompt.txt"

# View logs for troubleshooting
peeksy logs

# Run the daemon directly (usually not needed, use 'start' instead)
peeksy daemon
```

Note: The daemon must be running for Peeksy to monitor and rename your screenshots. Use `peeksy status` to verify the daemon is running.

## How to Use

Once you've completed the installation steps above, Peeksy will automatically monitor your screenshots directory. Here's what happens:

1. **Start the Service**
   ```bash
   peeksy start
   ```

2. **Take Screenshots**
   - Use your system's screenshot shortcut (⌘⇧3 or ⌘⇧4 on macOS)

3. **Watch the Magic**
   - Peeksy will detect new screenshots
   - Analyze the content using GPT-4 Vision
   - Automatically rename them with descriptive names
   - Original files are preserved with the new names

4. **Customize (Optional)**
   ```bash
   # View your current prompt template
   peeksy view-prompt-file

   # Update the prompt template
   peeksy update-prompt-file-path "path/to/your/custom-prompt.txt"
   ```

That's it! Peeksy will continue running in the background, automatically renaming your screenshots as you take them.

## Commands

Peeksy provides several commands to manage the daemon and configuration:

### Daemon Management
- `start` - Start the Peeksy daemon
- `stop` - Stop the running Peeksy daemon
- `restart` - Restart the Peeksy daemon
- `status` - Check if the Peeksy daemon is running and get its PID

### Configuration Management
- `current-config` - Display the current configuration in JSON format
- `view-prompt-file` - Display the contents of the current prompt file
- `update-api-key <value>` - Update the OpenAI API key
- `update-prompt-file-path <value>` - Update the path to the prompt template file

## Building in your machine

1. Clone the repository:
   ```bash
   git clone https://github.com/anubhavitis/peeksy.git
   cd peeksy
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   sudo cp target/release/peeksy /usr/local/bin/
   ```

4. Start the daemon:
   ```bash
   peeksy start
   ```

5. Verify the installation:
   ```bash
   # Check if the daemon is running
   peeksy status

   # View your current configuration
   peeksy current-config
   ```

Note: Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

# Support & Queries

> **Note**: This CLI version is deprecated. For new issues and feature requests, please use the new GUI version: **[peeksy-app](https://github.com/anubhavitis/peeksy-app)**

For legacy CLI support or migration help:
- [Email](mailto:ss.lfsgd@gmail.com)
- GitHub Issues: [Create an issue](https://github.com/anubhavitis/peeksy/issues) (CLI-related only)

For the new GUI version (recommended):
- **New Repository**: [peeksy-app](https://github.com/anubhavitis/peeksy-app)
- **Issues & Features**: [Create an issue in peeksy-app](https://github.com/anubhavitis/peeksy-app/issues)

We recommend migrating to [peeksy-app](https://github.com/anubhavitis/peeksy-app) for:
- Better user experience with GUI interface
- Active development and new features
- Enhanced functionality and performance
- Continued support and updates
   
# Special Thanks

A heartfelt thank you to:

[navedux](https://naved.xyz) for creating the beautiful Peeksy logo and identity