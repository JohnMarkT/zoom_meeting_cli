# Zoom Meeting CLI

This is a simple CLI application written in Rust to join Zoom meetings. It allows you to list, add, remove, and join Zoom meetings using predefined meeting names and IDs.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/zoom_meeting_cli.git
    cd zoom_meeting_cli
    ```

2. Build the project (If you don't have Rust installed, skip to step 4):
    ```sh
    cargo build --release
    ```

3. Rename the binary (optional):
    ```sh
    mv target/release/zoom_meeting_cli dist/release/zoom
    ```

4. Copy the binary to a directory in your `PATH`:
    ```sh
    sudo cp dist/release/zoom /usr/local/bin/
    sudo cp meetings.json /usr/local/bin/
    ```

    If you don't have write permissions for `/usr/local/bin`, you can use a directory like `~/.local/bin`:
    ```sh
    cp dist/release/zoom ~/.local/bin/
    ```

    Make sure `~/.local/bin` is in your `PATH`. You can add it to your `PATH` by adding the following line to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`):
    ```sh
    export PATH="$HOME/.local/bin:$PATH"
    ```

    Also, add the following to your shell configuration file:
    ```sh
    export ZOOMCLI_MEETINGS_FILE=/usr/local/bin/meetings.json
    ```

    Then, reload your shell configuration:
    ```sh
    source ~/.bashrc  # or source ~/.zshrc
    ```

5. Verify the installation:
    ```sh
    zoom help
    ```

## Usage

### Commands

- `ls`: List all available meetings.
    ```sh
    zoom ls
    ```

- `join <name>`: Join the specified meeting. You can also use `zoom <name>` directly to join a meeting.
    ```sh
    zoom join standup
    zoom standup
    ```

- `join <id>`: Join the specified meeting by ID. You can also use `zoom <id>` directly to join a meeting.
    ```sh
    zoom join 123 456 7890
    zoom 123 456 7890
    zoom join 1234567890
    zoom 1234567890
    ```

- `add <name> <id>`: Add a new meeting with the specified name and ID. The ID must contain only digits and be 9, 10, or 11 digits long after removing any non-digit characters.
    ```sh
    zoom add standup 123 456 7890
    ```

- `rm <name>`: Remove the specified meeting.
    ```sh
    zoom rm standup
    ```

- `help`: Display the help message.
    ```sh
    zoom help
    ```

### Example

1. Add a new meeting:
    ```sh
    zoom add standup 123 456 7890
    ```

2. List all available meetings:
    ```sh
    zoom ls
    ```

3. Join a meeting:
    ```sh
    zoom join standup
    ```

4. Remove a meeting:
    ```sh
    zoom rm standup
    ```

### Configuration

The meetings are stored in a `meetings.json` file. By default, the application looks for `meetings.json` in the current directory. You can specify a different path using the `ZOOMCLI_MEETINGS_FILE` environment variable:

```sh
export ZOOMCLI_MEETINGS_FILE=/path/to/your/meetings.json
```

Add this line to your shell configuration file (e.g., ~/.bashrc, ~/.zshrc) to make it persistent.

The `meetings.json` file should have the following format:

```json
{
    "meetings": {}
}
```

### License
This project is licensed under the MIT License.
