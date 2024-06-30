use std::str::FromStr;

pub(crate) fn to_bool(s: String) -> bool {
    bool::from_str(&s).unwrap_or(false)
}

pub(crate) fn to_i32(s: &str) -> Option<i32> {
    if s.is_empty() {
        return None;
    }
    match i32::from_str_radix(s, 10) {
        Ok(i) => Some(i),
        Err(e) => {
            println!("to_i32 {:?}", e);
            None
        }
    }
}

pub(crate) fn split_to_i32(s: String) -> Vec<i32> {
    s.split(',')
        .filter_map(|s| s.parse().ok())
        .filter(|i| *i > 0)
        .collect()
}
