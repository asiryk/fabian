#![allow(unused)]

pub fn search(haystack: &str, needle: &str) -> Option<usize> {
    for i in 0..(haystack.len() - needle.len()) {
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
