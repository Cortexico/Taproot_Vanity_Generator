# Taproot KEK Hunter

Ultra-fast Taproot vanity address generator specifically designed to find Bitcoin addresses containing multiple "kek" patterns.

## Features

- 🚀 **Ultra-fast multi-threaded generation** using all CPU cores
- 🎯 **Finds addresses with multiple "kek" occurrences** (minimum 2 by default)
- 📝 **Continuous logging** of found addresses to file
- ⚡ **Optimized for speed** with batch processing and efficient string matching
- 🛑 **Graceful shutdown** with Ctrl+C handling
- 📊 **Real-time progress monitoring** with attempts/second and found count

## What Changed

This is a modified version of the original Taproot Vanity Generator with the following key changes:

1. **Target Pattern**: Instead of prefix/suffix matching, it now searches for multiple "kek" occurrences anywhere in the address
2. **Continuous Operation**: Runs indefinitely until stopped, logging each qualifying address
3. **Multiple Results**: Finds and logs ALL addresses with the required kek count (not just the first one)
4. **Optimized Performance**: Increased batch sizes and more efficient string searching
5. **Better Logging**: Immediate file writing with flush for real-time results

## Usage

### Basic Usage (Find addresses with 2+ "kek" patterns)
```bash
./taproot-vanity.exe
```

### Find addresses with 3+ "kek" patterns
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

- `-m, --min-kek-count <COUNT>`: Minimum number of 'kek' occurrences required (default: 2)
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
Address: bc1pkekabckekdefkekghijklmnopqrstuvwxyz
Private Key: L1234567890abcdef...
KEK Count: 3
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

The difficulty increases exponentially with the number of required "kek" patterns:
- 2 keks: ~1 in 32,768 addresses
- 3 keks: ~1 in 1,048,576 addresses  
- 4 keks: ~1 in 33,554,432 addresses

## Security Note

⚠️ **Important**: The generated private keys control real Bitcoin addresses. Keep them secure and never share them publicly. The private keys are saved in the output file - protect this file appropriately.
