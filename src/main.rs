use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::{Address, Network, PrivateKey};
use bitcoin::key::{TapTweak, UntweakedPublicKey};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crossbeam::channel;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "taproot-vanity")]
#[command(about = "Ultra-fast Taproot vanity address generator")]
struct Args {
    /// Desired prefix (after bc1p)
    #[arg(short, long)]
    prefix: Option<String>,
    
    /// Desired suffix
    #[arg(short, long)]
    suffix: Option<String>,
    
    /// Number of worker threads (default: CPU cores)
    #[arg(short, long)]
    workers: Option<usize>,
    
    /// Case sensitive matching
    #[arg(long)]
    case_sensitive: bool,
}

struct VanityResult {
    address: String,
    private_key: String,
    attempts: u64,
    worker_id: usize,
}

fn main() {
    let args = Args::parse();
    
    if args.prefix.is_none() && args.suffix.is_none() {
        eprintln!("❌ You must specify at least a prefix or suffix!");
        std::process::exit(1);
    }
    
    let prefix = args.prefix.unwrap_or_default();
    let suffix = args.suffix.unwrap_or_default();
    let workers = args.workers.unwrap_or_else(num_cpus::get);
    
    println!("🎯 ULTRA-FAST TAPROOT VANITY GENERATOR");
    println!("=====================================");
    println!("Prefix: '{}'", prefix);
    println!("Suffix: '{}'", suffix);
    println!("Workers: {}", workers);
    println!("Case sensitive: {}", args.case_sensitive);
    println!();
    
    let difficulty = estimate_difficulty(&prefix, &suffix);
    println!("Estimated difficulty: 1 in {}", format_number(difficulty));
    println!("Expected attempts: ~{}", format_number(difficulty / 2));
    println!();
    
    let found = Arc::new(AtomicBool::new(false));
    let total_attempts = Arc::new(AtomicU64::new(0));
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
    let handles: Vec<_> = (0..workers)
        .map(|worker_id| {
            let prefix = prefix.clone();
            let suffix = suffix.clone();
            let found = Arc::clone(&found);
            let total_attempts = Arc::clone(&total_attempts);
            let tx = tx.clone();
            let case_sensitive = args.case_sensitive;
            
            std::thread::spawn(move || {
                worker_thread(worker_id, prefix, suffix, case_sensitive, found, total_attempts, tx)
            })
        })
        .collect();
    
    // Progress monitoring
    let pb_clone = pb.clone();
    let total_attempts_clone = Arc::clone(&total_attempts);
    let found_clone = Arc::clone(&found);
    let progress_handle = std::thread::spawn(move || {
        while !found_clone.load(Ordering::Relaxed) {
            let attempts = total_attempts_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 { attempts as f64 / elapsed } else { 0.0 };

            pb_clone.set_message(format!(
                "Attempts: {} | Rate: {:.0}/s | Elapsed: {:.1}s",
                format_number(attempts), rate, elapsed
            ));

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    // Wait for result
    if let Ok(result) = rx.recv() {
        found.store(true, Ordering::Relaxed);
        pb.finish_with_message("Found!");
        
        let elapsed = start_time.elapsed();
        let total_attempts = total_attempts.load(Ordering::Relaxed);
        
        println!("\n🎉 SUCCESS! Found matching address!");
        println!("   Address: {}", result.address);
        println!("   Private Key: {}", result.private_key);
        println!("   Total Attempts: {}", format_number(total_attempts));
        println!("   Time: {:.2}s", elapsed.as_secs_f64());
        println!("   Rate: {:.0}/s", total_attempts as f64 / elapsed.as_secs_f64());
        println!("   Found by Worker: #{}", result.worker_id);
        
        // Save result
        save_result(&result, &prefix, &suffix, total_attempts, elapsed);
    }
    
    // Clean up
    for handle in handles {
        let _ = handle.join();
    }
    let _ = progress_handle.join();
}

fn worker_thread(
    worker_id: usize,
    prefix: String,
    suffix: String,
    case_sensitive: bool,
    found: Arc<AtomicBool>,
    total_attempts: Arc<AtomicU64>,
    tx: channel::Sender<VanityResult>,
) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let mut local_attempts = 0u64;
    
    while !found.load(Ordering::Relaxed) {
        // Generate random private key
        let secret_key = SecretKey::new(&mut rng);
        let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
        
        // Generate Taproot address
        if let Ok(address) = generate_taproot_address(&secp, &secret_key) {
            local_attempts += 1;
            
            // Update global counter every 1000 attempts for performance
            if local_attempts % 1000 == 0 {
                total_attempts.fetch_add(1000, Ordering::Relaxed);
            }
            
            // Check if it matches our pattern
            if matches_pattern(&address, &prefix, &suffix, case_sensitive) {
                let result = VanityResult {
                    address,
                    private_key: private_key.to_wif(),
                    attempts: local_attempts,
                    worker_id,
                };
                
                let _ = tx.send(result);
                return;
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

fn matches_pattern(address: &str, prefix: &str, suffix: &str, case_sensitive: bool) -> bool {
    if !address.starts_with("bc1p") {
        return false;
    }
    
    let addr_body = &address[4..]; // Remove "bc1p"
    
    if case_sensitive {
        addr_body.starts_with(prefix) && addr_body.ends_with(suffix)
    } else {
        addr_body.to_lowercase().starts_with(&prefix.to_lowercase()) &&
        addr_body.to_lowercase().ends_with(&suffix.to_lowercase())
    }
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

fn estimate_difficulty(prefix: &str, suffix: &str) -> u64 {
    let charset_size = 32u64; // bech32 charset
    let mut difficulty = 1u64;
    
    if !prefix.is_empty() {
        difficulty = difficulty.saturating_mul(charset_size.saturating_pow(prefix.len() as u32));
    }
    if !suffix.is_empty() {
        difficulty = difficulty.saturating_mul(charset_size.saturating_pow(suffix.len() as u32));
    }
    
    difficulty
}

fn save_result(result: &VanityResult, prefix: &str, suffix: &str, total_attempts: u64, elapsed: Duration) {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("taproot_vanity_results.txt")
        .unwrap();
    
    writeln!(file, "🎯 TAPROOT VANITY ADDRESS FOUND! 🎯").unwrap();
    writeln!(file, "Timestamp: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")).unwrap();
    writeln!(file, "Address: {}", result.address).unwrap();
    writeln!(file, "Private Key: {}", result.private_key).unwrap();
    writeln!(file, "Prefix: '{}'", prefix).unwrap();
    writeln!(file, "Suffix: '{}'", suffix).unwrap();
    writeln!(file, "Total Attempts: {}", format_number(total_attempts)).unwrap();
    writeln!(file, "Time: {:.2}s", elapsed.as_secs_f64()).unwrap();
    writeln!(file, "Rate: {:.0}/s", total_attempts as f64 / elapsed.as_secs_f64()).unwrap();
    writeln!(file, "Found by Worker: #{}", result.worker_id).unwrap();
    writeln!(file, "{}", "=".repeat(70)).unwrap();
    writeln!(file).unwrap();
}
