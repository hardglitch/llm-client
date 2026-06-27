use crate::log;
use regex::Regex;
use std::io::Write;
use std::sync::LazyLock;

pub static RE_ASCII: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[│┌┬┐─]").unwrap_or_else(|e| {
        log!("RE_ASCII: {e}");
        std::process::exit(1);
    })
});

pub static RE_TEXT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[–—…]|\b(error|warning)\b").unwrap_or_else(|e| {
        log!("RE_TEXT: {e}");
        std::process::exit(1);
    })
});
