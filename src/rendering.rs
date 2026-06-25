use termimad::{MadSkin, Alignment, StyledChar, CompoundStyle, crossterm::style::Color};
use crate::highlighting::highlight_code;
use crate::text_processing::{strip_think, normalize_unicode};


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
    skin.table.align = Alignment::Center;
    skin.table.set_fg(Color::White);

    // ===== LISTS =====
	skin.bullet = StyledChar::new(CompoundStyle::with_fg(Color::Green), '•');

    // ===== LINKS =====
    skin.inline_code.set_fg(Color::Yellow);

    skin
}

pub fn render_markdown(text: &str, skin: &mut MadSkin) {
	let cleaned = strip_think(text);
	let colorized = colorize(&cleaned);
	
	println!();
	render(&colorized, skin);
	println!();
}

fn colorize(text: &str) -> String {
    text
        .replace("error", "**error**")
        .replace("warning", "*warning*")
}

fn render(text: &str, skin: &mut MadSkin) {
    let mut in_code = false;
    let mut lang = "";
    let mut buffer = String::new();

    for line in text.lines() {
        if line.starts_with("```") {
            if in_code {
                // render highlighted code
                println!("{}", highlight_code(&buffer, lang));
                buffer.clear();
                in_code = false;
            } else {
                lang = line.trim_start_matches("```");
                in_code = true;
            }
            continue;
        }

        if in_code {
            buffer.push_str(line);
            buffer.push('\n');
        } else {
			let normalized = normalize_unicode(line);
            skin.print_text(&normalized);
        }
    }
}
