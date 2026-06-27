use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "llm-client")]
#[command(about = "A minimalistic CLI agent for any OpenAI-compatible server", long_about = None)]
pub struct Args {
    /// Server port
    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    port: u16,
}
impl Args {
    #[inline]
    pub fn address(&self) -> String {
        format!("http://127.0.0.1:{}/v1/chat/completions", self.port)
    }
}