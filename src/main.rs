use std::io::Write;
use clap::Parser;
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
    let mut props = Props::default();

    println!("llm-client");
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") { break; }

        match prompt(input, &args, &mut props) {
            Ok(output) => render(&output, skin),
            Err(e) => {
                println!("Error: {e}\n");
                log!("Error: {e}\n")
            }
        }
    }
}
