use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "llm-client")]
#[command(about = "CLI agent for any OpenAI-compatible server", long_about = None)]
pub struct Args {
    /// Server host (e.g. localhost or 127.0.0.1)
    #[arg(short = 'h', long = "host", default_value = "127.0.0.1")]
    host: String,

    /// Server port
    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    port: u16,
}
impl Args {
    #[inline]
    pub fn address(&self) -> String {
        format!("http://{}:{}/v1/chat/completions", self.host, self.port)
    }
}