#![allow(unused)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct HashFrame<'a> {
    hash: u64,
    pattern: &'a [u8],
}

impl<'a> HashFrame<'a> {
    const DEFAULT_PRIME: u64 = 101;
    const ALPHABET_SIZE: u64 = 256;

    pub fn new(pattern: &'a [u8]) -> HashFrame<'a> {
        let prime = HashFrame::DEFAULT_PRIME;
        let base = HashFrame::ALPHABET_SIZE;

        let hash = pattern.iter()
            .fold(0, |hash, c| (hash * base + *c as u64) % prime);

        HashFrame { hash, pattern }
    }

    pub fn next<'b>(&self, pattern: &'b [u8]) -> HashFrame<'b> {
        let base = HashFrame::ALPHABET_SIZE;
        let prime = HashFrame::DEFAULT_PRIME;

        let mut hash = self.hash;
        let multiplier = (1..self.pattern.len()).fold(1, |mul, _| (mul * base) % prime);
        hash += prime; // Ensure non-negative hash by adding the modulus
        hash -= (multiplier * self.pattern[0] as u64) % prime;
        hash *= base;
        hash += pattern[pattern.len() - 1] as u64;
        hash %= prime;

        HashFrame { hash, pattern }
    }
}

impl<'a> From<&'a str> for HashFrame<'a> {
    fn from(pattern: &'a str) -> HashFrame<'a> {
        let pattern = pattern.as_bytes();

        HashFrame::new(pattern)
    }
}

impl PartialEq for HashFrame<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Hash for HashFrame<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.hash.to_ne_bytes());
    }
}

pub fn search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() { return None; };

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
                    break;
                }
            };

            if equal { return Some(i); }
        }

        if i < haystack.len() - needle_len {
            let pattern = &haystack[(i + 1)..(i + needle_len + 1)];
            hay_hash = hay_hash.next(pattern);
        }
    }

    None
}

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
