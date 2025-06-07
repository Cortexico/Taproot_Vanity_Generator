# Taproot Vanity Generator

Ultra-fast Taproot vanity address generator for Bitcoin. Generate custom Bitcoin Taproot addresses with specific prefixes and/or suffixes.

## Features

- 🚀 **Ultra-fast multi-threaded generation** using all CPU cores
- 🎯 **Prefix and suffix matching** for custom vanity addresses
- ⚡ **Optimized for speed** with efficient batch processing
- 📊 **Real-time progress monitoring** with attempts/second display
- 💾 **Automatic result saving** to file with detailed statistics
- 🔧 **Configurable worker threads** for optimal performance
- 📝 **Case-sensitive or case-insensitive** matching options

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (install from https://rustup.rs/)

### Quick Install & Run

**On Windows (PowerShell/CMD):**
```cmd
# Build the application
build.bat

# Run it
target\release\taproot-vanity.exe --prefix abc
```

**On Linux/macOS (Bash):**
```bash
# Build the application
./build.sh

# Run it
./target/release/taproot-vanity --prefix abc
```

The build scripts will automatically check for Rust installation and build the optimized release version.

## Usage

### Basic Examples

**Find address with specific prefix:**
```bash
./target/release/taproot-vanity --prefix abc
```

**Find address with specific suffix:**
```bash
./target/release/taproot-vanity --suffix xyz
```

**Find address with both prefix and suffix:**
```bash
./target/release/taproot-vanity --prefix abc --suffix xyz
```

**Use specific number of worker threads:**
```bash
./target/release/taproot-vanity --prefix abc --workers 8
```

**Case-sensitive matching:**
```bash
./target/release/taproot-vanity --prefix ABC --case-sensitive
```

## Command Line Options

- `-p, --prefix <PREFIX>`: Desired prefix (after bc1p)
- `-s, --suffix <SUFFIX>`: Desired suffix
- `-w, --workers <WORKERS>`: Number of worker threads (default: CPU cores)
- `--case-sensitive`: Enable case-sensitive matching

**Note:** You must specify at least a prefix or suffix (or both).

## Output

### Console Output
The program displays real-time progress including:
- Total attempts made
- Generation rate (addresses/second)
- Elapsed time
- Success notification with full details

### File Output
Results are automatically saved to `taproot_vanity_results.txt` with:
- Timestamp
- Bitcoin address
- Private key (WIF format)
- Search parameters (prefix/suffix)
- Performance statistics
- Worker thread information

Example output:
```
🎯 TAPROOT VANITY ADDRESS FOUND! 🎯
Timestamp: 2024-01-15 10:30:45 UTC
Address: bc1pabcdefghijklmnopqrstuvwxyz123456789
Private Key: L1234567890abcdef...
Prefix: 'abc'
Suffix: 'xyz'
Total Attempts: 1,234,567
Time: 45.67s
Rate: 27,045/s
Found by Worker: #3
======================================================================
```

## Performance

- **Multi-threaded**: Uses all available CPU cores by default
- **Optimized**: Batch processing reduces atomic operations overhead
- **Fast**: Typically processes 20,000-100,000+ addresses per second on modern hardware
- **Scalable**: Performance scales linearly with CPU cores

## Difficulty Estimation

The program estimates difficulty before starting:
- **Prefix only**: 32^(prefix_length) attempts
- **Suffix only**: 32^(suffix_length) attempts  
- **Both**: 32^(prefix_length + suffix_length) attempts

Examples:
- 3-character prefix: ~1 in 32,768 addresses
- 4-character prefix: ~1 in 1,048,576 addresses
- 3-char prefix + 3-char suffix: ~1 in 1,073,741,824 addresses

## Technical Details

### Address Format
- Generates Bitcoin Taproot (P2TR) addresses
- All addresses start with `bc1p`
- Uses bech32m encoding with 32-character charset
- Prefix/suffix matching applies to the part after `bc1p`

### Security
- Uses cryptographically secure random number generation
- Private keys are generated using secp256k1 curve
- All generated addresses are valid and spendable

## Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd Taproot_Vanity_Generator

# Build release version
cargo build --release

# Run
./target/release/taproot-vanity --prefix abc
```

## Security Warning

⚠️ **Important**: The generated private keys control real Bitcoin addresses. Keep them secure and never share them publicly. The private keys are saved in the output file - protect this file appropriately.

## License

This project is open source. Please use responsibly.
