use flexstr::ToLocalStr;
use flexstr::local_str;

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
            len < 200
        })
        .fold(String::with_capacity(205), |mut acc, c| {
            acc.push_str(c.as_str());
            acc
        });

    if s.len() > 200 {
        modified_s + "..."
    } else {
        modified_s
    }
}
