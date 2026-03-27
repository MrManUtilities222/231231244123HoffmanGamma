pub fn starts_with(s: &str, start: &str) -> bool {
    s.starts_with(start)
}

pub fn string_replace(mut s: String, src: &str, dst: &str, max_count: i32) -> String {
    if src.is_empty() {
        return s;
    }
    if max_count < 0 {
        return s.replace(src, dst);
    }

    let mut remaining = max_count;
    while remaining > 0 {
        if let Some(pos) = s.find(src) {
            s.replace_range(pos..(pos + src.len()), dst);
            remaining -= 1;
        } else {
            break;
        }
    }
    s
}

pub fn string_trim(s: &str) -> String {
    string_trim_chars(s, " \t\n\r", true, true)
}

pub fn string_trim_chars(s: &str, chars: &str, left: bool, right: bool) -> String {
    if s.is_empty() || chars.is_empty() || (!left && !right) {
        return String::new();
    }

    let bytes = s.as_bytes();
    let trim_set = chars.as_bytes();
    let mut i = 0usize;
    let mut j = bytes.len();

    if left {
        while i < bytes.len() && trim_set.contains(&bytes[i]) {
            i += 1;
        }
    }
    if right {
        while j > i && trim_set.contains(&bytes[j - 1]) {
            j -= 1;
        }
    }

    s[i..j].to_string()
}

pub fn hash_code(s: &str) -> i32 {
    let mut hash: i32 = 0;
    for b in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as i32);
    }
    hash
}

pub fn remove_all(mut s: String, reps: &[&str]) -> String {
    for rep in reps {
        s = string_replace(s, rep, "", -1);
    }
    s
}
