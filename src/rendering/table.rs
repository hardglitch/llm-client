use crate::rendering::regexes::RE_ASCII;

pub fn normalize_table(text: &str) -> String {
	let text = convert_ascii(text);
	convert_table_to_terminad_style(&text)
}

fn convert_ascii(text: &str) -> String {
    RE_ASCII.replace_all(text, |caps: &regex::Captures| {
        match &caps[0] {
            "│" | "┬" => "|",
            "─" => "-",
            "┌" | "┐" => "",
            _ => unreachable!(),
        }
    })
        .into_owned()
}

fn convert_table_to_terminad_style(input: &str) -> String {
    // dbg!(&input);
    let mut lines = input.lines().peekable();
    let mut out = Vec::new();

    while let Some(line) = lines.next() {
        if line.contains("```") { continue }

        if is_table_row(line) &&
		   let Some(next) = lines.peek() &&
		   is_separator_row(next)
	    {
			// consume original separator
			lines.next();

			let headers = split_row(line);
			let cols = headers.len();

			// 1. Top alignment row
			out.push(build_align_row(cols));

			// 2. Bold header row
			out.push(format!(
				"|{}|",
				headers
					.iter()
					.map(|h| format!("**{}**", h.trim()))
					.collect::<Vec<_>>()
					.join("|")
			));

			// 3. Middle separator row (normalized but varied)
			out.push(build_align_row(cols));

			// 4. Data rows
			while let Some(next) = lines.peek() {
				if is_table_row(next) {
					if let Some(l) = lines.next() {
						let row = split_row(l);
						out.push(format!("|{}|", row.join("|")));
					}
				} else {
					break;
				}
			}

			// 5. Bottom border
			out.push("|-".to_string());

			continue;
        }

        out.push(line.to_string());
    }

    // Add the Bottom border if it's missing
    if let Some(s) = out.last() && s != "|-" { out.push("|-".to_string()); };

    out.join("\n")
}

// ---------- helpers ----------

fn is_separator_row(line: &str) -> bool {
    let l = line.trim();
    l.starts_with('|') && l.contains('-')
}

fn is_table_row(line: &str) -> bool {
    line.trim().starts_with('|')
}

fn split_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|s| s.trim().to_string())
        .collect()
}

fn build_align_row(cols: usize) -> String {
    format!("|{}|", vec![":-:"; cols].join("|"))
}
