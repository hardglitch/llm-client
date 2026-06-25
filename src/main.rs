use std::io::Write;
use serde::{Deserialize, Serialize};
use termimad::print_text;
use unicode_normalization::UnicodeNormalization;

const SERVER_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageOwned,
}

#[derive(Deserialize)]
struct MessageOwned {
    content: String,
}

fn send_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let req = ChatRequest {
        model: "any",
        messages: vec![Message {
            role: "user",
            content: prompt,
        }],
    };

    let mut response = ureq::post(SERVER_URL)
        .header("Content-Type", "application/json")
        .send(serde_json::to_vec(&req)?)?;

    let body = response.body_mut().read_to_vec()?;
    let parsed: ChatResponse = serde_json::from_slice(&body)?;

    Ok(parsed
        .choices
        .first()
        .ok_or("No choices in response")?
        .message
        .content
        .clone())
}

fn strip_think(text: &str) -> String {
    let mut result = String::new();
    let mut inside = false;

    for line in text.lines() {
        if line.contains("<think>") {
            inside = true;
            continue;
        }
        if line.contains("</think>") {
            inside = false;
            continue;
        }
        if !inside {
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim().to_string()
}

fn normalize_unicode(s: &str) -> String {
    s.nfkc().collect()
}

fn main() {
    println!("Minimal LLaMA Client (ureq 3.x)\n");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        match send_prompt(input) {
            Ok(output) => {
                let clean = strip_think(&output);
				let s = normalize_unicode(&clean);
                println!();
                print_text(&s);
                println!();
            }
            Err(e) => println!("Error: {}\n", e),
        }
    }
}
