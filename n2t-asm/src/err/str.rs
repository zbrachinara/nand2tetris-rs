const MAX_CONTEXT_LEN: usize = 20;

pub fn legible_string(s: &str) -> String {
    let modified_s = s
        .chars()
        .filter_map(|c| match c {
            '\n' => Some(','),
            '\r' => None,
            x => Some(x),
        })
        .take(MAX_CONTEXT_LEN)
        .collect::<String>();

    if s.len() > MAX_CONTEXT_LEN {
        modified_s + "..."
    } else {
        modified_s
    }
}
