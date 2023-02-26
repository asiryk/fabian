#![allow(unused)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct HashFrame<'a> {
    hash: u64,
    pattern: &'a str,
}

impl<'a> HashFrame<'a> {
    const DEFAULT_PRIME: u64 = 101;
    const ALPHABET_SIZE: u64 = 256;

    pub fn new(hash: u64, pattern: &'a str) -> HashFrame<'a> {
        HashFrame { hash, pattern }
    }

    pub fn next<'b>(&self, pattern: &'b str) -> HashFrame<'b> {
        let base = HashFrame::ALPHABET_SIZE;
        let prime = HashFrame::DEFAULT_PRIME;

        let mut hash = self.hash;
        let multiplier = (1..self.pattern.len()).fold(1, |mul, _| (mul * base) % prime);
        hash += prime; // Ensure non-negative hash by adding the modulus
        hash -= (multiplier * self.pattern.chars().next().unwrap() as u64) % prime;
        hash *= base;
        hash += pattern.chars().last().unwrap() as u64;
        hash %= prime;

        HashFrame::new(hash, pattern)
    }
}

impl<'a> From<&'a str> for HashFrame<'a> {
    fn from(pattern: &'a str) -> HashFrame<'a> {
        if pattern.is_empty() { return HashFrame::new(0, pattern); }

        let prime = HashFrame::DEFAULT_PRIME;
        let base = HashFrame::ALPHABET_SIZE;

        let hash = pattern.chars()
            .fold(0, |hash, c| (hash * base + c as u64) % prime);

        HashFrame::new(hash, pattern)
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

pub fn search(haystack: &str, needles: HashSet<&str>) -> Option<usize> {
    if needles.is_empty() { return None; };

    let needle_len = needles.iter().next().unwrap().len();
    let mut hay_hash = HashFrame::from(&haystack[0..needle_len]);
    let needle_hashes = needles.iter()
        .map(|needle| HashFrame::from(*needle))
        .collect::<HashSet<HashFrame>>();

    for i in 0..(haystack.len() - needle_len + 1) {
        if needle_hashes.contains(&hay_hash) {
            for needle in needles.iter() {
                let hay = haystack[i..(i + needle_len)].chars();
                let mut equal = true;
                for (a, b) in hay.zip(needle.chars()) {
                    if a != b {
                        equal = false;
                        break;
                    }
                };

                if equal { return Some(i); }
            }
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
        let haystack = "abccddaefg";
        let id = search(haystack, ["cdd"].into());
        assert_eq!(3, id.unwrap());
    }

    #[test]
    fn test_search_multiple_patterns() {
        let haystack = "abccddaefg";
        let id = search(haystack, ["cdd", "bcc"].into());
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
        let next = frame.next("bra");
        assert_eq!(HashFrame::from("bra"), next);
        assert_eq!(30, next.hash);
    }
}
