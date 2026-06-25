use once_cell::sync::Lazy;
use syntect::{
    easy::HighlightLines,
    highlighting::{ThemeSet},
    parsing::SyntaxSet,
    util::as_24_bit_terminal_escaped,
};

static PS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static TS: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

pub fn highlight_code(code: &str, lang: &str) -> String {
    let syntax = PS.find_syntax_by_token(lang)
        .unwrap_or_else(|| PS.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);

    let mut out = String::new();

    for line in code.lines() {
        let ranges = h.highlight_line(line, &PS).unwrap();
        out.push_str(&as_24_bit_terminal_escaped(&ranges, false));
        out.push('\n');
    }

    out
}
