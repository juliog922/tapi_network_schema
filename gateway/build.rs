use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Define the path to the configuration file
    let config_path = Path::new("config/config.yaml");

    // Read the contents of the configuration file into memory
    let config_content = fs::read_to_string(config_path).expect("Failed to read config.yaml");

    // Generate a SHA-256 hash of the file content
    let mut hasher = Sha256::new();
    hasher.update(&config_content);
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);

    // Prepare the output path for the generated Rust file
    let dest_path = Path::new("src/config_hash.rs");

    // Write the hash as a Rust constant to the destination file
    let mut f = fs::File::create(dest_path).expect("Failed to create hash output file");
    write!(f, "pub const CONFIG_HASH: &str = \"{}\";", hash_hex)
        .expect("Failed to write hash to output file");

    // Instruct Cargo to rerun this build script if the config file changes
    println!("cargo:rerun-if-changed=config.yaml");
}
