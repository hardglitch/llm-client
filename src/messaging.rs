use serde::{Deserialize, Serialize};

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

pub fn send_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let req = ChatRequest {
        model: "any",
        messages: vec![Message {
            role: "user",
            content: prompt,
        }],
    };

	let value = serde_json::to_vec(&req)?;
    let mut response = ureq::post(SERVER_URL)
        .header("Content-Type", "application/json")
        .send(value)?;

    let body = response.body_mut().read_to_vec()?;
    let parsed: ChatResponse = serde_json::from_slice(&body)?;

    let res = parsed
        .choices
        .first()
        .ok_or("No choices in response")?
        .message
        .content
        .clone();

	Ok(res)
}
