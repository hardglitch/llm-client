use std::io::Write;
use rendering::{make_skin, render_markdown};
use messaging::send_prompt;

mod rendering;
mod text_processing;
mod highlighting;
mod messaging;

fn main() {
	let skin = &mut make_skin();

    println!("Minimal LLaMA Client");

    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let input = input.trim();

        if input == "exit" { break; }

        match send_prompt(input) {
            Ok(output) => render_markdown(&output, skin),
            Err(e) => println!("Error: {}\n", e),
        }
    }
}
