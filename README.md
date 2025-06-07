# Taproot KEK Hunter (bc1pkek Prefix)

Ultra-fast Taproot vanity address generator specifically designed to find Bitcoin addresses that start with "bc1pkek" and contain additional "kek" patterns.

## Features

- 🚀 **Ultra-fast multi-threaded generation** using all CPU cores
- 🎯 **Finds bc1pkek addresses** with configurable total "kek" count (minimum 1 by default)
- 📝 **Continuous logging** of found addresses to file
- ⚡ **Optimized for speed** with batch processing and efficient string matching
- 🛑 **Graceful shutdown** with Ctrl+C handling
- 📊 **Real-time progress monitoring** with attempts/second and found count

## What This Does

This generator specifically targets Bitcoin Taproot addresses that:

1. **Start with "bc1pkek"** - All addresses must have this exact prefix
2. **Count total "kek" occurrences** - Counts the prefix "kek" plus any additional "kek" patterns in the address
3. **Continuous Operation** - Runs indefinitely until stopped, logging each qualifying address
4. **Multiple Results** - Finds and logs ALL addresses with the required kek count (not just the first one)
5. **Optimized Performance** - Increased batch sizes and more efficient string searching
6. **Better Logging** - Immediate file writing with flush for real-time results

### Examples:
- `bc1pkek...` = 1 kek count (just the prefix)
- `bc1pkekkek...` = 2 kek count (prefix + one additional)
- `bc1pkekabckek...` = 2 kek count (prefix + one additional)
- `bc1pkekkekkek...` = 3 kek count (prefix + two additional)

## Usage

### Basic Usage (Find bc1pkek addresses with 1+ total "kek" patterns)
```bash
./taproot-vanity.exe
```

### Find bc1pkek addresses with 2+ total "kek" patterns
```bash
./taproot-vanity.exe --min-kek-count 2
```

### Find bc1pkek addresses with 3+ total "kek" patterns
```bash
./taproot-vanity.exe --min-kek-count 3
```

### Use specific number of worker threads
```bash
./taproot-vanity.exe --workers 8
```

### Custom output file
```bash
./taproot-vanity.exe --output-file my_kek_addresses.txt
```

### Case-sensitive matching
```bash
./taproot-vanity.exe --case-sensitive
```

## Command Line Options

- `-m, --min-kek-count <COUNT>`: Minimum number of total 'kek' occurrences required (default: 1, includes prefix)
- `-w, --workers <WORKERS>`: Number of worker threads (default: CPU cores)
- `-o, --output-file <FILE>`: Output file for results (default: kek_addresses.txt)
- `--case-sensitive`: Enable case-sensitive matching

## Output Format

Each found address is logged to the output file with:
- Timestamp
- Bitcoin address
- Private key (WIF format)
- Number of "kek" occurrences
- Worker thread that found it

Example output:
```
🎯 KEK ADDRESS #1 FOUND! 🎯
Timestamp: 2024-01-15 10:30:45 UTC
Address: bc1pkekabckekdefghijklmnopqrstuvwxyz
Private Key: L1234567890abcdef...
KEK Count: 2
Found by Worker: #2
======================================================================
```

## Performance

- **Multi-threaded**: Uses all available CPU cores by default
- **Optimized**: Batch processing reduces atomic operations overhead
- **Fast**: Typically processes 100,000+ addresses per second on modern hardware

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (install from https://rustup.rs/)

### Quick Install & Run

**On Windows (PowerShell/CMD):**
```cmd
# Build the application
build.bat

# Run it
target\release\taproot-vanity.exe
```

**On Linux/macOS (Bash):**
```bash
# Build the application
./build.sh

# Run it
./target/release/taproot-vanity
```

The build scripts will automatically check for Rust installation and build the optimized release version.

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/taproot-vanity.exe
```

## Stopping

Press `Ctrl+C` to gracefully stop the search. All found addresses will be saved to the output file.

## Difficulty Estimation

The difficulty for bc1pkek addresses increases with additional "kek" patterns:
- 1 kek (bc1pkek only): ~1 in 32,768 addresses
- 2 keks (bc1pkek + 1 more): ~1 in 1,073,741,824 addresses
- 3 keks (bc1pkek + 2 more): ~1 in 34,359,738,368 addresses

Note: The base difficulty is finding "bc1pkek" (~32K attempts), then each additional "kek" multiplies the difficulty significantly.

## Security Note

⚠️ **Important**: The generated private keys control real Bitcoin addresses. Keep them secure and never share them publicly. The private keys are saved in the output file - protect this file appropriately.
