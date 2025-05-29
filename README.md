# Peeksy

Earlier called as Screenshot-Auto

A Rust-based tool that automatically renames screenshots and images using OpenAI's GPT-4 Vision API. The tool analyzes image content and generates descriptive, meaningful filenames following a consistent naming convention.

Demo video [here](https://x.com/anubhavitis/status/1922303569639702976)

## Features

- Automatic image analysis using GPT-4 Vision API
- Intelligent context recognition (screenshots, album covers, artwork, posters, UI elements)
- Consistent filename formatting (lowercase, hyphen-separated)
- Real-time file monitoring and automatic renaming
- Configurable naming rules through prompt template

## Requirements

- Rust (latest stable version)
- OpenAI API key with access to GPT-4 Vision API
- (Optional) A `.env` file with the following environment variables:
  - `PEEKSY_OPENAI_API_KEY`: Your OpenAI API key
  - `PEEKSY_OPENAI_PROMPT_FILE`: Path to your prompt template file (default: `prompt.txt`)

Note: If the `.env` file or required variables are missing, Peeksy will prompt you to enter them during first run. The values will be automatically saved for future use.


## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/anubhavitis/peeksy.git
   cd peeksy
   ```

2. Create a `.env` file in the project root:
   ```bash
   PEEKSY_OPENAI_API_KEY=your_api_key_here
   PEEKSY_OPENAI_PROMPT_FILE=prompt.txt
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Run the compiled binary:
   ```bash
   ./target/release/peeksy
   ```

2. The program will monitor your screenshots directory and automatically rename new images based on their content.

## Filename Format

Output filename is expected to have ```-ss``` as suffix

## Customization

You can customize the naming rules by modifying the `prompt.txt` file. The file contains instructions for the AI model on how to generate appropriate filenames.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 
