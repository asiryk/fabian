#![allow(unused)]

#[derive(Debug)]
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

pub fn search(haystack: &str, needle: &str) -> Option<usize> {
    let needle_hash = HashFrame::from(needle);
    let mut hay_hash = HashFrame::from(&haystack[0..needle.len()]);

    for i in 0..(haystack.len() - needle.len()) {
        if needle_hash == hay_hash {
            let hay = haystack[i..(i + needle.len())].chars();
            let mut equal = true;
            for (a, b) in hay.zip(needle.chars()) {
                if a != b {
                    equal = false;
                    break;
                }
            }

            if equal { return Some(i); }
        }

        let pattern = &haystack[(i + 1)..(i + needle.len() + 1)];
        hay_hash = hay_hash.next(pattern);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let haystack = "abccddaefg";
        let needle = "cdd";
        let id = search(haystack, needle);
        assert_eq!(3, id.unwrap());
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
