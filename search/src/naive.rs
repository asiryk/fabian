#![allow(unused)]

pub fn search(haystack: &str, needle: &str) -> Option<usize> {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
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
        let haystack = "abccddaefg";
        let needle = "cdd";
        let id = search(haystack, needle);
        assert_eq!(3, id.unwrap());
    }
}
