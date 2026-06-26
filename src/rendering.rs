use termimad::{MadSkin, Alignment, StyledChar, CompoundStyle, crossterm::style::Color};
use crate::highlighting::highlight_code;
use crate::text_processing::{strip_think, normalize_unicode};
use crate::parsing::{Block, parse_blocks};
use crate::rendering::regexes::RE_TEXT;

mod table;
mod regexes;

pub fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    // ===== HEADINGS =====
    skin.headers[0].set_fg(Color::Magenta);
    skin.headers[1].set_fg(Color::Blue);
    skin.headers[2].set_fg(Color::Cyan);

    // ===== TEXT STYLES =====
    skin.bold.set_fg(Color::Rgb {r: 94, g: 227, b: 112});
    skin.italic.set_fg(Color::DarkCyan);
    skin.strikeout.set_fg(Color::DarkGrey);

    // ===== CODE =====
    skin.inline_code.set_fg(Color::Yellow);
    skin.code_block.set_fg(Color::Rgb {r: 134, g: 138, b: 145});
    skin.code_block.set_bg(Color::Black);

    // ===== QUOTES =====
    skin.quote_mark.set_fg(Color::DarkGrey);

    // ===== TABLES =====
    // skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;
    skin.table.set_fg(Color::Rgb {r: 248, g: 252, b: 222});

    // ===== LISTS =====
	skin.bullet = StyledChar::new(CompoundStyle::with_fg(Color::Rgb {r: 254, g: 164, b: 75}), '•');

    // ===== LINKS =====
    skin.inline_code.set_fg(Color::Yellow);

    skin
}

pub fn render(text: &str, skin: &mut MadSkin) {
	let text = strip_think(text);
	let text = normalize_text(&text);
    let text = normalize_unicode(&text);

    render_inner(&text, skin);
}

fn render_inner(text: &str, skin: &mut MadSkin) {
    let blocks = parse_blocks(text);

    print!("< ");

    for block in blocks {
        match block {
            Block::Code { code, lang } => {
                let hl = highlight_code(&code, &lang);
                println!("{hl}");
            }

            Block::Table(table) => {
                let table = table::normalize_table(&table);
                skin.print_text(&table);
            }

            Block::Text(txt) => {
                skin.print_text(&txt);
            }
        }
    }
}

#[inline]
fn normalize_text(text: &str) -> String {
    RE_TEXT.replace_all(text, |caps: &regex::Captures| {
        match &caps[0] {
            "–" | "—" => "-",
            "…" => "...",
            "error" => "**error**",
            "warning" => "*warning*",
            _ => unreachable!(),
        }
    })
        .into_owned()
}
