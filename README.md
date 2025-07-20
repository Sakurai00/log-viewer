# log-viewer

`log-viewer` is a command-line utility written in Rust for viewing, filtering, and highlighting log files in real-time or as a one-off output. It's designed to provide a flexible way to monitor and analyze log data directly from your terminal.

## Features

*   **Real-time Log Monitoring:** Tail log files and display new entries as they arrive.
*   **File Concatenation (Cat Mode):** Display the entire content of specified log files.
*   **Keyword Filtering:** Include or exclude lines based on specified keywords.
*   **Preset Exclusions:** Option to disable default exclusion rules for common log patterns.
*   **Syntax Highlighting:** Highlight critical, informational, and warning messages for better readability.
*   **Debug Information:** Display internal debug information for troubleshooting.

## Installation

To build and install `log-viewer`, you need to have Rust and Cargo installed. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/Sakurai00/log-viewer.git
cd log-viewer
cargo build --release
cargo install --path .
```

## Usage

`log-viewer` can be used to either watch log files in real-time or to display their content once (like `tail -f,  cat`).

```bash
log-viewer [OPTIONS] [LOG_FILES...]
```

### Arguments

*   `<LOG_FILES...>`: One or more paths to log files to process. If not specified, it defaults to `/var/log/messages`.

### Options

*   `-l`, `--log-files <LOG_FILES...>`: Specify one or more log files to process.
*   `-d`, `--disable-preset-excludes`: Disable preset exclusion rules. By default, `log-viewer` might exclude certain common log patterns. Use this flag to show all lines.
*   `-e`, `--exclude-words <EXCLUDE_WORDS...>`: Exclude lines containing any of the specified words.
*   `-i`, `--include-words <INCLUDE_WORDS...>`: Include only lines containing any of the specified words.
*   `--debug`: Enable debug mode, which prints additional information about the application's internal state.
*   `--cat`: Display the content of the log files once and exit, similar to the `cat` command. By default, `log-viewer` watches files for new content.

## Examples

1.  **Watch `/var/log/messages` in real-time (default behavior):**
    ```bash
    log-viewer
    ```

2.  **Watch a specific log file:**
    ```bash
    log-viewer -l /path/to/your/app.log
    ```

3.  **Display content of multiple log files once (cat mode):**
    ```bash
    log-viewer --cat -l /var/log/syslog /var/log/auth.log
    ```

4.  **Watch a log file, excluding lines with "error" or "fail":**
    ```bash
    log-viewer -e error fail
    ```

5.  **Watch a log file, including only lines with "success" or "completed":**
    ```bash
    log-viewer -i success completed
    ```

6.  **Combine include and exclude filters:**
    ```bash
    log-viewer -i "request" -e "healthcheck"
    ```

7.  **Disable preset exclusions and watch a log file:**
    ```bash
    log-viewer -d -l /var/log/kern.log
    ```

8.  **Enable debug mode:**
    ```bash
    log-viewer --debug -l /var/log/messages
    ```
