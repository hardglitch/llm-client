use unicode_normalization::UnicodeNormalization;

pub fn strip_think(text: &str) -> String {
    let mut result = String::new();
    let mut inside = false;

    for line in text.lines() {
        if line.contains("<think>") {
            inside = true;
            continue;
        }
        if line.contains("</think>") {
            inside = false;
            continue;
        }
        if !inside {
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim().to_string()
}

#[inline]
pub fn normalize_unicode(s: &str) -> String {
    s.nfkc().collect()
}
