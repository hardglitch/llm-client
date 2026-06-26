pub enum Block {
    Code { code: String, lang: String, },
    Table(String),
    Text(String),
}

pub fn parse_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut lines = input.lines().peekable();
    let mut buffer = String::new();
    let mut is_table = false;

    while let Some(line) = lines.next() {

        // ---------- TABLE (Markdown or box) ----------
        if line.starts_with("```") && lines.peek().is_some_and(|next| is_table_line(next)) ||
           is_table_line(line)
        {
            buffer.clear();
            buffer.push_str(line);
            buffer.push('\n');
            is_table = true;

            while let Some(next) = lines.peek() {
                if is_table_line(next) {
                    buffer.push_str(lines.next().unwrap());
                    buffer.push('\n');
                } else {
                    break;
                }
            }

            blocks.push(Block::Table(buffer.clone()));
            continue;
        }

        // ---------- CODE BLOCK ----------
        if line.starts_with("```") && !is_table {
            let lang = line.trim_start_matches("```").to_string();
            let mut code = String::new();

            while let Some(l) = lines.by_ref().next() {
                if l.starts_with("```") {
                    break;
                }
                code.push_str(l);
                code.push('\n');
            }

            blocks.push(Block::Code { code, lang });
            continue;
        }

        // ---------- TEXT ----------
        if !line.starts_with("```") {
            buffer.clear();
            buffer.push_str(line);
            buffer.push('\n');

            while let Some(next) = lines.peek() {
                if !is_table_line(next) && !next.starts_with("```") {
                    buffer.push_str(lines.next().unwrap());
                    buffer.push('\n');
                } else {
                    break;
                }
            }

            blocks.push(Block::Text(buffer.clone()));
        }

        is_table = false;
    }

    blocks
}

fn is_table_line(line: &str) -> bool {
    let l = line.trim();

    // Markdown-style
    if l.starts_with('|') && l.contains('|') {
        return true;
    }

    // Separator row
    if l.contains('-') && l.contains('|') {
        return true;
    }

    // Box-drawing
    if l.contains('│') || l.contains('┌') || l.contains('┬') || l.contains('─') {
        return true;
    }

    false
}
