use regex::Regex;
use std::sync::LazyLock;

pub static RE_ASCII: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[│┌┬┐─]").unwrap()
});

pub static RE_TEXT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[–—…]|\b(error|warning)\b").unwrap()
});
