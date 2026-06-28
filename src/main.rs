use std::io::Write;
use clap::Parser;
use tiktoken_rs::cl100k_base;
use rendering::{make_skin, render};
use messaging::prompt;
use crate::commands::Args;
use crate::logging::Log;
use crate::messaging::Props;

mod rendering;
mod text_processing;
mod highlighting;
mod messaging;
mod parsing;
mod commands;
mod logging;

fn main() {
    let args = Args::parse();
    Log::init(&args.log_file, args.log_size);

	let skin = &mut make_skin();
    let host = format!("http://127.0.0.1:{}", args.port);
    let mut props = if args.show_stat { Some(&mut Props::init(&host)) } else { None };
    let tokenizer = if args.show_stat {
        match cl100k_base() {
            Ok(t) => { Some(t) }
            Err(e) => { log!("{e}"); return; }
        }
    } else { None };

    println!("llm-client");
    loop {
        print!("> ");
        if let Err(e) = std::io::stdout().flush() { log!("{e}"); }

        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) { log!("{e}"); }
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") { break; }

        match prompt(input, &args, &mut props, &tokenizer) {
            Ok(output) => render(&output, skin),
            Err(e) => {
                println!("Error: {e}\n");
                log!("Prompt error: {e}\n")
            }
        }
    }
}
