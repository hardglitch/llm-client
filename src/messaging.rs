use crate::rendering::{stat, Stat};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use anyhow::anyhow;
use serde_json::Value;
use tiktoken_rs::{cl100k_base, CoreBPE};
use crate::commands::Args;

const SERVER_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    stream: bool,
}
#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}
#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}
#[derive(Deserialize)]
struct StreamChoice {
    delta: Delta,
}
#[derive(Deserialize)]
struct Delta {
    content: Option<String>,
}
#[derive(Deserialize, Default)]
pub struct Props {
    total_used_tokens: usize,
}
impl Props {
    fn context_size(address: &str) -> Option<u64> {
        if let Ok(ctx) = Self::try_llama(address) { Some(ctx) }
        else if let Ok(ctx) = Self::try_tgi(address) { Some(ctx) }
        else { Self::try_webui(address).ok() }
    }
    fn try_llama(address: &str) -> Result<u64, anyhow::Error> {
        let data = ureq::get(&format!("{}/props", address))
            .call()?
            .body_mut()
            .read_to_vec()?;
        let props: Value = serde_json::from_slice(&data)?;
        let ctx_size =
            if let Some(val) = props.get("n_ctx") &&
               let Some(ctx_size_) = val.as_u64()
            {
                ctx_size_
            }
            else if let Some(val) = props.get("default_generation_settings") &&
                let Some(val) = val.get("n_ctx") &&
                let Some(ctx_size_) = val.as_u64()
            {
                ctx_size_
            }
            else { return Err(anyhow!("Context size not found")) };
        Ok(ctx_size)
    }
    fn try_webui(address: &str) -> Result<u64, anyhow::Error> {
        let data = ureq::get(&format!("{}/v1/internal/model/info", address))
            .call()?
            .body_mut()
            .read_to_vec()?;
        let props: Value = serde_json::from_slice(&data)?;
        let ctx_size =
            if let Some(val) = props.get("max_seq_len") &&
               let Some(ctx_size) = val.as_u64()
            {
                ctx_size
            }
            else { return Err(anyhow!("Context size not found")) };
        Ok(ctx_size)
    }
    fn try_tgi(address: &str) -> Result<u64, anyhow::Error> {
        let data = ureq::get(&format!("{}/info", address))
            .call()?
            .body_mut()
            .read_to_vec()?;
        let props: Value = serde_json::from_slice(&data)?;
        let ctx_size =
            if let Some(val) = props.get("max_total_tokens") &&
                let Some(ctx_size) = val.as_u64()
            {
                ctx_size
            }
            else { return Err(anyhow!("Context size not found")) };
        Ok(ctx_size)
    }
}

pub fn prompt(prompt: &str, args: &Args, props: &mut Props) -> Result<String, anyhow::Error> {
    let mut result = String::new();
    let mut current_tokens = 0;
    let tokenizer = cl100k_base()?;
    let host = format!("http://127.0.0.1:{}", args.port);
    let address = format!("{host}/v1/chat/completions");
    let ctx_size = Props::context_size(&host).unwrap_or_default() as usize;

    let req = ChatRequest {
        model: "any",
        messages: vec![Message {
            role: "user",
            content: prompt,
        }],
        stream: true,
    };
    let value = serde_json::to_vec(&req)?;
    let address = if address.is_empty() { SERVER_URL } else { &address };
    let mut response = ureq::post(address)
        .header("Content-Type", "application/json")
        .send(value)?;

    let reader = BufReader::new(response.body_mut().as_reader());

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("data: ") { continue; }

        let data = &line[6..];
        if data == "[DONE]" { break; }
        let chunk: StreamChunk = serde_json::from_str(data)?;
        if let Some(content_) = &chunk.choices.first() &&
           let Some(content) = &content_.delta.content
        {
            // stream output
            // print!("{}", content);
            // std::io::stdout().flush()?;

            // collect result
            result.push_str(content);

            // update stat
            let tokens = count_tokens(&tokenizer, content);
            current_tokens += tokens;
            props.total_used_tokens += tokens;

            let stat_info = Stat::new(current_tokens, props.total_used_tokens, ctx_size);
            stat(stat_info);
        }
    }

    println!();
    Ok(result)
}

fn count_tokens(bpe: &CoreBPE, text: &str) -> usize {
    bpe.encode(text, &HashSet::new())
        .map(|(_, n)| n)
        .unwrap_or(0)
}