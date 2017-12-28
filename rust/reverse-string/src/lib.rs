pub fn reverse(s: &str) -> String {
    let string = s.to_string();
    let mut chars = string.chars();
    chars.into_iter().rev().collect()
}
