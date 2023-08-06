#[derive(Debug, PartialEq, Eq)]
struct Hash(u32);

impl Hash {
    /// Create a new hash from byte array slice.
    pub fn new(bytes: &[u8]) -> Self {
        let mut hash = Hash(0);
        for &b in bytes {
            hash.add(b);
        }

        hash
    }

    /// Create a new hash, which is equal to empty string.
    pub fn new_empty() -> Self {
        Hash(0)
    }

    /// Delete byte from the "beginning" of the rolling hash and add byte to the "end".
    pub fn roll(&mut self, old: u8, new: u8, factor: u32) {
        self.del(old, factor);
        self.add(new);
    }

    /// Add byte to the "end" of rolling hash.
    /// It bitwise shifts current hash (which is equal to multiplication by 2) and adds new byte.
    pub fn add(&mut self, byte: u8) {
        let w = self.0.wrapping_shl(1);
        self.0 = w.wrapping_add(byte as u32);
    }

    /// Delete byte from the "beginning" of the rolling hash.
    /// Since it is all power of 2, first we need to multiply this byte by factor (2^len),
    /// and only then subtract from rolling hash.
    pub fn del(&mut self, byte: u8, factor: u32) {
        let w = (byte as u32).wrapping_mul(factor);
        self.0 = self.0.wrapping_sub(w);
    }

    /// Get struct hash value.
    pub fn get_val(&self) -> u32 {
        self.0
    }
}

struct NeedleHash {
    hash: Hash,
    len_pow_2: u32,
}

impl NeedleHash {
    pub fn new(needle: &[u8]) -> Self {
        let mut nh = NeedleHash { hash: Hash::new_empty(), len_pow_2: 1 };
        if needle.is_empty() {
            return nh;
        }
        nh.hash.add(needle[0]);
        for &b in needle.iter().skip(1) {
            nh.hash.add(b);
            nh.len_pow_2 = nh.len_pow_2.wrapping_shl(1);
        }

        nh
    }

    pub fn get_val(&self) -> &Hash {
        &self.hash
    }

    pub fn get_len_pow_2(&self) -> u32 {
        self.len_pow_2
    }
}

pub fn search(mut haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() { return None; };
    if haystack.len() < needle.len() { return None; }

    let start = haystack.as_ptr() as usize;
    let mut hash = Hash::new(&haystack[..needle.len()]);
    let nhash = NeedleHash::new(needle);
    loop {
        if nhash.get_val().eq(&hash) && byte_by_byte_eq(haystack, needle) {
            return Some(haystack.as_ptr() as usize - start);
        }
        if needle.len() >= haystack.len() {
            return None;
        }
        hash.roll(haystack[0], haystack[needle.len()], nhash.get_len_pow_2());
        haystack = &haystack[1..];
    }
}

pub fn byte_by_byte_eq(haystack: &[u8], needle: &[u8]) -> bool {
    needle.iter().zip(haystack).all(|(a, b)| a.eq(b))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_del_idempotent() {
        let mut hash = Hash::new_empty();

        hash.add(5);
        hash.add(3);
        hash.add(7);
        hash.add(13);
        hash.del(5, 8);
        hash.del(3, 4);
        hash.del(7, 2);
        hash.del(13, 1);

        assert_eq!(Hash::new_empty(), hash);
    }

    #[test]
    fn roll_and_new_equal() {
        let hash1 = Hash::new("hello".as_bytes());
        let mut hash2 = Hash::new("xhell".as_bytes());

        // remove x with factor of 16
        // x  | h | e | l | l
        // 16 | 8 | 4 | 2 | 1
        hash2.roll(b'x', b'o', 16);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_search() {
        let haystack = "abccddaefg".as_bytes();
        let id = search(haystack, "cdd".as_bytes());

        assert_eq!(3, id.unwrap());
    }

    #[test]
    fn test_search_not_found() {
        let haystack = "abccddaefg".as_bytes();
        let id = search(haystack, "aaa".as_bytes());

        assert_eq!(None, id);
    }
}
