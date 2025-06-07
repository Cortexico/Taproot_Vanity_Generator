use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::{Address, Network, PrivateKey};
use bitcoin::key::{TapTweak, UntweakedPublicKey};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crossbeam::channel;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[command(name = "taproot-vanity")]
#[command(about = "Ultra-fast Taproot vanity address generator for 'bc1pkek' prefix with additional 'kek' patterns")]
struct Args {
    /// Minimum number of 'kek' occurrences required (default: 1, includes the prefix kek)
    #[arg(short, long, default_value = "1")]
    min_kek_count: usize,

    /// Number of worker threads (default: CPU cores)
    #[arg(short, long)]
    workers: Option<usize>,

    /// Output file for results (default: kek_addresses.txt)
    #[arg(short, long, default_value = "kek_addresses.txt")]
    output_file: String,

    /// Case sensitive matching
    #[arg(long)]
    case_sensitive: bool,
}

struct VanityResult {
    address: String,
    private_key: String,
    kek_count: usize,
    worker_id: usize,
}

fn main() {
    let args = Args::parse();

    if args.min_kek_count < 1 {
        eprintln!("❌ Minimum kek count must be at least 1!");
        std::process::exit(1);
    }

    let workers = args.workers.unwrap_or_else(num_cpus::get);

    println!("🎯 ULTRA-FAST TAPROOT KEK HUNTER (bc1pkek PREFIX)");
    println!("=================================================");
    println!("Target: bc1pkek addresses with {} or more total 'kek' occurrences", args.min_kek_count);
    println!("Workers: {}", workers);
    println!("Case sensitive: {}", args.case_sensitive);
    println!("Output file: {}", args.output_file);
    println!();

    let difficulty = estimate_kek_difficulty(args.min_kek_count);
    println!("Estimated difficulty: 1 in {}", format_number(difficulty));
    println!("Expected attempts per find: ~{}", format_number(difficulty / 2));
    println!("Press Ctrl+C to stop the search");
    println!();
    
    let total_attempts = Arc::new(AtomicU64::new(0));
    let total_found = Arc::new(AtomicU64::new(0));
    let (tx, rx) = channel::unbounded();

    let start_time = Instant::now();

    // Progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap()
    );

    // Spawn worker threads
    let _handles: Vec<_> = (0..workers)
        .map(|worker_id| {
            let total_attempts = Arc::clone(&total_attempts);
            let tx = tx.clone();
            let min_kek_count = args.min_kek_count;
            let case_sensitive = args.case_sensitive;

            std::thread::spawn(move || {
                worker_thread(worker_id, min_kek_count, case_sensitive, total_attempts, tx)
            })
        })
        .collect();
    
    // Progress monitoring
    let pb_clone = pb.clone();
    let total_attempts_clone = Arc::clone(&total_attempts);
    let total_found_clone = Arc::clone(&total_found);
    let _progress_handle = std::thread::spawn(move || {
        loop {
            let attempts = total_attempts_clone.load(Ordering::Relaxed);
            let found_count = total_found_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 { attempts as f64 / elapsed } else { 0.0 };

            pb_clone.set_message(format!(
                "Attempts: {} | Rate: {:.0}/s | Found: {} | Elapsed: {:.1}s",
                format_number(attempts), rate, found_count, elapsed
            ));

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    // Handle results continuously
    let output_file = args.output_file.clone();
    let _result_handle = std::thread::spawn(move || {
        while let Ok(result) = rx.recv() {
            total_found.fetch_add(1, Ordering::Relaxed);
            let found_count = total_found.load(Ordering::Relaxed);

            println!("\n🎉 KEK ADDRESS #{} FOUND!", found_count);
            println!("   Address: {}", result.address);
            println!("   Private Key: {}", result.private_key);
            println!("   KEK Count: {}", result.kek_count);
            println!("   Found by Worker: #{}", result.worker_id);

            // Save result immediately
            save_kek_result(&result, &output_file, found_count);
        }
    });

    // Keep running until Ctrl+C
    println!("🔍 Searching for bc1pkek addresses with {} or more total 'kek' patterns...", args.min_kek_count);
    println!("Press Ctrl+C to stop");

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    ctrlc::set_handler(move || {
        println!("\n🛑 Received Ctrl+C, stopping search...");
        running_clone.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl+C handler");

    // Wait for Ctrl+C or result handler to finish
    while running.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Search stopped. Check {} for results.", args.output_file);
}

fn worker_thread(
    worker_id: usize,
    min_kek_count: usize,
    case_sensitive: bool,
    total_attempts: Arc<AtomicU64>,
    tx: channel::Sender<VanityResult>,
) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let mut local_attempts = 0u64;

    loop {
        // Generate random private key
        let secret_key = SecretKey::new(&mut rng);
        let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

        // Generate Taproot address
        if let Ok(address) = generate_taproot_address(&secp, &secret_key) {
            local_attempts += 1;

            // Update global counter every 5000 attempts for better performance
            if local_attempts % 5000 == 0 {
                total_attempts.fetch_add(5000, Ordering::Relaxed);
            }

            // Check if it starts with bc1pkek and has enough total 'kek' occurrences
            if let Some(kek_count) = count_bc1pkek_occurrences(&address, case_sensitive) {
                if kek_count >= min_kek_count {
                    let result = VanityResult {
                        address,
                        private_key: private_key.to_wif(),
                        kek_count,
                        worker_id,
                    };

                    let _ = tx.send(result);
                    // Continue searching for more addresses
                }
            }
        }
    }
}

fn generate_taproot_address(secp: &Secp256k1<bitcoin::secp256k1::All>, secret_key: &SecretKey) -> Result<String, Box<dyn std::error::Error>> {
    let public_key = PublicKey::from_secret_key(secp, secret_key);
    let untweaked_pubkey = UntweakedPublicKey::from(public_key);
    let (_tweaked_pubkey, _parity) = untweaked_pubkey.tap_tweak(secp, None);

    let address = Address::p2tr(secp, untweaked_pubkey, None, Network::Bitcoin);
    Ok(address.to_string())
}

fn count_bc1pkek_occurrences(address: &str, case_sensitive: bool) -> Option<usize> {
    // Must start with "bc1pkek"
    if !address.starts_with("bc1pkek") {
        return None;
    }

    // Count total "kek" occurrences in the entire address (including the prefix)
    let search_text = if case_sensitive {
        address.to_string()
    } else {
        address.to_lowercase()
    };

    let pattern = if case_sensitive { "kek" } else { "kek" };

    // Count non-overlapping occurrences of "kek"
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = search_text[start..].find(pattern) {
        count += 1;
        start += pos + pattern.len();
    }

    Some(count)
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    result
}

fn estimate_kek_difficulty(min_kek_count: usize) -> u64 {
    let charset_size = 32u64; // bech32 charset

    // Base difficulty for bc1pkek prefix (3 characters after bc1p)
    let base_difficulty = charset_size.pow(3); // ~32,768 for "kek"

    // Additional difficulty for extra keks beyond the first one
    if min_kek_count <= 1 {
        base_difficulty
    } else {
        let extra_keks = min_kek_count - 1;
        let extra_difficulty = charset_size.pow(3 * extra_keks as u32);
        base_difficulty * extra_difficulty
    }
}

fn save_kek_result(result: &VanityResult, output_file: &str, found_count: u64) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .unwrap();

    writeln!(file, "🎯 KEK ADDRESS #{} FOUND! 🎯", found_count).unwrap();
    writeln!(file, "Timestamp: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")).unwrap();
    writeln!(file, "Address: {}", result.address).unwrap();
    writeln!(file, "Private Key: {}", result.private_key).unwrap();
    writeln!(file, "KEK Count: {}", result.kek_count).unwrap();
    writeln!(file, "Found by Worker: #{}", result.worker_id).unwrap();
    writeln!(file, "{}", "=".repeat(70)).unwrap();
    writeln!(file).unwrap();

    // Flush to ensure immediate write
    let _ = file.flush();
}
