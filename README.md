<img src="./title.png" align="center">
<br>
<br>
x360 is a command-line tool for working with .x360 files an open, ultra-compact audio format designed for size-constrained projects like game jams (e.g., JS13K).
Instead of storing audio samples, x360 stores musical notes and timing, allowing for tiny file sizes while maintaining musical expressiveness.


### Features
**Ultra Compact:** Stores notes + timing instead of audio samples (kilobytes instead of megabytes)

**Open Format:** Fully documented, free to use in any project

**CSV Support:** Encode/decode from human-readable CSV files

**Variant Support:** Multiple variations within a single file

**CLI Tool:** Play, encode, and inspect x360 files from the terminal

**Written in Rust:** Fast, safe, and cross-platform

### Installation

#### Arch Linux (AUR)

```bash
yay -S x360
```

#### Build Manually

```bash
git clone https://github.com/WilkerSebastian/libX360.git

cd x360

cargo build --release

sudo cp target/release/x360 /usr/local/bin/
```

### Usage


#### Play an x360 File

```bash
x360 music.x360
```

#### Encode CSV to x360
```bash
x360 music.csv -o music.x360
```

#### Adjust Volume (0-100)
```bash
x360 music.x360 --volume 75
```

#### Verbose Mode (Show Logs)
```
x360 music.x360 --verbose
```

### Full Command Reference

| Command | Description |
| :--- | :---: |
| x360 <file.x360> | Play an x360 audio file |
| x360 <file.csv> -o <file.x360> | Encode CSV to x360 format |
| --volume <0-100> | Set playback volume
| --verbose | Show detailed processing logs |
| --version, -v | Show version number |
| --help, -h | Show help message |

### File Format

x360 Binary Structure

| Offset | Size | Field | Description |
| :--- | --- | --- | --- |
| 0 | 1 byte | Version | Format version number |
| 1 | 4 bytes | Signature | ASCII: X360 |
| 5 | N × 3 bytes | Note Pairs | [note: u8][time: u16 BE] repeated |

### CSV Format

For encoding, use a simple CSV with two columns:

```csv
note,time
69,500
72,250
65,500
255,100
```

| Column | Type | Description |
| :--- | --- | ---: |
| note | 0–255 | MIDI-like note number (255 = silence/rest) |
| time | integer | Duration in milliseconds |


### Note Reference

Notes follow standard musical frequencies. See the full table:

<a href="https://iazzetta.eca.usp.br/tutor/acustica/introducao/tabela1.html"> USP Acoustics Frequency Table </a>


### Why x360?

The x360 format was created to solve a specific problem: how to include music in 13KB games?
Traditional audio formats (WAV, MP3, OGG) are too large for size-limited competitions like JS13K. By storing notes and timing instead of samples.

obs: The name x360 means extreme 2π.