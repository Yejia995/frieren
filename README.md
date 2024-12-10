# Frieren

Frieren is minimal program that automates the renaming and relocation in qBittorrent based on `config.toml`. It is recommended to use it with the *"Run external program on torrent finished"* option in qBittorrent.

## Features

- Flexible renaming of torrent files using patterns.
- Relocation of files to specified directories.
- Automatic tagging for organized torrent management.
- Configurable via a `config.toml` file.

## Example Configuration

Here's an example `config.toml` file for setting up the tool:

```toml
server = "http://127.0.0.1:8080"
username = "foo"
password = "bar"
category = "RSS"
add_tags = ["Moved", "Shows"]

[[rule]]

[rule.source]
spec = "[VCB-Studio] Sword Art Online [#][Ma10p_1080p]*.mkv"

[rule.target]
path = "/downloads/Storage/Bangumi/Sword Art Online (2012)/Season 1"
spec = "Sword Art Online S01E#.mp4"
```

Given the example config:
- A completed torrent with the file `[VCB-Studio] Sword Art Online [12][Ma10p_1080p].mkv` will:
  - Be renamed to `Sword Art Online S01E12.mp4`.
  - Be moved to `/downloads/Storage/Bangumi/Sword Art Online (2012)/Season 1`.
  - Be tagged with `Moved` and `Shows`.

### Configuration Details

- **server**: The URL for the qBittorrent WebUI (e.g., `http://127.0.0.1:8080`).
- **username** and **password**: Credentials for qBittorrent WebUI access.
- **category** *(optional)*: Only process torrents from this category.
- **add_tags** *(optional)*: Tags to add to processed torrents.

#### Rules

- **source.spec**: A pattern matching the source filenames. 
  - `#` matches an episode number.
  - `*` matches any characters.
- **target.path**: The target directory for the renamed file.
- **target.spec**: The naming format for the renamed file, with `#` representing the episode number.

## Usage

1. Clone the repository and build the binary.
2. Place the binary in the desired directory.
3. Create a `config.toml` file in the same directory as the binary.
4. Run the binary. It will automatically process completed torrents based on the specified rules.

## Dependencies

- [qbit_rs](https://crates.io/crates/qbit_rs) for interacting with qBittorrent.
- [regex](https://crates.io/crates/regex) for pattern matching.
- [serde](https://crates.io/crates/serde) and [toml](https://crates.io/crates/toml) for configuration parsing.

## Contribution

Please open issues or submit pull requests for improvements and bug fixes. Thank you!
