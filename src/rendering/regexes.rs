use once_cell::sync::Lazy;
use regex::Regex;

pub static RE_ASCII: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[│┌┬┐─]").unwrap()
});

pub static RE_TEXT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[–—…]|\b(error|warning)\b").unwrap()
});
