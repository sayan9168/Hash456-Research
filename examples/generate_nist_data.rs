use hash456_research::Hash456;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nist_input.bin").unwrap();
    let mut counter = 0u64;
    let mut total_bytes = 0;
    let target_bytes = 1_000_000; // 1 MB data for STS

    while total_bytes < target_bytes {
        let data = counter.to_le_bytes();
        let hash = Hash456::hash(&data);
        file.write_all(&hash).unwrap();
        total_bytes += hash.len();
        counter += 1;
    }
    println!("Generated {} bytes for NIST STS", total_bytes);
}
