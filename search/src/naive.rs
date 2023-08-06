pub fn search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    for i in 0..(haystack.len() - needle.len() + 1) {
        let hay = haystack[i..(i + needle.len())].iter();
        let mut equal = true;
        for (a, b) in hay.zip(needle) {
            if a != b {
                equal = false;
                break;
            }
        }

        if equal { return Some(i); }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let haystack = "abccddaefg".as_bytes();
        let needle = "cdd".as_bytes();
        let id = search(haystack, needle);
        assert_eq!(3, id.unwrap());
    }
}
