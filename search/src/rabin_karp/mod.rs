use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct HashFrame {
    hash: u64,
    first_byte: u8,
    pattern_len: usize,
}

impl HashFrame {
    const DEFAULT_PRIME: u64 = 101;
    const ALPHABET_SIZE: u64 = 256;

    pub fn new(pattern: &[u8]) -> Self {
        let prime = HashFrame::DEFAULT_PRIME;
        let base = HashFrame::ALPHABET_SIZE;

        let hash = pattern.iter()
            .fold(0, |hash, c| (hash * base + *c as u64) % prime);

        let first_byte = pattern[0];
        let pattern_len = pattern.len();

        HashFrame { hash, first_byte, pattern_len }
    }

    pub fn next(&self, pattern: &[u8]) -> Self {
        let base = HashFrame::ALPHABET_SIZE;
        let prime = HashFrame::DEFAULT_PRIME;

        let mut hash = self.hash;
        let multiplier = (1..self.pattern_len).fold(1, |mul, _| (mul * base) % prime);
        hash += prime; // Ensure non-negative hash by adding the modulus
        hash -= (multiplier * self.first_byte as u64) % prime;
        hash *= base;
        hash += pattern[pattern.len() - 1] as u64;
        hash %= prime;

        let first_byte = pattern[0];
        let pattern_len = pattern.len();

        HashFrame { hash, first_byte, pattern_len }
    }
}

impl From<&str> for HashFrame {
    fn from(pattern: &str) -> HashFrame {
        let pattern = pattern.as_bytes();

        HashFrame::new(pattern)
    }
}

impl PartialEq for HashFrame {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Hash for HashFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.hash.to_ne_bytes());
    }
}

pub fn search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() { return None; };

    let mut hash_collisions = 0;

    let needle_len = needle.len();
    let needle_hash = HashFrame::new(needle);
    let mut hay_hash = HashFrame::new(&haystack[0..needle.len()]);

    for i in 0..(haystack.len() - needle_len + 1) {
        if needle_hash == hay_hash {
            let hay = &haystack[i..(i + needle_len)];
            let mut equal = true;
            for (a, b) in hay.iter().zip(needle) {
                if a != b {
                    equal = false;
                    hash_collisions += 1;
                    break;
                }
            };

            if equal {
                log::trace!("total hash collisions = {}", hash_collisions);
                return Some(i);
            }
        }

        if i < haystack.len() - needle_len {
            let pattern = &haystack[(i + 1)..(i + needle_len + 1)];
            hay_hash = hay_hash.next(pattern);
        }
    }

    log::trace!("total hash collisions = {}", hash_collisions);

    None
}

pub mod rabin_fingerprint_hash {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let haystack = "abccddaefg".as_bytes();
        let id = search(haystack, "cdd".as_bytes());
        assert_eq!(3, id.unwrap());
    }

    #[test]
    fn test_search_multiple_patterns() {
        let haystack = "abccddaefg".as_bytes();
        let id = search(haystack, "bcc".as_bytes());
        assert_eq!(1, id.unwrap());
    }

    #[test]
    fn test_frame() {
        let frame: HashFrame = "abr".into();
        assert_eq!(4, frame.hash);
    }

    #[test]
    fn test_rolling_frame() {
        let frame: HashFrame = "abr".into();
        let next = frame.next("bra".as_bytes());
        assert_eq!(HashFrame::from("bra"), next);
        assert_eq!(30, next.hash);
    }
}
