pub mod rabin_fingerprint;
pub mod basic_hash;

pub fn search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    rabin_fingerprint::search(haystack, needle)
}
