use flexstr::ToLocalStr;
use flexstr::local_str;

const MAX_CONTEXT_LEN: usize = 20;

pub fn legible_string(s: &str) -> String {

    let mut len = 0;

    let modified_s = s
        .chars()
        .map(|c| match c {
            '\n' => local_str!(","),
            '\r' => local_str!(""),
            x => x.to_local_str(),
        })
        .take_while(|s| {
            len += s.len();
            len < MAX_CONTEXT_LEN
        })
        .fold(String::with_capacity(MAX_CONTEXT_LEN), |mut acc, c| {
            acc.push_str(c.as_str());
            acc
        });

    if s.len() > MAX_CONTEXT_LEN {
        modified_s + "..."
    } else {
        modified_s
    }
}
