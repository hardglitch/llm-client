use std::io::Write;
use clap::Parser;
use rendering::{make_skin, render};
use messaging::send_prompt;
use crate::commands::Args;

mod rendering;
mod text_processing;
mod highlighting;
mod messaging;
mod parsing;
mod commands;

fn main() {
    let args = Args::parse();
    let address = args.address();
	let skin = &mut make_skin();

    println!("llm-client");
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") { break; }

        match send_prompt(input, &address) {
            Ok(output) => render(&output, skin),
            Err(e) => println!("Error: {}\n", e),
        }
    }
}
