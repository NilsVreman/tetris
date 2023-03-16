pub fn u16_to_string(u: u16) -> String {
    (0..16).rev()
        .map(|i| if u & (1 << i) != 0 { "1" } else { "0" })
        .collect()
}
