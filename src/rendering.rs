use termimad::{MadSkin, Alignment, StyledChar, CompoundStyle, crossterm::style::Color};
use crate::highlighting::highlight_code;
use crate::text_processing::{strip_think, normalize_unicode};
use crate::parsing::{Block, parse_blocks};

mod table;

pub fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    // ===== HEADINGS =====
    skin.headers[0].set_fg(Color::Magenta);
    skin.headers[1].set_fg(Color::Blue);
    skin.headers[2].set_fg(Color::Cyan);

    // ===== TEXT STYLES =====
    skin.bold.set_fg(Color::Green);
    skin.italic.set_fg(Color::DarkCyan);
    skin.strikeout.set_fg(Color::DarkGrey);

    // ===== CODE =====
    skin.inline_code.set_fg(Color::Yellow);
    skin.code_block.set_fg(Color::White);
    skin.code_block.set_bg(Color::Black);

    // ===== QUOTES =====
    skin.quote_mark.set_fg(Color::DarkGrey);

    // ===== TABLES =====
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;
    skin.table.set_fg(Color::parse_ansi("2;248;252;222").unwrap()); // rgb

    // ===== LISTS =====
	skin.bullet = StyledChar::new(CompoundStyle::with_fg(Color::Green), '•');

    // ===== LINKS =====
    skin.inline_code.set_fg(Color::Yellow);

    skin
}

pub fn render_markdown(text: &str, skin: &mut MadSkin) {
	let text = strip_think(text);
	let text = normalize_text(&text);
	
	println!();
	render(&text, skin);
	println!();
}

fn normalize_text(text: &str) -> String {
    text
        .replace("–", "-")
        .replace("—", "-")
        .replace("…", "...")
        .replace("error", "**error**")
        .replace("warning", "*warning*")
}

pub fn render(text: &str, skin: &mut MadSkin) {
    let blocks = parse_blocks(text);

    println!();

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
                let normalized = normalize_unicode(&txt);
                skin.print_text(&normalized);
            }
        }
    }

    println!();
}
