/// File Integrity Check Tool (Simplified Version)
///
/// Uses SHA256 for byte-level detection to prevent file tampering
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};

/// Calculate SHA256 hash of file content
pub fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify if file content matches the given hash
pub fn verify_hash(content: &str, expected_hash: &str) -> bool {
    calculate_hash(content) == expected_hash
}

/// Baseline hashes - update these after generating new hashes
const BASELINE_HASHES: &[(&str, &str)] = &[
    (
        "tp_graph_test.rs",
        "a7536302360c4be32d56986b1dbd42117f6fbdbfd46099864623cef9e8ae0d36",
    ),
    (
        "isolation_level_test.rs",
        "d8ba568ee8fcaf32872d8d923a66790e09607d1f38b83f79be8d9aa399b3c535",
    ),
    (
        "mod.rs",
        "fb4a903ecda3e09858ce0764b967e81b01e3985d0f598b204403202e87ceedf0",
    ),
];

/// Verify integrity of specified file
pub fn verify_file(filename: &str, content: &str) -> Result<(), String> {
    if let Some((_, expected_hash)) = BASELINE_HASHES.iter().find(|(name, _)| *name == filename) {
        let current_hash = calculate_hash(content);
        if current_hash == *expected_hash {
            Ok(())
        } else {
            Err(format!(
                "File {} has been tampered!\nExpected hash: {}\nCurrent hash:  {}",
                filename,
                &expected_hash[..16],
                &current_hash[..16]
            ))
        }
    } else {
        Err(format!("Baseline hash not found for file {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = "fn test() { assert_eq!(1, 1); }";

    #[test]
    fn test_calculate_hash() {
        let hash = calculate_hash(TEST_CONTENT);
        assert_eq!(hash.len(), 64); // SHA256 = 64 hex chars

        // Same content should produce same hash
        let hash2 = calculate_hash(TEST_CONTENT);
        assert_eq!(hash, hash2);

        // Different content should produce different hash
        let hash3 = calculate_hash("different content");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_verify_hash() {
        let hash = calculate_hash(TEST_CONTENT);

        // Verify with same content
        assert!(verify_hash(TEST_CONTENT, &hash));

        // Verify with modified content (even one character change)
        let modified = "fn test() { assert_eq!(1, 2); }";
        assert!(!verify_hash(modified, &hash));
    }

    /// Generate baseline hashes - run this test to get hash values
    #[test]
    fn test_generate_baseline() {
        let files = vec![
            ("tp_graph_test.rs", "tests/tp/tp_graph_test.rs"),
            (
                "isolation_level_test.rs",
                "tests/tp/isolation_level_test.rs",
            ),
            ("mod.rs", "tests/tp/mod.rs"),
        ];

        println!("\n{}", "=".repeat(70));
        println!("Baseline Hashes (copy to BASELINE_HASHES constant):");
        println!("{}", "=".repeat(70));

        for (name, path) in files {
            let full_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
            if let Ok(content) = fs::read_to_string(&full_path) {
                let hash = calculate_hash(&content);
                println!("    (\"{}\", \"{}\"),", name, hash);
            }
        }

        println!("{}", "=".repeat(70));
    }

    /// Verify all test files
    #[test]
    fn test_verify_all_files() {
        let files = vec![
            ("tp_graph_test.rs", "tests/tp/tp_graph_test.rs"),
            (
                "isolation_level_test.rs",
                "tests/tp/isolation_level_test.rs",
            ),
            ("mod.rs", "tests/tp/mod.rs"),
        ];

        println!("\nVerification Results:");
        println!("{}", "-".repeat(50));

        let mut all_passed = true;
        for (name, path) in files {
            let full_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
            if let Ok(content) = fs::read_to_string(&full_path) {
                match verify_file(name, &content) {
                    Ok(()) => println!("✅ {} - Integrity verified", name),
                    Err(msg) => {
                        println!("error {}", msg);
                        all_passed = false;
                    }
                }
            }
        }

        println!("{}", "-".repeat(50));
        assert!(all_passed, "Some files failed integrity check!");
    }
}
