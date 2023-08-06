pub mod rabin_fingerprint;
pub mod basic_hash;

pub enum HashingMethod {
    Basic,
    RabinFingerprint,
}

pub fn search(haystack: &[u8], needle: &[u8], method: HashingMethod) -> Option<usize> {
    match method {
        HashingMethod::Basic => basic_hash::search(haystack, needle),
        HashingMethod::RabinFingerprint => rabin_fingerprint::search(haystack, needle),
    }
}
